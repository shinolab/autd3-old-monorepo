/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
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
mod dynamic_transducer;

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
pub use dynamic_transducer::TransMode;

pub use libc::c_void;

pub type ConstPtr = *const c_void;
pub type Cnt = Controller<DynamicTransducer, DynamicLink>;
pub type L = dyn Link<DynamicTransducer>;
pub type G = dyn DynamicGain;
pub type M = dyn DynamicModulation;
pub type SF = dyn DynamicFocusSTM;
pub type SG = dyn DynamicGainSTM;

pub const OK: i32 = 1;
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;
pub const ERR: i32 = -1;

pub fn to_level(level: u16) -> Option<Level> {
    match level {
        0 => Some(Level::Critical),
        1 => Some(Level::Error),
        2 => Some(Level::Warn),
        3 => Some(Level::Info),
        4 => Some(Level::Debug),
        5 => Some(Level::Trace),
        _ => None,
    }
}

pub fn to_mode(level: u8) -> Option<TransMode> {
    match level {
        0 => Some(TransMode::Legacy),
        1 => Some(TransMode::Advanced),
        2 => Some(TransMode::AdvancedPhase),
        _ => None,
    }
}

#[macro_export]
macro_rules! try_or_return {
    ($expr:expr, $err:ident) => {
        match $expr {
            Ok(v) => v,
            Err(e) => {
                let msg = std::ffi::CString::new(e.to_string()).unwrap();
                libc::strcpy($err, msg.as_ptr());
                return ERR;
            }
        }
    };
}
