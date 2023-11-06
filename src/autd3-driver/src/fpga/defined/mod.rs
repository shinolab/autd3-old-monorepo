/*
 * File: fpga_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::defined::{float, METER};

mod drive;
mod filter;
mod fpga_info;
mod stm_focus;

pub use drive::FPGADrive;
pub use filter::{FilterDuty, FilterPhase};
pub use fpga_info::FPGAInfo;
pub use stm_focus::STMFocus;

/// FPGA clock frequency
pub const FPGA_CLK_FREQ: usize = 20480000;

pub const FOCUS_STM_FIXED_NUM_UNIT: float = 0.025e-3 * METER;
pub const FOCUS_STM_FIXED_NUM_WIDTH: usize = 18;
pub const FOCUS_STM_FIXED_NUM_UPPER: i32 = (1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1)) - 1;
pub const FOCUS_STM_FIXED_NUM_LOWER: i32 = -(1 << (FOCUS_STM_FIXED_NUM_WIDTH - 1));

pub const MIN_CYCLE: u16 = 2;
pub const MAX_CYCLE: u16 = 8191;

pub const SAMPLING_FREQ_DIV_MIN: u32 = 512;
pub const SAMPLING_FREQ_DIV_MAX: u32 = u32::MAX;

pub const MOD_BUF_SIZE_MIN: usize = 2;
pub const MOD_BUF_SIZE_MAX: usize = 65536;

pub const STM_BUF_SIZE_MIN: usize = 2;
pub const FOCUS_STM_BUF_SIZE_MAX: usize = 65536;
pub const GAIN_STM_BUF_SIZE_MAX: usize = 2048;
