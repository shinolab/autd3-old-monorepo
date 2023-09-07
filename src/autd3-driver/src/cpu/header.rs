/*
 * File: header.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::fpga::FPGAControlFlags;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Header {
    pub msg_id: u8,
    pub fpga_flag: FPGAControlFlags,
    pub slot_2_offset: u16,
}
