/*
 * File: link.rs
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

use std::time::Duration;

use crate::{
    cpu::{RxMessage, TxDatagram},
    error::AUTDInternalError,
    geometry::{Device, Transducer},
};

/// Link is a interface to the AUTD device
pub trait Link<T: Transducer>: Send {
    /// Open link
    fn open(&mut self, devices: &[Device<T>]) -> Result<(), AUTDInternalError>;
    /// Close link
    fn close(&mut self) -> Result<(), AUTDInternalError>;
    /// Send data to devices
    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError>;
    /// Receive data from devices
    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError>;
    /// Check if link is open
    fn is_open(&self) -> bool;
    /// Get timeout
    fn timeout(&self) -> Duration;
    /// Send and receive data
    fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        if !self.send(tx)? {
            return Ok(false);
        }
        if timeout.is_zero() {
            return self.receive(rx);
        }
        self.wait_msg_processed(tx, rx, timeout)
    }

    fn update_geometry(&mut self, _: &[Device<T>]) -> Result<(), AUTDInternalError> {
        Ok(())
    }

    /// Wait until message is processed
    fn wait_msg_processed(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        let start = std::time::Instant::now();
        let _ = self.receive(rx)?;
        if tx.headers().zip(rx.iter()).all(|(h, r)| h.msg_id == r.ack) {
            return Ok(true);
        }
        loop {
            if start.elapsed() > timeout {
                return Ok(false);
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
            if !self.receive(rx)? {
                continue;
            }
            if tx.headers().zip(rx.iter()).all(|(h, r)| h.msg_id == r.ack) {
                return Ok(true);
            }
        }
    }

    fn check(&self, tx: &TxDatagram, rx: &mut [RxMessage]) -> Vec<bool> {
        tx.headers()
            .zip(rx.iter())
            .map(|(h, r)| h.msg_id == r.ack)
            .collect()
    }
}

impl<T: Transducer> Link<T> for Box<dyn Link<T>> {
    fn open(&mut self, devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
        self.as_mut().open(devices)
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        self.as_mut().close()
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        self.as_mut().send(tx)
    }

    fn receive(&mut self, rx: &mut [RxMessage]) -> Result<bool, AUTDInternalError> {
        self.as_mut().receive(rx)
    }

    fn is_open(&self) -> bool {
        self.as_ref().is_open()
    }

    fn timeout(&self) -> Duration {
        self.as_ref().timeout()
    }

    fn update_geometry(&mut self, devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
        self.as_mut().update_geometry(devices)
    }

    fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        self.as_mut().send_receive(tx, rx, timeout)
    }

    fn wait_msg_processed(
        &mut self,
        tx: &TxDatagram,
        rx: &mut [RxMessage],
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        self.as_mut().wait_msg_processed(tx, rx, timeout)
    }

    fn check(&self, tx: &TxDatagram, rx: &mut [RxMessage]) -> Vec<bool> {
        self.as_ref().check(tx, rx)
    }
}
