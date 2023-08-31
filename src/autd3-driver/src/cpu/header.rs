/*
 * File: header.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::fpga::FPGAControlFlags;

pub const MSG_CLEAR: u8 = 0x00;
pub const MSG_RD_CPU_VERSION: u8 = 0x01;
pub const MSG_RD_CPU_VERSION_MINOR: u8 = 0x02;
pub const MSG_RD_FPGA_VERSION: u8 = 0x03;
pub const MSG_RD_FPGA_VERSION_MINOR: u8 = 0x04;
pub const MSG_RD_FPGA_FUNCTION: u8 = 0x05;
pub const MSG_BEGIN: u8 = 0x10;
pub const MSG_END: u8 = 0xF0;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Header {
    pub msg_id: u8,
    pub fpga_flag: FPGAControlFlags,
    pub slot_2_offset: u16,
}
