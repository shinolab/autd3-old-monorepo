/*
 * File: mod.rs
 * Project: link
 * Created Date: 09/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 03/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod debug;
pub mod log;

use autd3_core::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
    link::Link,
    RxDatagram, TxDatagram,
};
pub use debug::Debug;
pub use log::Log;

pub struct NullLink {}

impl<T: Transducer> Link<T> for NullLink {
    fn open(&mut self, _geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        Ok(())
    }

    fn send(&mut self, _tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        Ok(true)
    }

    fn receive(&mut self, _rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        Ok(true)
    }

    fn is_open(&self) -> bool {
        true
    }

    fn timeout(&self) -> std::time::Duration {
        std::time::Duration::ZERO
    }
}
