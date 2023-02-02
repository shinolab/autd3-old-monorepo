/*
 * File: defined.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/01/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

pub const VERSION_NUM: u8 = 0x88;

pub const MAX_CYCLE: u16 = 8191;

pub const MOD_SAMPLING_FREQ_DIV_MIN: u32 = 580;
pub const MOD_BUF_SIZE_MAX: usize = 65536;

pub const FOCUS_STM_SAMPLING_FREQ_DIV_MIN: u32 = 806;
pub const GAIN_STM_SAMPLING_FREQ_DIV_MIN: u32 = 138;
pub const GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN: u32 = 76;
pub const FOCUS_STM_BUF_SIZE_MAX: usize = 65536;
pub const GAIN_STM_BUF_SIZE_MAX: usize = 1024;
pub const GAIN_STM_LEGACY_BUF_SIZE_MAX: usize = 2048;

pub const SILENCER_CYCLE_MIN: u16 = 522;
