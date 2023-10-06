/*
 * File: lib.rs
 * Project: src
 * Created Date: 29/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![cfg_attr(coverage_nightly, feature(no_coverage))]

pub mod acoustics;
pub mod cpu;
pub mod datagram;
pub mod defined;
pub mod error;
pub mod firmware_version;
pub mod fpga;
pub mod geometry;
pub mod link;
pub mod operation;
pub mod osal_timer;
pub mod sync_mode;
pub mod timer_strategy;

pub mod derive {
    pub mod prelude {
        pub use crate::{
            datagram::{Datagram, Gain, GainAsAny, GainFilter, Modulation, ModulationProperty},
            defined::{float, Drive},
            error::AUTDInternalError,
            fpga::{FPGA_SUB_CLK_FREQ, SAMPLING_FREQ_DIV_MIN},
            geometry::{Geometry, Transducer},
            operation::{GainOp, ModulationOp, NullOp, Operation},
        };
    }
}
