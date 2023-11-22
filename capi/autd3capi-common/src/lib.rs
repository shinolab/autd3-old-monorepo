/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod custom;
mod dynamic_datagram;
mod dynamic_link;

pub use autd3;
pub use autd3_derive as derive;
pub use autd3_driver as driver;
pub use autd3_gain_holo as holo;
use driver::link::LinkSync;
pub use libc;

pub use autd3::{controller::Controller, error::AUTDError};
pub use autd3_driver::{
    datagram::{Datagram, Gain, GainAsAny, GainFilter, Modulation, STMProps},
    defined::float,
    error::AUTDInternalError,
    firmware_version::FirmwareInfo,
    geometry::{Device, Geometry},
};

pub use custom::{CustomGain, CustomModulation};
pub use dynamic_datagram::{DynamicDatagram, DynamicDatagramPack, DynamicDatagramPack2};
pub use dynamic_link::DynamicLinkBuilder;

pub use libc::c_void;

pub type ConstPtr = *const c_void;
pub type L = dyn LinkSync;
pub type G = dyn Gain;
pub type M = dyn Modulation;
pub type Cnt = Controller<Box<L>>;

pub const NULL: ConstPtr = std::ptr::null();

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
