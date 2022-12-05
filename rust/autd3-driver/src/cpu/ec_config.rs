/*
 * File: ec_config.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

pub const HEADER_SIZE: usize = 128;
pub const EC_INPUT_FRAME_SIZE: usize = 2;

pub const EC_CYCLE_TIME_BASE_MICRO_SEC: u32 = 500;
pub const EC_CYCLE_TIME_BASE_NANO_SEC: u32 = EC_CYCLE_TIME_BASE_MICRO_SEC * 1000;
