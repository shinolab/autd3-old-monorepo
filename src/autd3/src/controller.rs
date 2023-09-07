/*
 * File: controller.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3_driver::{
    cpu::{RxDatagram, TxDatagram},
    datagram::{Clear, Datagram, Synchronize, UpdateFlags},
    firmware_version::FirmwareInfo,
    fpga::FPGAInfo,
    geometry::{
        AdvancedPhaseTransducer, AdvancedTransducer, Device, IntoDevice, LegacyTransducer,
        Transducer,
    },
    link::Link,
    operation::OperationHandler,
};

use crate::{error::AUTDError, geometry::Geometry, link::NullLink, software_stm::SoftwareSTM};

/// Builder for `Controller`
pub struct ControllerBuilder<T: Transducer> {
    devices: Vec<Device<T>>,
}

impl<T: Transducer> Default for ControllerBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Transducer> ControllerBuilder<T> {
    #[doc(hidden)]
    /// This is used only for capi.
    pub fn new() -> ControllerBuilder<T> {
        Self { devices: vec![] }
    }

    /// Add device
    pub fn add_device<D: IntoDevice<T>>(mut self, dev: D) -> Self {
        self.devices.push(dev.into_device(self.devices.len()));
        self
    }

    /// Open controller
    pub fn open_with<L: Link<T>>(self, link: L) -> Result<Controller<T, L>, AUTDError> {
        Controller::open_impl(Geometry::<T>::new(self.devices), link)
    }
}

/// Controller for AUTD
pub struct Controller<T: Transducer, L: Link<T>> {
    link: L,
    geometry: Geometry<T>,
    tx_buf: TxDatagram,
    rx_buf: RxDatagram,
}

impl Controller<LegacyTransducer, NullLink> {
    /// Create Controller builder (legacy mode)
    pub fn builder() -> ControllerBuilder<LegacyTransducer> {
        ControllerBuilder::<LegacyTransducer>::new()
    }

    /// Create Controller builder (legacy mode)
    pub fn legacy_builder() -> ControllerBuilder<LegacyTransducer> {
        ControllerBuilder::<LegacyTransducer>::new()
    }

    /// Create Controller builder (advanced mode)
    pub fn advanced_builder() -> ControllerBuilder<AdvancedTransducer> {
        ControllerBuilder::<AdvancedTransducer>::new()
    }

    /// Create Controller builder (advanced phase mode)
    pub fn advanced_phase_builder() -> ControllerBuilder<AdvancedPhaseTransducer> {
        ControllerBuilder::<AdvancedPhaseTransducer>::new()
    }
}

impl<T: Transducer, L: Link<T>> Controller<T, L> {
    #[doc(hidden)]
    pub fn open_impl(geometry: Geometry<T>, link: L) -> Result<Controller<T, L>, AUTDError> {
        let mut link = link;
        link.open(&geometry)?;
        let num_devices = geometry.num_devices();
        let tx_buf = TxDatagram::new(num_devices);
        let mut cnt = Controller {
            link,
            geometry,
            tx_buf,
            rx_buf: RxDatagram::new(num_devices),
        };
        cnt.send(UpdateFlags::new())?;
        cnt.send(Clear::new())?;
        cnt.send(Synchronize::new())?;
        Ok(cnt)
    }

    /// get geometry
    pub const fn geometry(&self) -> &Geometry<T> {
        &self.geometry
    }

    /// get geometry mutably
    pub fn geometry_mut(&mut self) -> &mut Geometry<T> {
        &mut self.geometry
    }

    /// get link
    pub const fn link(&self) -> &L {
        &self.link
    }

    /// get link mutably
    pub fn link_mut(&self) -> &L {
        &self.link
    }

    /// Send data to the devices
    ///
    /// # Arguments
    ///
    /// * `s` - Datagram
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - It is confirmed that the data has been successfully transmitted
    /// * `Ok(false)` - There are no errors, but it is unclear whether the data has been sent reliably or not
    ///
    pub fn send<S: Datagram<T>>(&mut self, s: S) -> Result<bool, AUTDError> {
        let timeout = s.timeout().unwrap_or(self.link.timeout());

        let (mut op1, mut op2) = s.operation()?;
        OperationHandler::init(&mut op1, &mut op2, self.geometry.iter())?;
        loop {
            OperationHandler::pack(&mut op1, &mut op2, self.geometry.iter(), &mut self.tx_buf)?;

            if !self
                .link
                .send_receive(&self.tx_buf, &mut self.rx_buf, timeout)?
            {
                return Ok(false);
            }
            if OperationHandler::is_finished(&mut op1, &mut op2, self.geometry.iter()) {
                break;
            }
            if timeout.is_zero() {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        Ok(true)
    }

    /// Send data to the devices asynchronously
    ///
    /// # Arguments
    ///
    /// * `s` - Datagram
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - It is confirmed that the data has been successfully transmitted
    /// * `Ok(false)` - There are no errors, but it is unclear whether the data has been sent reliably or not
    ///
    pub async fn send_async<S: Datagram<T>>(&mut self, s: S) -> Result<bool, AUTDError> {
        async { self.send(s) }.await
    }

    // Close connection
    pub fn close(&mut self) -> Result<bool, AUTDError> {
        if !self.link.is_open() {
            return Ok(false);
        }
        let res = true;
        // let res = self.send(Stop::new())?;
        let res = res & self.send(Clear::new())?;
        self.link.close()?;
        Ok(res)
    }

    /// Get firmware information
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<FirmwareInfo>)` - List of firmware information
    ///
    pub fn firmware_infos(&mut self) -> Result<Vec<FirmwareInfo>, AUTDError> {
        let mut op = autd3_driver::operation::FirmInfoOp::default();
        let mut null_op = autd3_driver::operation::NullOp::default();

        OperationHandler::init(&mut op, &mut null_op, self.geometry.iter())?;

        OperationHandler::pack(
            &mut op,
            &mut null_op,
            self.geometry.iter(),
            &mut self.tx_buf,
        )?;
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let cpu_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(
            &mut op,
            &mut null_op,
            self.geometry.iter(),
            &mut self.tx_buf,
        )?;
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let cpu_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(
            &mut op,
            &mut null_op,
            self.geometry.iter(),
            &mut self.tx_buf,
        )?;
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let fpga_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(
            &mut op,
            &mut null_op,
            self.geometry.iter(),
            &mut self.tx_buf,
        )?;
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let fpga_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(
            &mut op,
            &mut null_op,
            self.geometry.iter(),
            &mut self.tx_buf,
        )?;
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let fpga_functions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(
            &mut op,
            &mut null_op,
            self.geometry.iter(),
            &mut self.tx_buf,
        )?;
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;

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

    /// Get FPGA information
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<FPGAInfo>)` - List of FPGA information
    ///
    pub fn fpga_info(&mut self) -> Result<Vec<FPGAInfo>, AUTDError> {
        self.link.receive(&mut self.rx_buf)?;
        Ok(self.rx_buf.iter().map(FPGAInfo::from).collect())
    }

    /// Start software Spatio-Temporal Modulation
    ///
    /// # Arguments
    ///
    /// * `callback` - Callback function called specified interval
    /// * `finish_handler` - If this function returns true, STM will be finished
    ///
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

#[cfg(test)]
mod tests {
    use crate::{
        autd3_device::AUTD3,
        link::Debug,
        prelude::{Focus, Sine},
    };

    use autd3_driver::{
        acoustics::{propagate, Complex, Sphere},
        datagram::{
            Amplitudes, FocusSTM, Gain, GainFilter, GainSTM, Modulation, Silencer, Stop,
            Synchronize,
        },
        defined::{float, PI},
        fpga::LegacyDrive,
        geometry::Vector3,
        operation::GainSTMMode,
    };

    use super::*;
    use spdlog::LevelFilter;

    #[test]
    fn basic_usage() {
        let mut autd = Controller::builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        let firm_infos = autd.firmware_infos().unwrap();
        assert_eq!(firm_infos.len(), autd.geometry().num_devices());
        firm_infos.iter().for_each(|f| {
            assert_eq!(f.cpu_version(), "v3.0.0");
            assert_eq!(f.fpga_version(), "v3.0.0");
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

        let silencer = Silencer::default();
        autd.send(silencer).unwrap();

        let f = autd.geometry().center() + Vector3::new(0.0, 0.0, 150.0);
        let m = Sine::new(150);
        let g = Focus::new(f);

        autd.send((m, g)).unwrap();

        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { cpu.fpga().is_legacy_mode() }));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga().is_stm_mode() }));

        let base_tr = &autd.geometry()[0][0];
        let expect = (propagate::<Sphere>(
            base_tr.position(),
            &base_tr.z_direction(),
            0.,
            base_tr.wavenumber(autd.geometry()[0].sound_speed),
            &f,
        ) * Complex::new(
            0.,
            2. * PI * autd.link().emulators()[0].fpga().duties_and_phases(0)[0].1 as float
                / autd.link().emulators()[0].fpga().cycles()[0] as float,
        )
        .exp())
        .arg();

        autd.geometry()[0]
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
                let p = (propagate::<Sphere>(
                    tr.position(),
                    &tr.z_direction(),
                    0.,
                    tr.wavenumber(autd.geometry()[0].sound_speed),
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
        let mut autd = Controller::advanced_builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

        for tr in autd.geometry_mut()[0].iter_mut() {
            tr.set_cycle(2341).unwrap();
        }

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
        let mut autd = Controller::advanced_builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(
                Vector3::new(AUTD3::DEVICE_WIDTH, 0., 0.),
                Vector3::zeros(),
            ))
            .open_with(Debug::new().with_log_level(LevelFilter::Off))
            .unwrap();

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
            .all(|&c| { c == 0x1000 })));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| cpu.fpga().modulation_cycle() == 2
                && cpu.fpga().modulation_frequency_division() == 40960
                && cpu.fpga().modulation().iter().all(|&m| m == 0x00)));

        let silencer = Silencer::default();
        autd.send(silencer).unwrap();

        let f = autd.geometry().center() + Vector3::new(0.0, 0.0, 150.0);
        let m = Sine::new(150);
        let g = Focus::new(f);

        autd.send((m, g)).unwrap();

        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga().is_legacy_mode() }));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga().is_stm_mode() }));

        let base_tr = &autd.geometry()[0][0];
        let expect = (propagate::<Sphere>(
            base_tr.position(),
            &base_tr.z_direction(),
            0.,
            base_tr.wavenumber(autd.geometry()[0].sound_speed),
            &f,
        ) * Complex::new(
            0.,
            2. * PI * autd.link().emulators()[0].fpga().duties_and_phases(0)[0].1 as float
                / autd.link().emulators()[0].fpga().cycles()[0] as float,
        )
        .exp())
        .arg();

        let sound_speed = autd.geometry()[0].sound_speed;
        autd.geometry()
            .iter()
            .flat_map(|dev| dev.iter())
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
                let p = (propagate::<Sphere>(
                    tr.position(),
                    &tr.z_direction(),
                    0.,
                    tr.wavenumber(sound_speed),
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
        let mut autd = Controller::advanced_phase_builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(
                Vector3::new(AUTD3::DEVICE_WIDTH, 0., 0.),
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

        let silencer = Silencer::default();
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
            .all(|cpu| { !cpu.fpga().is_legacy_mode() }));
        assert!(autd
            .link()
            .emulators()
            .iter()
            .all(|cpu| { !cpu.fpga().is_stm_mode() }));

        let base_tr = &autd.geometry()[0][0];
        let expect = (propagate::<Sphere>(
            base_tr.position(),
            &base_tr.z_direction(),
            0.,
            base_tr.wavenumber(autd.geometry()[0].sound_speed),
            &f,
        ) * Complex::new(
            0.,
            2. * PI * autd.link().emulators()[0].fpga().duties_and_phases(0)[0].1 as float
                / autd.link().emulators()[0].fpga().cycles()[0] as float,
        )
        .exp())
        .arg();

        let sound_speed = autd.geometry()[0].sound_speed;
        autd.geometry()
            .iter()
            .flat_map(|dev| dev.iter())
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
                let p = (propagate::<Sphere>(
                    tr.position(),
                    &tr.z_direction(),
                    0.,
                    tr.wavenumber(sound_speed),
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

        let base_tr = &autd.geometry()[0][0];
        let sound_speed = autd.geometry()[0].sound_speed;
        (0..size).for_each(|k| {
            let f = points[k];
            let expect = (propagate::<Sphere>(
                base_tr.position(),
                &base_tr.z_direction(),
                0.,
                base_tr.wavenumber(sound_speed),
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
                .flat_map(|dev| dev.iter())
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
                    let p = (propagate::<Sphere>(
                        tr.position(),
                        &tr.z_direction(),
                        0.,
                        tr.wavenumber(sound_speed),
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

        let center = autd.geometry().center();
        let size = 30;

        let gains = (0..size)
            .map(|i| {
                let theta = 2. * PI * i as float / size as float;
                Focus::new(center + Vector3::new(30. * theta.cos(), 30. * theta.sin(), 150.))
            })
            .collect::<Vec<_>>();

        let stm = GainSTM::new(1.).add_gains_from_iter(gains.iter().copied());

        autd.send(stm.clone()).unwrap();

        autd.link()
            .emulators()
            .iter()
            .for_each(|cpu| assert_eq!(cpu.fpga().stm_cycle(), size));

        autd.link().emulators().iter().for_each(|cpu| {
            assert!(cpu.fpga().stm_start_idx().is_none());
            assert!(cpu.fpga().stm_finish_idx().is_none());
        });

        autd.link().emulators().iter().for_each(|cpu| {
            assert!(cpu.fpga().is_legacy_mode());
        });

        (0..size).for_each(|k| {
            let g = gains[k]
                .calc(&autd.geometry().iter().collect::<Vec<_>>(), GainFilter::All)
                .unwrap();
            autd.link().emulators().iter().for_each(|cpu| {
                cpu.fpga()
                    .duties_and_phases(k)
                    .iter()
                    .zip(g[&cpu.idx()].iter())
                    .for_each(|(&(d, p), g)| {
                        assert_eq!(d, ((LegacyDrive::to_duty(g) as u16) << 3) + 0x08);
                        assert_eq!(p, (LegacyDrive::to_phase(g) as u16) << 4);
                    });
            })
        });

        let stm = stm.with_start_idx(Some(1)).with_finish_idx(Some(2));
        autd.send(stm.clone()).unwrap();

        autd.link().emulators().iter().for_each(|cpu| {
            assert_eq!(cpu.fpga().stm_start_idx(), Some(1));
            assert_eq!(cpu.fpga().stm_finish_idx(), Some(2));
        });

        autd.link().emulators().iter().for_each(|cpu| {
            assert!(cpu.fpga().is_legacy_mode());
        });

        let stm = stm.with_mode(GainSTMMode::PhaseFull);
        autd.send(stm.clone()).unwrap();

        (0..size).for_each(|k| {
            let g = gains[k]
                .calc(&autd.geometry().iter().collect::<Vec<_>>(), GainFilter::All)
                .unwrap();
            autd.link().emulators().iter().for_each(|cpu| {
                cpu.fpga()
                    .duties_and_phases(k)
                    .iter()
                    .zip(g[&cpu.idx()].iter())
                    .zip(cpu.fpga().cycles().iter())
                    .for_each(|((&(d, p), g), c)| {
                        assert_eq!(d, c >> 1);
                        assert_eq!(p, (LegacyDrive::to_phase(g) as u16) << 4);
                    })
            });
        });

        let stm = stm.with_mode(GainSTMMode::PhaseHalf);
        autd.send(stm.clone()).unwrap();

        autd.link().emulators().iter().for_each(|cpu| {
            assert!(cpu.fpga().is_legacy_mode());
        });

        (0..size).for_each(|k| {
            let g = gains[k]
                .calc(&autd.geometry().iter().collect::<Vec<_>>(), GainFilter::All)
                .unwrap();
            autd.link().emulators().iter().for_each(|cpu| {
                cpu.fpga()
                    .duties_and_phases(k)
                    .iter()
                    .zip(g[&cpu.idx()].iter())
                    .zip(cpu.fpga().cycles().iter())
                    .for_each(|((&(d, p), g), c)| {
                        assert_eq!(d, c >> 1);
                        let phase = (LegacyDrive::to_phase(g) as u16) >> 4;
                        let phase = ((phase << 4) + phase) << 4;
                        assert_eq!(p, phase);
                    })
            });
        });

        autd.close().unwrap();
    }

    // #[test]
    // fn gain_stm_advanced() {
    //     let mut autd = Controller::builder()
    //         .advanced()
    //         .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    //         .open_with(Debug::new().with_log_level(LevelFilter::Off))
    //         .unwrap();

    //     autd.send(Clear::new()).unwrap();
    //     autd.send(Synchronize::new()).unwrap();

    //     let center = autd.geometry().center();
    //     let size = 30;

    //     let gains = (0..size)
    //         .map(|i| {
    //             let theta = 2. * PI * i as float / size as float;
    //             Focus::new(center + Vector3::new(30. * theta.cos(), 30. * theta.sin(), 150.))
    //         })
    //         .collect::<Vec<_>>();

    //     let stm = GainSTM::new(1.).add_gains_from_iter(gains.iter().copied());

    //     autd.send(&stm).unwrap();

    //     autd.link()
    //         .emulators()
    //         .iter()
    //         .for_each(|cpu| assert_eq!(cpu.fpga().stm_cycle(), size));

    //     autd.link().emulators().iter().for_each(|cpu| {
    //         assert!(cpu.fpga().stm_start_idx().is_none());
    //         assert!(cpu.fpga().stm_finish_idx().is_none());
    //     });

    //     (0..size).for_each(|k| {
    //         autd.link()
    //             .emulators()
    //             .iter()
    //             .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
    //             .zip(gains[k].calc(autd.geometry(), GainFilter::All).unwrap())
    //             .zip(
    //                 autd.link()
    //                     .emulators()
    //                     .iter()
    //                     .flat_map(|cpu| cpu.fpga().cycles()),
    //             )
    //             .for_each(|(((d, p), g), c)| {
    //                 assert_eq!(d, autd3_core::AdvancedDriveDuty::to_duty(&g, c));
    //                 assert_eq!(p, autd3_core::AdvancedDrivePhase::to_phase(&g, c));
    //             });
    //     });

    //     let stm = stm.with_start_idx(Some(1)).with_finish_idx(Some(2));
    //     autd.send(&stm).unwrap();

    //     autd.link().emulators().iter().for_each(|cpu| {
    //         assert_eq!(cpu.fpga().stm_start_idx(), Some(1));
    //         assert_eq!(cpu.fpga().stm_finish_idx(), Some(2));
    //     });

    //     let stm = stm.with_mode(Mode::PhaseFull);
    //     autd.send(&stm).unwrap();

    //     (0..size).for_each(|k| {
    //         autd.link()
    //             .emulators()
    //             .iter()
    //             .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
    //             .zip(gains[k].calc(autd.geometry(), GainFilter::All).unwrap())
    //             .zip(
    //                 autd.link()
    //                     .emulators()
    //                     .iter()
    //                     .flat_map(|cpu| cpu.fpga().cycles()),
    //             )
    //             .for_each(|(((d, p), g), c)| {
    //                 assert_eq!(d, c >> 1);
    //                 assert_eq!(p, autd3_core::AdvancedDrivePhase::to_phase(&g, c));
    //             });
    //     });

    //     let stm = stm.with_mode(Mode::PhaseHalf);
    //     assert!(autd.send(&stm).is_err());

    //     autd.close().unwrap();
    // }

    // #[test]
    // fn gain_stm_advanced_phase() {
    //     let mut autd = Controller::builder()
    //         .advanced_phase()
    //         .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    //         .open_with(Debug::new().with_log_level(LevelFilter::Off))
    //         .unwrap();

    //     autd.send(Clear::new()).unwrap();
    //     autd.send(Synchronize::new()).unwrap();

    //     autd.send(Amplitudes::none()).unwrap();

    //     let center = autd.geometry().center();
    //     let size = 30;

    //     let gains = (0..size)
    //         .map(|i| {
    //             let theta = 2. * PI * i as float / size as float;
    //             Focus::new(center + Vector3::new(30. * theta.cos(), 30. * theta.sin(), 150.))
    //         })
    //         .collect::<Vec<_>>();

    //     let stm = GainSTM::new(1.).add_gains_from_iter(gains.iter().copied());
    //     autd.send(&stm).unwrap();

    //     autd.link()
    //         .emulators()
    //         .iter()
    //         .for_each(|cpu| assert_eq!(cpu.fpga().stm_cycle(), size));

    //     autd.link().emulators().iter().for_each(|cpu| {
    //         assert!(cpu.fpga().stm_start_idx().is_none());
    //         assert!(cpu.fpga().stm_finish_idx().is_none());
    //     });

    //     (0..size).for_each(|k| {
    //         autd.link()
    //             .emulators()
    //             .iter()
    //             .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
    //             .zip(gains[k].calc(autd.geometry(), GainFilter::All).unwrap())
    //             .zip(
    //                 autd.link()
    //                     .emulators()
    //                     .iter()
    //                     .flat_map(|cpu| cpu.fpga().cycles()),
    //             )
    //             .for_each(|(((d, p), g), c)| {
    //                 assert_eq!(d, c >> 1);
    //                 assert_eq!(p, autd3_core::AdvancedDrivePhase::to_phase(&g, c));
    //             });
    //     });

    //     let stm = stm.with_start_idx(Some(1)).with_finish_idx(Some(2));
    //     autd.send(&stm).unwrap();

    //     autd.link().emulators().iter().for_each(|cpu| {
    //         assert_eq!(cpu.fpga().stm_start_idx(), Some(1));
    //         assert_eq!(cpu.fpga().stm_finish_idx(), Some(2));
    //     });

    //     let stm = stm.with_mode(Mode::PhaseFull);
    //     autd.send(&stm).unwrap();

    //     (0..size).for_each(|k| {
    //         autd.link()
    //             .emulators()
    //             .iter()
    //             .flat_map(|cpu| cpu.fpga().duties_and_phases(k))
    //             .zip(gains[k].calc(autd.geometry(), GainFilter::All).unwrap())
    //             .zip(
    //                 autd.link()
    //                     .emulators()
    //                     .iter()
    //                     .flat_map(|cpu| cpu.fpga().cycles()),
    //             )
    //             .for_each(|(((d, p), g), c)| {
    //                 assert_eq!(d, c >> 1);
    //                 assert_eq!(p, autd3_core::AdvancedDrivePhase::to_phase(&g, c));
    //             });
    //     });

    //     let stm = stm.with_mode(Mode::PhaseHalf);
    //     assert!(autd.send(&stm).is_err());

    //     autd.close().unwrap();
    // }
}
