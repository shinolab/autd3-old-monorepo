/*
 * File: controller.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, hash::Hash, time::Duration};

use autd3_driver::{
    cpu::{RxMessage, TxDatagram},
    datagram::{Clear, Datagram, Stop, Synchronize, UpdateFlags},
    error::AUTDInternalError,
    firmware_version::FirmwareInfo,
    fpga::FPGAInfo,
    geometry::{
        AdvancedPhaseTransducer, AdvancedTransducer, Device, Geometry, IntoDevice,
        LegacyTransducer, Transducer,
    },
    link::Link,
    operation::{Operation, OperationHandler},
};

use crate::{
    error::{AUTDError, ReadFirmwareInfoState},
    link::NullLink,
    software_stm::SoftwareSTM,
};

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
    fn new() -> ControllerBuilder<T> {
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

    fn convert<T2: Transducer>(self) -> ControllerBuilder<T2> {
        ControllerBuilder {
            devices: self
                .devices
                .iter()
                .map(|dev| {
                    Device::new(
                        dev.idx(),
                        dev.iter()
                            .map(|tr| T2::new(tr.local_idx(), *tr.position(), *tr.rotation()))
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

impl ControllerBuilder<LegacyTransducer> {
    pub fn advanced(self) -> ControllerBuilder<AdvancedTransducer> {
        self.convert()
    }

    pub fn advanced_phase(self) -> ControllerBuilder<AdvancedPhaseTransducer> {
        self.convert()
    }
}

impl ControllerBuilder<AdvancedTransducer> {
    pub fn legacy(self) -> ControllerBuilder<LegacyTransducer> {
        self.convert()
    }

    pub fn advanced_phase(self) -> ControllerBuilder<AdvancedPhaseTransducer> {
        self.convert()
    }
}

impl ControllerBuilder<AdvancedPhaseTransducer> {
    pub fn advanced(self) -> ControllerBuilder<AdvancedTransducer> {
        self.convert()
    }

    pub fn legacy(self) -> ControllerBuilder<LegacyTransducer> {
        self.convert()
    }
}

/// Controller for AUTD
pub struct Controller<T: Transducer, L: Link<T>> {
    link: L,
    geometry: Geometry<T>,
    tx_buf: TxDatagram,
    rx_buf: Vec<RxMessage>,
}

impl Controller<LegacyTransducer, NullLink> {
    /// Create Controller builder
    pub fn builder() -> ControllerBuilder<LegacyTransducer> {
        ControllerBuilder::<LegacyTransducer>::new()
    }
}

#[allow(clippy::type_complexity)]
pub struct GroupGuard<
    'a,
    K: Hash + Eq + Clone,
    T: Transducer,
    L: Link<T>,
    F: Fn(&Device<T>) -> Option<K>,
> {
    cnt: &'a mut Controller<T, L>,
    f: F,
    timeout: Option<Duration>,
    op: HashMap<K, (Box<dyn Operation<T>>, Box<dyn Operation<T>>)>,
}

impl<'a, K: Hash + Eq + Clone, T: Transducer, L: Link<T>, F: Fn(&Device<T>) -> Option<K>>
    GroupGuard<'a, K, T, L, F>
{
    pub fn set<D: Datagram<T>>(mut self, k: K, d: D) -> Result<Self, AUTDInternalError>
    where
        D::O1: 'static,
        D::O2: 'static,
    {
        self.timeout = match (self.timeout, d.timeout()) {
            (None, None) => None,
            (None, Some(t)) => Some(t),
            (Some(t), None) => Some(t),
            (Some(t1), Some(t2)) => Some(t1.max(t2)),
        };
        let (op1, op2) = d.operation()?;
        self.op.insert(k, (Box::new(op1), Box::new(op2)));
        Ok(self)
    }

    #[doc(hidden)]
    pub fn set_boxed_op(
        mut self,
        k: K,
        op1: Box<dyn autd3_driver::operation::Operation<T>>,
        op2: Box<dyn autd3_driver::operation::Operation<T>>,
        timeout: Option<Duration>,
    ) -> Result<Self, AUTDInternalError> {
        self.timeout = match (self.timeout, timeout) {
            (None, None) => None,
            (None, Some(t)) => Some(t),
            (Some(t), None) => Some(t),
            (Some(t1), Some(t2)) => Some(t1.max(t2)),
        };
        self.op.insert(k, (op1, op2));
        Ok(self)
    }

    pub fn send(mut self) -> Result<bool, AUTDInternalError> {
        let timeout = self.timeout.unwrap_or(self.cnt.link.timeout());

        let enable_flags_store = self
            .cnt
            .geometry
            .iter()
            .map(|dev| dev.enable)
            .collect::<Vec<_>>();

        let enable_flags_map: HashMap<K, Vec<bool>> = self
            .op
            .keys()
            .map(|k| {
                (
                    k.clone(),
                    self.cnt
                        .geometry
                        .iter()
                        .map(|dev| {
                            if !dev.enable {
                                return false;
                            }
                            if let Some(kk) = (self.f)(dev) {
                                kk == *k
                            } else {
                                false
                            }
                        })
                        .collect(),
                )
            })
            .collect();

        self.op.iter_mut().try_for_each(|(k, (op1, op2))| {
            self.cnt.geometry_mut().iter_mut().for_each(|dev| {
                dev.enable = enable_flags_map[k][dev.idx()];
            });
            OperationHandler::init(op1, op2, &self.cnt.geometry)
        })?;

        let r = loop {
            self.op.iter_mut().try_for_each(|(k, (op1, op2))| {
                self.cnt.geometry_mut().iter_mut().for_each(|dev| {
                    dev.enable = enable_flags_map[k][dev.idx()];
                });
                OperationHandler::pack(op1, op2, &self.cnt.geometry, &mut self.cnt.tx_buf)
            })?;

            if !self
                .cnt
                .link
                .send_receive(&self.cnt.tx_buf, &mut self.cnt.rx_buf, timeout)?
            {
                break false;
            }
            if self.op.iter_mut().all(|(k, (op1, op2))| {
                self.cnt.geometry_mut().iter_mut().for_each(|dev| {
                    dev.enable = enable_flags_map[k][dev.idx()];
                });
                OperationHandler::is_finished(op1, op2, &self.cnt.geometry)
            }) {
                break true;
            }
            if timeout.is_zero() {
                std::thread::sleep(Duration::from_millis(1));
            }
        };

        self.cnt
            .geometry
            .iter_mut()
            .zip(enable_flags_store.iter())
            .for_each(|(dev, &enable)| dev.enable = enable);

        Ok(r)
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

    pub fn notify_link_geometry_updated(&mut self) -> Result<(), AUTDError> {
        self.link.update_geometry(&self.geometry)?;
        Ok(())
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
        let timeout = s.timeout().unwrap_or(self.link.timeout());

        let (mut op1, mut op2) = s.operation()?;
        OperationHandler::init(&mut op1, &mut op2, &self.geometry)?;
        loop {
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
            if timeout.is_zero() {
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
        if !self
            .link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?
        {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                self.link.check(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let cpu_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self
            .link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?
        {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                self.link.check(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let cpu_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self
            .link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?
        {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                self.link.check(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let fpga_versions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self
            .link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?
        {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                self.link.check(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let fpga_versions_minor = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
        if !self
            .link
            .send_receive(&self.tx_buf, &mut self.rx_buf, Duration::from_millis(200))?
        {
            return Err(AUTDError::ReadFirmwareInfoFailed(ReadFirmwareInfoState(
                self.link.check(&self.tx_buf, &mut self.rx_buf),
            )));
        }
        let fpga_functions = self.rx_buf.iter().map(|rx| rx.data).collect::<Vec<_>>();

        OperationHandler::pack(&mut op, &mut null_op, &self.geometry, &mut self.tx_buf)?;
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
            .open_with(Audit::new())
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
