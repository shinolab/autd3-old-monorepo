/*
 * File: ec_config.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

pub const EC_OUTPUT_FRAME_SIZE: usize = 626;
pub const EC_INPUT_FRAME_SIZE: usize = 2;

pub const EC_CYCLE_TIME_BASE_MICRO_SEC: u64 = 500;
pub const EC_CYCLE_TIME_BASE_NANO_SEC: u64 = EC_CYCLE_TIME_BASE_MICRO_SEC * 1000;
