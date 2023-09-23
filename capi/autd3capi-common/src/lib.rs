/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod custom;
mod dynamic_datagram;
mod dynamic_op;
mod dynamic_transducer;

pub use autd3;
pub use autd3_derive as derive;
pub use autd3_driver as driver;
pub use autd3_gain_holo as holo;
pub use libc;

pub use autd3::{controller::ControllerBuilder, link::Debug, prelude::*};
pub use autd3_driver::{
    datagram::{Datagram, Gain, GainAsAny, GainFilter, Modulation, STMProps},
    error::AUTDInternalError,
    firmware_version::FirmwareInfo,
    geometry::{Device, Geometry},
    logger::get_logger_with_custom_func,
    spdlog,
};

pub use custom::{CustomGain, CustomModulation};
pub use dynamic_datagram::DynamicDatagram;
pub use dynamic_transducer::{DynamicTransducer, TransMode};

pub use libc::c_void;

pub type ConstPtr = *const c_void;
pub type Cnt = Controller<DynamicTransducer, Box<dyn Link<DynamicTransducer>>>;
pub type Dev = Device<DynamicTransducer>;
pub type Tr = DynamicTransducer;
pub type Geo = Geometry<DynamicTransducer>;
pub type L = dyn Link<DynamicTransducer>;
pub type G = dyn Gain<DynamicTransducer>;
pub type M = dyn Modulation;

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
macro_rules! cast {
    ($ptr:expr, $type:ty) => {
        ($ptr as *const $type).as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! cast_mut {
    ($ptr:expr, $type:ty) => {
        ($ptr as *mut $type).as_mut().unwrap()
    };
}
