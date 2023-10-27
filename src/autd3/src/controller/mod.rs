/*
 * File: mod.rs
 * Project: controller
 * Created Date: 05/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/10/2023
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
    operation::OperationHandler,
};

use crate::error::{AUTDError, ReadFirmwareInfoState};

use builder::ControllerBuilder;
use group::GroupGuard;

#[cfg(not(feature = "sync"))]
use crate::link::nop::Nop;
#[cfg(not(feature = "sync"))]
use autd3_driver::link::Link;

#[cfg(feature = "sync")]
use crate::link::nop::NopSync as Nop;
#[cfg(feature = "sync")]
use autd3_driver::link::LinkSync as Link;

/// Controller for AUTD
pub struct Controller<T: Transducer, L: Link> {
    pub link: L,
    pub geometry: Geometry<T>,
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
}

#[cfg(not(feature = "sync"))]
impl<T: Transducer, L: Link> Controller<T, L> {
    #[doc(hidden)]
    pub async fn open_impl(geometry: Geometry<T>, link: L) -> Result<Controller<T, L>, AUTDError> {
        let num_devices = geometry.num_devices();
        let tx_buf = TxDatagram::new(num_devices);
        let mut cnt = Controller {
            link,
            geometry,
            tx_buf,
            rx_buf: vec![RxMessage { data: 0, ack: 0 }; num_devices],
        };
        cnt.send(UpdateFlags::new()).await?;
        cnt.send(Clear::new()).await?;
        cnt.send(Synchronize::new()).await?;
        Ok(cnt)
    }
}

#[cfg(feature = "sync")]
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
}

#[cfg(not(feature = "sync"))]
impl<T: Transducer, L: Link> Controller<T, L> {
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
    pub async fn send<S: Datagram<T>>(&mut self, s: S) -> Result<bool, AUTDError> {
        let timeout = s.timeout();

        let (mut op1, mut op2) = s.operation()?;
        OperationHandler::init(&mut op1, &mut op2, &self.geometry)?;
        loop {
            let start = std::time::Instant::now();
            OperationHandler::pack(&mut op1, &mut op2, &self.geometry, &mut self.tx_buf)?;

            if !self
                .link
                .send_receive(&self.tx_buf, &mut self.rx_buf, timeout)
                .await?
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

    // Close connection
    pub async fn close(&mut self) -> Result<bool, AUTDError> {
        if !self.link.is_open() {
            return Ok(false);
        }
        for dev in self.geometry.iter_mut() {
            dev.enable = true;
        }
        let res = self.send(Stop::new()).await?;
        let res = res & self.send(Clear::new()).await?;
        self.link.close().await?;
        Ok(res)
    }

    /// Get firmware information
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<FirmwareInfo>)` - List of firmware information
    ///
    pub async fn firmware_infos(&mut self) -> Result<Vec<FirmwareInfo>, AUTDError> {
        let mut op = autd3_driver::operation::FirmInfoOp::default();
        let mut null_op = autd3_driver::operation::NullOp::default();

        OperationHandler::init(&mut op, &mut null_op, &self.geometry)?;

        macro_rules! pack_and_send {
            ($op:expr, $null_op:expr, $link:expr, $geometry:expr, $tx_buf:expr, $rx_buf:expr ) => {
                OperationHandler::pack($op, $null_op, $geometry, $tx_buf)?;
                if !$link
                    .send_receive($tx_buf, $rx_buf, Some(Duration::from_millis(200)))
                    .await?
                {
                    return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                        autd3_driver::cpu::check_if_msg_is_processed($tx_buf, $rx_buf),
                    )));
                }
            };
        }

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let cpu_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let cpu_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let fpga_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let fpga_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let fpga_functions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );

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
    pub async fn fpga_info(&mut self) -> Result<Vec<FPGAInfo>, AUTDError> {
        self.link.receive(&mut self.rx_buf).await?;
        Ok(self.rx_buf.iter().map(FPGAInfo::from).collect())
    }
}

#[cfg(feature = "sync")]
impl<T: Transducer, L: Link> Controller<T, L> {
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

    pub fn close(&mut self) -> Result<bool, AUTDError> {
        if !self.link.is_open() {
            return Ok(false);
        }
        for dev in self.geometry.iter_mut() {
            dev.enable = true;
        }
        let res = self.send(Stop::new())?;
        let res = res & self.send(Clear::new())?;
        self.link.close()?;
        Ok(res)
    }

    pub fn firmware_infos(&mut self) -> Result<Vec<FirmwareInfo>, AUTDError> {
        let mut op = autd3_driver::operation::FirmInfoOp::default();
        let mut null_op = autd3_driver::operation::NullOp::default();

        OperationHandler::init(&mut op, &mut null_op, &self.geometry)?;

        macro_rules! pack_and_send {
            ($op:expr, $null_op:expr, $link:expr, $geometry:expr, $tx_buf:expr, $rx_buf:expr ) => {
                OperationHandler::pack($op, $null_op, $geometry, $tx_buf)?;
                if !$link.send_receive($tx_buf, $rx_buf, Some(Duration::from_millis(200)))? {
                    return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                        autd3_driver::cpu::check_if_msg_is_processed($tx_buf, $rx_buf),
                    )));
                }
            };
        }

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let cpu_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let cpu_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let fpga_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let fpga_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );
        let fpga_functions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        pack_and_send!(
            &mut op,
            &mut null_op,
            &mut self.link,
            &self.geometry,
            &mut self.tx_buf,
            &mut self.rx_buf
        );

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
}

#[cfg(test)]
mod tests {
    use crate::link::Audit;

    use autd3_driver::{autd3_device::AUTD3, geometry::Vector3};

    use super::*;

    #[tokio::test]
    async fn group() {
        let mut autd = Controller::builder()
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
            .open_with(Audit::builder())
            .await
            .unwrap();

        for dev in &mut autd.geometry {
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
        .await
        .unwrap();

        assert!(autd.link.emulators()[0].fpga().is_force_fan());
        assert!(!autd.link.emulators()[1].fpga().is_force_fan());
        assert!(!autd.link.emulators()[2].fpga().is_force_fan());
    }
}
