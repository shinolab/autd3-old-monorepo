/*
 * File: controller.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    sync::atomic::{self, AtomicU8},
    time::Duration,
};

use autd3_core::{
    clear::Clear, geometry::*, link::Link, sendable::Sendable, stop::Stop, FirmwareInfo, Operation,
    RxDatagram, TxDatagram, MSG_BEGIN, MSG_END,
};

use crate::error::AUTDError;

pub struct Controller<T: Transducer, L: Link<T>> {
    link: L,
    geometry: Geometry<T>,
    tx_buf: TxDatagram,
    rx_buf: RxDatagram,
    force_fan: autd3_core::ForceFan,
    reads_fpga_info: autd3_core::ReadsFPGAInfo,
    msg_id: AtomicU8,
}

impl<T: Transducer, L: Link<T>> Controller<T, L> {
    pub fn open(geometry: Geometry<T>, link: L) -> Result<Controller<T, L>, AUTDError> {
        let mut link = link;
        link.open(&geometry)?;
        let num_devices = geometry.num_devices();
        let tx_buf = TxDatagram::new(geometry.device_map());
        Ok(Controller {
            link,
            geometry,
            tx_buf,
            rx_buf: RxDatagram::new(num_devices),
            force_fan: autd3_core::ForceFan::default(),
            reads_fpga_info: autd3_core::ReadsFPGAInfo::default(),
            msg_id: AtomicU8::new(MSG_BEGIN),
        })
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

    /// Send header and body to the devices
    ///
    /// # Arguments
    ///
    /// * `header` - Header
    /// * `body` - Body
    ///
    pub fn send<S: Sendable<T>>(&mut self, s: S) -> Result<bool, AUTDError> {
        self.send_with_timeout(s, None)
    }

    pub fn send_with_timeout<S: Sendable<T>>(
        &mut self,
        s: S,
        timeout: Option<Duration>,
    ) -> Result<bool, AUTDError> {
        let mut s = s;
        let (mut op_header, mut op_body) = s.operation(&self.geometry)?;

        op_header.init();
        op_body.init();

        self.force_fan.pack(&mut self.tx_buf);
        self.reads_fpga_info.pack(&mut self.tx_buf);

        let timeout = timeout.unwrap_or(s.timeout().unwrap_or(self.link.timeout()));
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

    pub fn close(&mut self) -> Result<bool, AUTDError> {
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
        let cpu_versions = self
            .rx_buf
            .messages()
            .iter()
            .map(|rx| rx.ack)
            .collect::<Vec<_>>();

        let mut op = autd3_core::FPGAVersionMajor::default();
        op.pack(&mut self.tx_buf);
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let fpga_versions = self
            .rx_buf
            .messages()
            .iter()
            .map(|rx| rx.ack)
            .collect::<Vec<_>>();

        let mut op = autd3_core::FPGAFunctions::default();
        op.pack(&mut self.tx_buf);
        self.link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?;
        let fpga_functions = self
            .rx_buf
            .messages()
            .iter()
            .map(|rx| rx.ack)
            .collect::<Vec<_>>();

        let mut op = autd3_core::FPGAVersionMinor::default();
        op.pack(&mut self.tx_buf);
        let fpga_versions_minor =
            match self
                .link
                .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))
            {
                Ok(_) => self
                    .rx_buf
                    .messages()
                    .iter()
                    .map(|rx| rx.ack)
                    .collect::<Vec<_>>(),
                _ => vec![0x00; self.geometry.num_devices()],
            };

        let mut op = autd3_core::CPUVersionMinor::default();
        op.pack(&mut self.tx_buf);
        let cpu_versions_minor =
            match self
                .link
                .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))
            {
                Ok(_) => self
                    .rx_buf
                    .messages()
                    .iter()
                    .map(|rx| rx.ack)
                    .collect::<Vec<_>>(),
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

    pub fn fpga_info(&mut self) -> Result<Vec<u8>, AUTDError> {
        self.link.receive(&mut self.rx_buf)?;
        Ok(self.rx_buf.messages().iter().map(|m| m.ack).collect())
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
        autd3_device::AUTD3, geometry::Vector3, silencer_config::SilencerConfig,
        synchronize::Synchronize,
    };

    use crate::prelude::{Focus, Sine};

    use super::*;

    struct EmulatorLink {
        is_open: bool,
    }

    impl EmulatorLink {
        pub fn new() -> Self {
            Self { is_open: false }
        }
    }

    impl<T: Transducer> Link<T> for EmulatorLink {
        fn open(
            &mut self,
            _geometry: &Geometry<T>,
        ) -> Result<(), autd3_core::error::AUTDInternalError> {
            self.is_open = true;
            Ok(())
        }

        fn close(&mut self) -> Result<(), autd3_core::error::AUTDInternalError> {
            self.is_open = false;
            Ok(())
        }

        fn send(&mut self, _tx: &TxDatagram) -> Result<bool, autd3_core::error::AUTDInternalError> {
            Ok(true)
        }

        fn receive(
            &mut self,
            _rx: &mut RxDatagram,
        ) -> Result<bool, autd3_core::error::AUTDInternalError> {
            Ok(true)
        }

        fn is_open(&self) -> bool {
            self.is_open
        }

        fn timeout(&self) -> Duration {
            std::time::Duration::ZERO
        }
    }

    #[test]
    fn basic_usage() {
        let geometry = Geometry::builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .build()
            .unwrap();

        let link = EmulatorLink::new();

        let mut autd = Controller::open(geometry, link).unwrap();

        let _firm_infos = autd.firmware_infos().unwrap();

        autd.send(Clear::new()).unwrap();
        autd.send(Synchronize::new()).unwrap();

        let silencer = SilencerConfig::default();
        autd.send(silencer).unwrap();

        let m = Sine::new(150);
        let g = Focus::new(autd.geometry().center() + Vector3::new(0.0, 0.0, 150.0));

        autd.send((m, g)).unwrap();
    }
}
