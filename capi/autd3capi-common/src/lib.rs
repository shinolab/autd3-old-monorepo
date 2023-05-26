/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod dynamic_datagram;
mod dynamic_gain;
mod dynamic_link;
mod dynamic_modulation;
mod dynamic_stm;
pub mod dynamic_transducer;

pub use autd3::{
    core::{
        datagram::Datagram, firmware_version::FirmwareInfo, gain::Gain,
        link::get_logger_with_custom_func, modulation::Modulation, spdlog,
    },
    link::debug::{Debug, DebugBuilder},
    prelude::*,
};

pub use dynamic_datagram::DynamicDatagram;
pub use dynamic_gain::*;
pub use dynamic_link::DynamicLink;
pub use dynamic_modulation::*;
pub use dynamic_stm::*;
pub use dynamic_transducer::DynamicTransducer;

pub use libc::c_void;

pub type ConstPtr = *const c_void;
pub type Cnt = Controller<DynamicTransducer, DynamicLink>;
pub type L = dyn Link<DynamicTransducer>;
pub type G = dyn DynamicGain;
pub type M = dyn DynamicModulation;
pub type SF = dyn DynamicFocusSTM;
pub type SG = dyn DynamicGainSTM;

pub const NULL: ConstPtr = std::ptr::null();

#[macro_export]
macro_rules! try_or_return {
    ($expr:expr, $err:ident, $ret:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                let msg = std::ffi::CString::new(e.to_string()).unwrap();
                libc::strcpy($err, msg.as_ptr());
                return $ret;
            }
        }
    };
}

#[macro_export]
macro_rules! cast_without_ownership {
    ($ptr:expr, $type:ty) => {
        ($ptr as *const $type).as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! cast_without_ownership_mut {
    ($ptr:expr, $type:ty) => {
        ($ptr as *mut $type).as_mut().unwrap()
    };
}

pub struct NullLink {}

impl Link<DynamicTransducer> for NullLink {
    fn open(
        &mut self,
        _geometry: &Geometry<DynamicTransducer>,
    ) -> Result<(), autd3::core::error::AUTDInternalError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), autd3::core::error::AUTDInternalError> {
        Ok(())
    }

    fn send(
        &mut self,
        _tx: &autd3::core::TxDatagram,
    ) -> Result<bool, autd3::core::error::AUTDInternalError> {
        Ok(true)
    }

    fn receive(
        &mut self,
        _rx: &mut autd3::core::RxDatagram,
    ) -> Result<bool, autd3::core::error::AUTDInternalError> {
        Ok(true)
    }

    fn is_open(&self) -> bool {
        true
    }

    fn timeout(&self) -> std::time::Duration {
        std::time::Duration::ZERO
    }
}
