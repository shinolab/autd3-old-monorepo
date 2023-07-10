/*
 * File: controller.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    marker::PhantomData,
    sync::atomic::{self, AtomicU8},
    time::Duration,
};

use autd3_core::{
    clear::Clear, datagram::Datagram, float, geometry::*, link::Link, stop::Stop,
    synchronize::Synchronize, FPGAInfo, FirmwareInfo, Operation, RxDatagram, TxDatagram, METER,
    MSG_BEGIN, MSG_END,
};

use crate::{error::AUTDError, link::NullLink, software_stm::SoftwareSTM};

pub struct ControllerBuilder<T: Transducer> {
    attenuation: float,
    sound_speed: float,
    transducers: Vec<(usize, Vector3, UnitQuaternion)>,
    device_map: Vec<usize>,
    phantom: PhantomData<T>,
}

impl<T: Transducer> Default for ControllerBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transducer> ControllerBuilder<T> {
    pub fn new() -> ControllerBuilder<T> {
        Self {
            attenuation: 0.0,
            sound_speed: 340.0 * METER,
            transducers: vec![],
            device_map: vec![],
            phantom: PhantomData,
        }
    }

    pub fn attenuation(self, attenuation: float) -> Self {
        Self {
            attenuation,
            ..self
        }
    }

    pub fn sound_speed(self, sound_speed: float) -> Self {
        Self {
            sound_speed,
            ..self
        }
    }

    pub fn add_device<D: Device>(mut self, dev: D) -> Self {
        let id = self.transducers.len();
        let mut t = dev.get_transducers(id);
        self.device_map.push(t.len());
        self.transducers.append(&mut t);
        self
    }

    pub fn open_with<L: Link<T>>(self, link: L) -> Result<Controller<T, L>, AUTDError> {
        Controller::open_impl(
            Geometry::<T>::new(
                self.transducers
                    .iter()
                    .map(|&(id, pos, rot)| T::new(id, pos, rot))
                    .collect(),
                self.device_map.clone(),
                self.sound_speed,
                self.attenuation,
            )?,
            link,
        )
    }
}

impl ControllerBuilder<LegacyTransducer> {
    pub fn advanced(self) -> ControllerBuilder<AdvancedTransducer> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn advanced_phase(self) -> ControllerBuilder<AdvancedPhaseTransducer> {
        unsafe { std::mem::transmute(self) }
    }
}

impl ControllerBuilder<AdvancedTransducer> {
    pub fn legacy(self) -> ControllerBuilder<LegacyTransducer> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn advanced_phase(self) -> ControllerBuilder<AdvancedPhaseTransducer> {
        unsafe { std::mem::transmute(self) }
    }
}

impl ControllerBuilder<AdvancedPhaseTransducer> {
    pub fn advanced(self) -> ControllerBuilder<AdvancedTransducer> {
        unsafe { std::mem::transmute(self) }
    }

    pub fn legacy(self) -> ControllerBuilder<LegacyTransducer> {
        unsafe { std::mem::transmute(self) }
    }
}

pub struct Controller<T: Transducer, L: Link<T>> {
    link: L,
    geometry: Geometry<T>,
    tx_buf: TxDatagram,
    rx_buf: RxDatagram,
    force_fan: autd3_core::ForceFan,
    reads_fpga_info: autd3_core::ReadsFPGAInfo,
    msg_id: AtomicU8,
}

impl Controller<LegacyTransducer, NullLink> {
    pub fn builder() -> ControllerBuilder<LegacyTransducer> {
        ControllerBuilder::<LegacyTransducer>::new()
    }
}

impl<T: Transducer, L: Link<T>> Controller<T, L> {
    pub(crate) fn open_impl(geometry: Geometry<T>, link: L) -> Result<Controller<T, L>, AUTDError> {
        let mut link = link;
        link.open(&geometry)?;
        let num_devices = geometry.num_devices();
        let tx_buf = TxDatagram::new(geometry.device_map());
        let mut cnt = Controller {
            link,
            geometry,
            tx_buf,
            rx_buf: RxDatagram::new(num_devices),
            force_fan: autd3_core::ForceFan::default(),
            reads_fpga_info: autd3_core::ReadsFPGAInfo::default(),
            msg_id: AtomicU8::new(MSG_BEGIN),
        };
        cnt.send(Clear::new())?;
        cnt.send(Synchronize::new())?;
        Ok(cnt)
    }

    pub fn force_fan(&mut self, value: bool) {
        self.force_fan.value = value
    }

    pub fn reads_fpga_info(&mut self, value: bool) {
        self.reads_fpga_info.value = value
    }

    pub fn geometry(&self) -> &Geometry<T> {
        &self.geometry
    }

    pub fn geometry_mut(&mut self) -> &mut Geometry<T> {
        &mut self.geometry
    }

    pub fn link(&self) -> &L {
        &self.link
    }

    pub fn link_mut(&self) -> &L {
        &self.link
    }

    /// Send header and body to the devices
    ///
    /// # Arguments
    ///
    /// * `header` - Header
    /// * `body` - Body
    ///
    pub fn send<S: Datagram<T>>(&mut self, s: S) -> Result<bool, AUTDError> {
        let (mut op_header, mut op_body) = s.operation(&self.geometry)?;

        op_header.init();
        op_body.init();

        self.force_fan.pack(&mut self.tx_buf);
        self.reads_fpga_info.pack(&mut self.tx_buf);

        let timeout = s.timeout().unwrap_or(self.link.timeout());
        loop {
            self.tx_buf.header_mut().msg_id = self.get_id();

            op_header.pack(&mut self.tx_buf)?;
            op_body.pack(&mut self.tx_buf)?;

            if !self
                .link
                .send_receive(&self.tx_buf, &mut self.rx_buf, timeout)?
            {
                return Ok(false);
            }
            if op_header.is_finished() && op_body.is_finished() {
                break;
            }
            if timeout.is_zero() {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        Ok(true)
    }

    pub async fn send_async<S: Datagram<T>>(&mut self, s: S) -> Result<bool, AUTDError> {
        async { self.send(s) }.await
    }

    pub fn close(&mut self) -> Result<bool, AUTDError> {
        if !self.link.is_open() {
            return Ok(false);
        }
        let res = self.send(Stop::new())?;
        let res = res & self.send(Clear::new())?;
        self.link.close()?;
        Ok(res)
    }

    /// Return firmware information of the devices
    pub fn firmware_infos(&mut self) -> Result<Vec<FirmwareInfo>, AUTDError> {
        let mut op = autd3_core::CPUVersionMajor::default();
        op.pack(&mut self.tx_buf);
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let cpu_versions = self.rx_buf.iter().map(|rx| rx.ack).collect::<Vec<_>>();

        let mut op = autd3_core::FPGAVersionMajor::default();
        op.pack(&mut self.tx_buf);
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let fpga_versions = self.rx_buf.iter().map(|rx| rx.ack).collect::<Vec<_>>();

        let mut op = autd3_core::FPGAFunctions::default();
        op.pack(&mut self.tx_buf);
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let fpga_functions = self.rx_buf.iter().map(|rx| rx.ack).collect::<Vec<_>>();

        let mut op = autd3_core::FPGAVersionMinor::default();
        op.pack(&mut self.tx_buf);
        let fpga_versions_minor =
            match self
                .link
                .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))
            {
                Ok(_) => self.rx_buf.iter().map(|rx| rx.ack).collect::<Vec<_>>(),
                _ => vec![0x00; self.geometry.num_devices()],
            };

        let mut op = autd3_core::CPUVersionMinor::default();
        op.pack(&mut self.tx_buf);
        let cpu_versions_minor =
            match self
                .link
                .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))
            {
                Ok(_) => self.rx_buf.iter().map(|rx| rx.ack).collect::<Vec<_>>(),
                _ => vec![0x00; self.geometry.num_devices()],
            };

        Ok((0..self.geometry.num_devices())
            .map(|i| {
                FirmwareInfo::new(
                    i,
                    cpu_versions[i],
                    cpu_versions_minor[i],
                    fpga_versions[i],
                    fpga_versions_minor[i],
                    fpga_functions[i],
                )
            })
            .collect())
    }

    pub fn fpga_info(&mut self) -> Result<Vec<FPGAInfo>, AUTDError> {
        self.link.receive(&mut self.rx_buf)?;
        Ok(self.rx_buf.iter().map(FPGAInfo::from).collect())
    }

    pub fn software_stm<
        S: Datagram<T>,
        Fs: FnMut(usize, std::time::Duration) -> S + Send + 'static,
        Ff: FnMut(usize, std::time::Duration) -> bool + Send + 'static,
    >(
        &mut self,
        callback: Fs,
        finish_handler: Ff,
    ) -> SoftwareSTM<T, L, S, Fs, Ff> {
        SoftwareSTM::new(self, callback, finish_handler)
    }
}

impl<T: Transducer, L: Link<T>> Controller<T, L> {
    pub fn get_id(&mut self) -> u8 {
        if self
            .msg_id
            .compare_exchange(
                MSG_END,
                MSG_BEGIN,
                atomic::Ordering::SeqCst,
                atomic::Ordering::SeqCst,
            )
            .is_err()
        {
            self.msg_id.fetch_add(1, atomic::Ordering::SeqCst);
        }
        self.msg_id.load(atomic::Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {

    use autd3_core::{
        acoustics::Complex,
        amplitude::Amplitudes,
        autd3_device::{AUTD3, DEVICE_WIDTH},
        gain::Gain,
        geometry::Vector3,
        modulation::Modulation,
        silencer_config::SilencerConfig,
        stm::{FocusSTM, GainSTM},
        synchronize::Synchronize,
        FPGAControlFlags, Mode, PI,
    };

    use spdlog::LevelFilter;

    use crate::{
        link::Debug,
        prelude::{Focus, Sine},
    };

    use super::*;

    #[test]
    fn basic_usage() {
        let mut autd = Controller::builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        let firm_infos = autd.firmware_infos().unwrap();
        assert_eq!(firm_infos.len(), autd.geometry().num_devices());
        firm_infos.iter().for_each(|f| {
            assert_eq!(f.cpu_version(), "v2.9.0");
            assert_eq!(f.fpga_version(), "v2.9.0");
        });

        assert!(autd.link().emulators().iter().all(|cpu| {
            cpu.fpga()
                .duties_and_phases(0)
                .iter()
                .all(|&(d, p)| d == 0x0000 && p == 0x0000)
        }));
        assert!(autd.link().emulators().iter().all(|cpu| cpu
            .fpga()
            .cycles()
            .iter()
            .all(|&c| c == 0x1000)));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| cpu.fpga().modulation_cycle() == 2
                && cpu.fpga().modulation_frequency_division() == 40960
                && cpu.fpga().modulation().iter().all(|&m| m == 0x00)));

        let silencer = SilencerConfig::default();
        autd.send(silencer).unwrap();

        let f = autd.geometry().center() + Vector3::new(0.0, 0.0, 150.0);
        let m = Sine::new(150);
        let g = Focus::new(f);

        autd.send((m, g)).unwrap();

        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { cpu.fpga_flags().contains(FPGAControlFlags::LEGACY_MODE) }));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga_flags().contains(FPGAControlFlags::STM_MODE) }));

        let base_tr = &autd.geometry()[0];
        let expect = (autd3_core::acoustics::propagate::<autd3_core::acoustics::Sphere>(
            base_tr.position(),
            &base_tr.z_direction(),
            0.,
            base_tr.wavenumber(autd.geometry().sound_speed),
            &f,
        ) * Complex::new(
            0.,
            2. * PI * autd.link().emulators()[0].fpga().duties_and_phases(0)[0].1 as float
                / autd.link().emulators()[0].fpga().cycles()[0] as float,
        )
        .exp())
        .arg();

        autd.geometry()
            .iter()
            .zip(
                autd.link()
                    .emulators()
                    .iter()
                    .flat_map(|cpu| cpu.fpga().duties_and_phases(0)),
            )
            .zip(
                autd.link()
                    .emulators()
                    .iter()
                    .flat_map(|cpu| cpu.fpga().cycles()),
            )
            .for_each(|((tr, (d, p)), c)| {
                let p = (autd3_core::acoustics::propagate::<autd3_core::acoustics::Sphere>(
                    tr.position(),
                    &tr.z_direction(),
                    0.,
                    tr.wavenumber(autd.geometry().sound_speed),
                    &f,
                ) * Complex::new(0., 2. * PI * p as float / c as float).exp())
                .arg();
                assert_eq!(d, c >> 1);
                assert_approx_eq::assert_approx_eq!(p, expect, 2. * PI / 256.);
            });

        let expect_mod = {
            m.calc()
                .unwrap()
                .iter()
                .map(|d| (d.clamp(0., 1.).asin() * 2.0 / PI * 255.0) as u8)
                .collect::<Vec<_>>()
        };
        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().modulation().len(), expect_mod.len());
            cpu.fpga()
                .modulation()
                .iter()
                .zip(expect_mod.iter())
                .for_each(|(a, b)| assert_eq!(a, b));
        });

        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().silencer_step(), 10);
        });

        autd.send(Stop::new()).unwrap();
        autd.link().emulators().iter().for_each(|cpu| {
            cpu.fpga().duties_and_phases(0).iter().for_each(|&(d, _)| {
                assert_eq!(d, 0x0000);
            })
        });

        autd.send(Clear::new()).unwrap();
        autd.link().emulators().iter().for_each(|cpu| {
            cpu.fpga().duties_and_phases(0).iter().for_each(|&(d, p)| {
                assert_eq!(d, 0x0000);
                assert_eq!(p, 0x0000);
            });
            cpu.fpga().cycles().iter().for_each(|&c| {
                assert_eq!(c, 0x1000);
            });
            assert_eq!(cpu.fpga().modulation_cycle(), 2);
            assert_eq!(cpu.fpga().modulation_frequency_division(), 40960);
            cpu.fpga().modulation().iter().for_each(|&m| {
                assert_eq!(m, 0x00);
            });
        });

        autd.close().unwrap();
    }

    #[test]
    fn freq_config() {
        let mut autd = Controller::builder()
            .advanced()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        for tr in autd.geometry_mut().iter_mut() {
            tr.set_cycle(2341).unwrap();
        }

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        autd.link().emulators().iter().for_each(|cpu| {
            cpu.fpga()
                .cycles()
                .iter()
                .for_each(|&c| assert_eq!(c, 2341))
        });

        autd.close().unwrap();
    }

    #[test]
    fn basic_usage_advanced() {
        let mut autd = Controller::builder()
            .advanced()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(
                Vector3::new(DEVICE_WIDTH, 0., 0.),
                Vector3::zeros(),
            ))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        assert!(autd.link().emulators().iter().all(|cpu| {
            cpu.fpga()
                .duties_and_phases(0)
                .iter()
                .all(|&(d, p)| d == 0x0000 && p == 0x0000)
        }));
        assert!(autd.link().emulators().iter().all(|cpu| cpu
            .fpga()
            .cycles()
            .iter()
            .all(|&c| c == 0x1000)));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| cpu.fpga().modulation_cycle() == 2
                && cpu.fpga().modulation_frequency_division() == 40960
                && cpu.fpga().modulation().iter().all(|&m| m == 0x00)));

        let silencer = SilencerConfig::default();
        autd.send(silencer).unwrap();

        let f = autd.geometry().center() + Vector3::new(0.0, 0.0, 150.0);
        let m = Sine::new(150);
        let g = Focus::new(f);

        autd.send((m, g)).unwrap();

        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga_flags().contains(FPGAControlFlags::LEGACY_MODE) }));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga_flags().contains(FPGAControlFlags::STM_MODE) }));

        let base_tr = &autd.geometry()[0];
        let expect = (autd3_core::acoustics::propagate::<autd3_core::acoustics::Sphere>(
            base_tr.position(),
            &base_tr.z_direction(),
            0.,
            base_tr.wavenumber(autd.geometry().sound_speed),
            &f,
        ) * Complex::new(
            0.,
            2. * PI * autd.link().emulators()[0].fpga().duties_and_phases(0)[0].1 as float
                / autd.link().emulators()[0].fpga().cycles()[0] as float,
        )
        .exp())
        .arg();

        autd.geometry()
            .iter()
            .zip(
                autd.link()
                    .emulators()
                    .iter()
                    .flat_map(|cpu| cpu.fpga().duties_and_phases(0)),
            )
            .zip(
                autd.link()
                    .emulators()
                    .iter()
                    .flat_map(|cpu| cpu.fpga().cycles()),
            )
            .for_each(|((tr, (d, p)), c)| {
                let p = (autd3_core::acoustics::propagate::<autd3_core::acoustics::Sphere>(
                    tr.position(),
                    &tr.z_direction(),
                    0.,
                    tr.wavenumber(autd.geometry().sound_speed),
                    &f,
                ) * Complex::new(0., 2. * PI * p as float / c as float).exp())
                .arg();
                assert_eq!(d, c >> 1);
                assert_approx_eq::assert_approx_eq!(p, expect, 2. * PI / 256.);
            });

        let expect_mod = {
            m.calc()
                .unwrap()
                .iter()
                .map(|d| (d.clamp(0., 1.).asin() * 2.0 / PI * 255.0) as u8)
                .collect::<Vec<_>>()
        };
        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().modulation().len(), expect_mod.len());
            cpu.fpga()
                .modulation()
                .iter()
                .zip(expect_mod.iter())
                .for_each(|(a, b)| assert_eq!(a, b));
        });

        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().silencer_step(), 10);
        });

        autd.send(Stop::new()).unwrap();
        autd.link().emulators().iter().for_each(|cpu| {
            cpu.fpga().duties_and_phases(0).iter().for_each(|&(d, _)| {
                assert_eq!(d, 0x0000);
            })
        });

        autd.send(Clear::new()).unwrap();
        autd.link().emulators().iter().for_each(|cpu| {
            cpu.fpga().duties_and_phases(0).iter().for_each(|&(d, p)| {
                assert_eq!(d, 0x0000);
                assert_eq!(p, 0x0000);
            });
            cpu.fpga().cycles().iter().for_each(|&c| {
                assert_eq!(c, 0x1000);
            });
            assert_eq!(cpu.fpga().modulation_cycle(), 2);
            assert_eq!(cpu.fpga().modulation_frequency_division(), 40960);
            cpu.fpga().modulation().iter().for_each(|&m| {
                assert_eq!(m, 0x00);
            });
        });

        autd.close().unwrap();
    }

    #[test]
    fn basic_usage_advanced_phase() {
        let mut autd = Controller::builder()
            .advanced_phase()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(
                Vector3::new(DEVICE_WIDTH, 0., 0.),
                Vector3::zeros(),
            ))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        assert!(autd.link().emulators().iter().all(|cpu| {
            cpu.fpga()
                .duties_and_phases(0)
                .iter()
                .all(|&(d, p)| d == 0x0000 && p == 0x0000)
        }));
        assert!(autd.link().emulators().iter().all(|cpu| cpu
            .fpga()
            .cycles()
            .iter()
            .all(|&c| c == 0x1000)));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| cpu.fpga().modulation_cycle() == 2
                && cpu.fpga().modulation_frequency_division() == 40960
                && cpu.fpga().modulation().iter().all(|&m| m == 0x00)));

        let silencer = SilencerConfig::default();
        autd.send(silencer).unwrap();

        let amp = Amplitudes::uniform(1.);
        autd.send(amp).unwrap();

        let f = autd.geometry().center() + Vector3::new(0.0, 0.0, 150.0);
        let m = Sine::new(150);
        let g = Focus::new(f);

        autd.send((m, g)).unwrap();

        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga_flags().contains(FPGAControlFlags::LEGACY_MODE) }));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga_flags().contains(FPGAControlFlags::STM_MODE) }));

        let base_tr = &autd.geometry()[0];
        let expect = (autd3_core::acoustics::propagate::<autd3_core::acoustics::Sphere>(
            base_tr.position(),
            &base_tr.z_direction(),
            0.,
            base_tr.wavenumber(autd.geometry().sound_speed),
            &f,
        ) * Complex::new(
            0.,
            2. * PI * autd.link().emulators()[0].fpga().duties_and_phases(0)[0].1 as float
                / autd.link().emulators()[0].fpga().cycles()[0] as float,
        )
        .exp())
        .arg();

        autd.geometry()
            .iter()
            .zip(
                autd.link()
                    .emulators()
                    .iter()
                    .flat_map(|cpu| cpu.fpga().duties_and_phases(0)),
            )
            .zip(
                autd.link()
                    .emulators()
                    .iter()
                    .flat_map(|cpu| cpu.fpga().cycles()),
            )
            .for_each(|((tr, (d, p)), c)| {
                let p = (autd3_core::acoustics::propagate::<autd3_core::acoustics::Sphere>(
                    tr.position(),
                    &tr.z_direction(),
                    0.,
                    tr.wavenumber(autd.geometry().sound_speed),
                    &f,
                ) * Complex::new(0., 2. * PI * p as float / c as float).exp())
                .arg();
                assert_eq!(d, c >> 1);
                assert_approx_eq::assert_approx_eq!(p, expect, 2. * PI / 256.);
            });

        let expect_mod = {
            m.calc()
                .unwrap()
                .iter()
                .map(|d| (d.clamp(0., 1.).asin() * 2.0 / PI * 255.0) as u8)
                .collect::<Vec<_>>()
        };
        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().modulation().len(), expect_mod.len());
            cpu.fpga()
                .modulation()
                .iter()
                .zip(expect_mod.iter())
                .for_each(|(a, b)| assert_eq!(a, b));
        });

        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().silencer_step(), 10);
        });

        autd.send(Stop::new()).unwrap();
        autd.link().emulators().iter().for_each(|cpu| {
            cpu.fpga().duties_and_phases(0).iter().for_each(|&(d, _)| {
                assert_eq!(d, 0x0000);
            })
        });

        autd.send(Clear::new()).unwrap();
        autd.link().emulators().iter().for_each(|cpu| {
            cpu.fpga().duties_and_phases(0).iter().for_each(|&(d, p)| {
                assert_eq!(d, 0x0000);
                assert_eq!(p, 0x0000);
            });
            cpu.fpga().cycles().iter().for_each(|&c| {
                assert_eq!(c, 0x1000);
            });
            assert_eq!(cpu.fpga().modulation_cycle(), 2);
            assert_eq!(cpu.fpga().modulation_frequency_division(), 40960);
            cpu.fpga().modulation().iter().for_each(|&m| {
                assert_eq!(m, 0x00);
            });
        });

        autd.close().unwrap();
    }

    #[test]
    fn focus_stm() {
        let mut autd = Controller::builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        let center = autd.geometry().center();
        let size = 200;
        let points = (0..size)
            .map(|i| {
                let theta = 2. * PI * i as float / size as float;
                center + Vector3::new(30. * theta.cos(), 30. * theta.sin(), 150.)
            })
            .collect::<Vec<_>>();
        let stm = FocusSTM::new(1.).add_foci_from_iter(&points);

        autd.send(stm.clone()).unwrap();

        autd.link()
            .emulators()
            .iter()
            .for_each(|cpu| assert_eq!(cpu.fpga().stm_cycle(), size));

        autd.link().emulators().iter().for_each(|cpu| {
            assert!(cpu.fpga().stm_start_idx().is_none());
            assert!(cpu.fpga().stm_finish_idx().is_none());
        });

        let base_tr = &autd.geometry()[0];
        (0..size).for_each(|k| {
            let f = points[k];
            let expect = (autd3_core::acoustics::propagate::<autd3_core::acoustics::Sphere>(
                base_tr.position(),
                &base_tr.z_direction(),
                0.,
                base_tr.wavenumber(autd.geometry().sound_speed),
                &f,
            ) * Complex::new(
                0.,
                2. * PI * autd.link().emulators()[0].fpga().duties_and_phases(k)[0].1 as float
                    / autd.link().emulators()[0].fpga().cycles()[0] as float,
            )
            .exp())
            .arg();
            autd.geometry()
                .iter()
                .zip(
                    autd.link()
                        .emulators()
                        .iter()
                        .flat_map(|cpu| cpu.fpga().duties_and_phases(k)),
                )
                .zip(
                    autd.link()
                        .emulators()
                        .iter()
                        .flat_map(|cpu| cpu.fpga().cycles()),
                )
                .for_each(|((tr, (d, p)), c)| {
                    let p = (autd3_core::acoustics::propagate::<autd3_core::acoustics::Sphere>(
                        tr.position(),
                        &tr.z_direction(),
                        0.,
                        tr.wavenumber(autd.geometry().sound_speed),
                        &f,
                    ) * Complex::new(0., 2. * PI * p as float / c as float).exp())
                    .arg();
                    assert_eq!(d, c >> 1);
                    assert_approx_eq::assert_approx_eq!(p, expect, 2. * PI / 100.);
                });
        });

        let stm = stm.with_start_idx(Some(1)).with_finish_idx(Some(2));
        autd.send(stm).unwrap();

        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().stm_start_idx(), Some(1));
            assert_eq!(cpu.fpga().stm_finish_idx(), Some(2));
        });

        autd.close().unwrap();
    }

    #[test]
    fn gain_stm_legacy() {
        let mut autd = Controller::builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        let center = autd.geometry().center();
        let size = 30;

        let gains = (0..size)
            .map(|i| {
                let theta = 2. * PI * i as float / size as float;
                Focus::new(center + Vector3::new(30. * theta.cos(), 30. * theta.sin(), 150.))
            })
            .collect::<Vec<_>>();

        let stm = GainSTM::new(1.).add_gains_from_iter(gains.iter().map(|g| Box::new(*g) as _));

        autd.send(&stm).unwrap();

        autd.link()
            .emulators()
            .iter()
            .for_each(|cpu| assert_eq!(cpu.fpga().stm_cycle(), size));

        autd.link().emulators().iter().for_each(|cpu| {
            assert!(cpu.fpga().stm_start_idx().is_none());
            assert!(cpu.fpga().stm_finish_idx().is_none());
        });

        (0..size).for_each(|k| {
            autd.link()
                .emulators()
                .iter()
                .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
                .zip(gains[k].calc(autd.geometry()).unwrap())
                .for_each(|((d, p), g)| {
                    assert_eq!(
                        d,
                        ((autd3_core::LegacyDrive::to_duty(&g) as u16) << 3) + 0x08
                    );
                    assert_eq!(p, (autd3_core::LegacyDrive::to_phase(&g) as u16) << 4);
                });
        });

        let stm = stm.with_start_idx(Some(1)).with_finish_idx(Some(2));
        autd.send(&stm).unwrap();

        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().stm_start_idx(), Some(1));
            assert_eq!(cpu.fpga().stm_finish_idx(), Some(2));
        });

        let stm = stm.with_mode(Mode::PhaseFull);
        autd.send(&stm).unwrap();

        (0..size).for_each(|k| {
            autd.link()
                .emulators()
                .iter()
                .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
                .zip(gains[k].calc(autd.geometry()).unwrap())
                .zip(
                    autd.link()
                        .emulators()
                        .iter()
                        .flat_map(|cpu| cpu.fpga().cycles()),
                )
                .for_each(|(((d, p), g), c)| {
                    assert_eq!(d, c >> 1);
                    assert_eq!(p, (autd3_core::LegacyDrive::to_phase(&g) as u16) << 4);
                });
        });

        let stm = stm.with_mode(Mode::PhaseHalf);
        autd.send(&stm).unwrap();

        (0..size).for_each(|k| {
            autd.link()
                .emulators()
                .iter()
                .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
                .zip(gains[k].calc(autd.geometry()).unwrap())
                .zip(
                    autd.link()
                        .emulators()
                        .iter()
                        .flat_map(|cpu| cpu.fpga().cycles()),
                )
                .for_each(|(((d, p), g), c)| {
                    assert_eq!(d, c >> 1);
                    let phase = (autd3_core::LegacyDrive::to_phase(&g) as u16) >> 4;
                    let phase = ((phase << 4) + phase) << 4;
                    assert_eq!(p, phase);
                });
        });

        autd.close().unwrap();
    }

    #[test]
    fn gain_stm_advanced() {
        let mut autd = Controller::builder()
            .advanced()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        let center = autd.geometry().center();
        let size = 30;

        let gains = (0..size)
            .map(|i| {
                let theta = 2. * PI * i as float / size as float;
                Focus::new(center + Vector3::new(30. * theta.cos(), 30. * theta.sin(), 150.))
            })
            .collect::<Vec<_>>();

        let stm = GainSTM::new(1.).add_gains_from_iter(gains.iter().map(|g| Box::new(*g) as _));

        autd.send(&stm).unwrap();

        autd.link()
            .emulators()
            .iter()
            .for_each(|cpu| assert_eq!(cpu.fpga().stm_cycle(), size));

        autd.link().emulators().iter().for_each(|cpu| {
            assert!(cpu.fpga().stm_start_idx().is_none());
            assert!(cpu.fpga().stm_finish_idx().is_none());
        });

        (0..size).for_each(|k| {
            autd.link()
                .emulators()
                .iter()
                .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
                .zip(gains[k].calc(autd.geometry()).unwrap())
                .zip(
                    autd.link()
                        .emulators()
                        .iter()
                        .flat_map(|cpu| cpu.fpga().cycles()),
                )
                .for_each(|(((d, p), g), c)| {
                    assert_eq!(d, autd3_core::AdvancedDriveDuty::to_duty(&g, c));
                    assert_eq!(p, autd3_core::AdvancedDrivePhase::to_phase(&g, c));
                });
        });

        let stm = stm.with_start_idx(Some(1)).with_finish_idx(Some(2));
        autd.send(&stm).unwrap();

        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().stm_start_idx(), Some(1));
            assert_eq!(cpu.fpga().stm_finish_idx(), Some(2));
        });

        let stm = stm.with_mode(Mode::PhaseFull);
        autd.send(&stm).unwrap();

        (0..size).for_each(|k| {
            autd.link()
                .emulators()
                .iter()
                .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
                .zip(gains[k].calc(autd.geometry()).unwrap())
                .zip(
                    autd.link()
                        .emulators()
                        .iter()
                        .flat_map(|cpu| cpu.fpga().cycles()),
                )
                .for_each(|(((d, p), g), c)| {
                    assert_eq!(d, c >> 1);
                    assert_eq!(p, autd3_core::AdvancedDrivePhase::to_phase(&g, c));
                });
        });

        let stm = stm.with_mode(Mode::PhaseHalf);
        assert!(autd.send(&stm).is_err());

        autd.close().unwrap();
    }

    #[test]
    fn gain_stm_advanced_phase() {
        let mut autd = Controller::builder()
            .advanced_phase()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        autd.send(Amplitudes::none()).unwrap();

        let center = autd.geometry().center();
        let size = 30;

        let gains = (0..size)
            .map(|i| {
                let theta = 2. * PI * i as float / size as float;
                Focus::new(center + Vector3::new(30. * theta.cos(), 30. * theta.sin(), 150.))
            })
            .collect::<Vec<_>>();

        let stm = GainSTM::new(1.).add_gains_from_iter(gains.iter().map(|g| Box::new(*g) as _));
        autd.send(&stm).unwrap();

        autd.link()
            .emulators()
            .iter()
            .for_each(|cpu| assert_eq!(cpu.fpga().stm_cycle(), size));

        autd.link().emulators().iter().for_each(|cpu| {
            assert!(cpu.fpga().stm_start_idx().is_none());
            assert!(cpu.fpga().stm_finish_idx().is_none());
        });

        (0..size).for_each(|k| {
            autd.link()
                .emulators()
                .iter()
                .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
                .zip(gains[k].calc(autd.geometry()).unwrap())
                .zip(
                    autd.link()
                        .emulators()
                        .iter()
                        .flat_map(|cpu| cpu.fpga().cycles()),
                )
                .for_each(|(((d, p), g), c)| {
                    assert_eq!(d, c >> 1);
                    assert_eq!(p, autd3_core::AdvancedDrivePhase::to_phase(&g, c));
                });
        });

        let stm = stm.with_start_idx(Some(1)).with_finish_idx(Some(2));
        autd.send(&stm).unwrap();

        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().stm_start_idx(), Some(1));
            assert_eq!(cpu.fpga().stm_finish_idx(), Some(2));
        });

        let stm = stm.with_mode(Mode::PhaseFull);
        autd.send(&stm).unwrap();

        (0..size).for_each(|k| {
            autd.link()
                .emulators()
                .iter()
                .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
                .zip(gains[k].calc(autd.geometry()).unwrap())
                .zip(
                    autd.link()
                        .emulators()
                        .iter()
                        .flat_map(|cpu| cpu.fpga().cycles()),
                )
                .for_each(|(((d, p), g), c)| {
                    assert_eq!(d, c >> 1);
                    assert_eq!(p, autd3_core::AdvancedDrivePhase::to_phase(&g, c));
                });
        });

        let stm = stm.with_mode(Mode::PhaseHalf);
        assert!(autd.send(&stm).is_err());

        autd.close().unwrap();
    }
}
