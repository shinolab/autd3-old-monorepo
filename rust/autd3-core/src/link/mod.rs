/*
 * File: link.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod builder;
mod log;
mod logger;

pub use builder::LinkBuilder;
pub use log::Log;
pub use logger::get_logger;
pub use logger::get_logger_with_custom_func;

use std::time::Duration;

use crate::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

use autd3_driver::{RxDatagram, TxDatagram};

/// Link is a interface to the AUTD device.
pub trait Link: Send {
    fn open<T: Transducer>(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError>;
    fn close(&mut self) -> Result<(), AUTDInternalError>;
    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError>;
    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError>;
    fn is_open(&self) -> bool;
    fn timeout(&self) -> Duration;

    fn send_receive(
        &mut self,
        tx: &TxDatagram,
        rx: &mut RxDatagram,
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        if !self.send(tx)? {
            return Ok(false);
        }
        if timeout.is_zero() {
            return self.receive(rx);
        }
        self.wait_msg_processed(tx.header().msg_id, rx, timeout)
    }

    fn wait_msg_processed(
        &mut self,
        msg_id: u8,
        rx: &mut RxDatagram,
        timeout: Duration,
    ) -> Result<bool, AUTDInternalError> {
        let start = std::time::Instant::now();
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1));
            if self.receive(rx)? && rx.is_msg_processed(msg_id) {
                return Ok(true);
            }
            if start.elapsed() > timeout {
                return Ok(false);
            }
        }
    }
}
