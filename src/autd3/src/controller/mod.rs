/*
 * File: mod.rs
 * Project: controller
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod builder;
mod group;

use std::{collections::HashMap, hash::Hash, time::Duration};

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    datagram::{Clear, Datagram, Stop, Synchronize, UpdateFlags},
    firmware_version::FirmwareInfo,
    fpga::FPGAInfo,
    geometry::{Device, Geometry, LegacyTransducer, Transducer},
    link::Link,
    operation::OperationHandler,
};

use crate::{
    error::{AUTDError, ReadFirmwareInfoState},
    link::Nop,
    software_stm::SoftwareSTM,
};

use builder::ControllerBuilder;
use group::GroupGuard;

/// Controller for AUTD
pub struct Controller<T: Transducer, L: Link> {
    link: L,
    geometry: Geometry<T>,
    tx_buf: TxDatagram,
    rx_buf: Vec<RxMessage>,
}

impl Controller<LegacyTransducer, Nop> {
    /// Create Controller builder
    pub fn builder() -> ControllerBuilder<LegacyTransducer> {
        ControllerBuilder::<LegacyTransducer>::new()
    }

    /// Create Controller builder
    pub fn builder_with<T: Transducer>() -> ControllerBuilder<T> {
        ControllerBuilder::<T>::new()
    }
}

impl<T: Transducer, L: Link> Controller<T, L> {
    #[doc(hidden)]
    pub fn open_impl(geometry: Geometry<T>, link: L) -> Result<Controller<T, L>, AUTDError> {
        let num_devices = geometry.num_devices();
        let tx_buf = TxDatagram::new(num_devices);
        let mut cnt = Controller {
            link,
            geometry,
            tx_buf,
            rx_buf: vec![RxMessage { data: 0, ack: 0 }; num_devices],
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
    pub fn link_mut(&mut self) -> &mut L {
        &mut self.link
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
    /// * `Ok(false)` - There are no errors, but it is unclear whether the data has been sent or not
    ///
    pub fn send<S: Datagram<T>>(&mut self, s: S) -> Result<bool, AUTDError> {
        let timeout = s.timeout();

        let (mut op1, mut op2) = s.operation()?;
        OperationHandler::init(&mut op1, &mut op2, &self.geometry)?;
        loop {
            let start = std::time::Instant::now();
            OperationHandler::pack(&mut op1, &mut op2, &self.geometry, &mut self.tx_buf)?;

            if !self
                .link
                .send_receive(&self.tx_buf, &mut self.rx_buf, timeout)?
            {
                return Ok(false);
            }
            if OperationHandler::is_finished(&mut op1, &mut op2, &self.geometry) {
                break;
            }
            if start.elapsed() < std::time::Duration::from_millis(1) {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
        Ok(true)
    }

    #[must_use]
    pub fn group<K: Hash + Eq + Clone, F: Fn(&Device<T>) -> Option<K>>(
        &mut self,
        f: F,
    ) -> GroupGuard<K, T, L, F> {
        GroupGuard {
            cnt: self,
            f,
            timeout: None,
            op: HashMap::new(),
        }
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
        for dev in self.geometry_mut() {
            dev.enable = true;
        }
        let res = self.send(Stop::new())?;
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

        OperationHandler::init(&mut op, &mut null_op, &self.geometry)?;

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self.link.send_receive(
            &self.tx_buf,
            &mut self.rx_buf,
            Some(Duration::from_millis(200)),
        )? {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                autd3_driver::cpu::check_if_msg_is_processed(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let cpu_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self.link.send_receive(
            &self.tx_buf,
            &mut self.rx_buf,
            Some(Duration::from_millis(200)),
        )? {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                autd3_driver::cpu::check_if_msg_is_processed(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let cpu_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self.link.send_receive(
            &self.tx_buf,
            &mut self.rx_buf,
            Some(Duration::from_millis(200)),
        )? {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                autd3_driver::cpu::check_if_msg_is_processed(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let fpga_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self.link.send_receive(
            &self.tx_buf,
            &mut self.rx_buf,
            Some(Duration::from_millis(200)),
        )? {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                autd3_driver::cpu::check_if_msg_is_processed(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let fpga_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self.link.send_receive(
            &self.tx_buf,
            &mut self.rx_buf,
            Some(Duration::from_millis(200)),
        )? {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                autd3_driver::cpu::check_if_msg_is_processed(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let fpga_functions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        self.link.send_receive(
            &self.tx_buf,
            &mut self.rx_buf,
            Some(Duration::from_millis(200)),
        )?;

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
    /// * `callback` - Callback function called specified interval. If this callback returns false, the STM finish.
    ///
    pub fn software_stm<
        F: FnMut(&mut Controller<T, L>, usize, std::time::Duration) -> bool + Send + 'static,
    >(
        &mut self,
        callback: F,
    ) -> SoftwareSTM<T, L, F> {
        SoftwareSTM::new(self, callback)
    }
}

#[cfg(test)]
mod tests {
    use crate::{autd3_device::AUTD3, link::Audit};

    use autd3_driver::geometry::Vector3;

    use super::*;

    #[test]
    fn group() {
        let mut autd = Controller::builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Audit::builder())
            .unwrap();

        for dev in autd.geometry_mut() {
            dev.force_fan = true;
        }

        autd.group(|dev| match dev.idx() {
            0 => Some("0"),
            1 => Some("1"),
            _ => None,
        })
        .set("0", UpdateFlags::new())
        .unwrap()
        .send()
        .unwrap();

        assert!(autd.link().emulators()[0].fpga().is_force_fan());
        assert!(!autd.link().emulators()[1].fpga().is_force_fan());
        assert!(!autd.link().emulators()[2].fpga().is_force_fan());
    }
}
