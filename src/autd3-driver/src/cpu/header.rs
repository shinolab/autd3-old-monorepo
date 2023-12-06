/*
 * File: header.rs
 * Project: cpu
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

#[repr(C)]
pub struct Header {
    pub msg_id: u8,
    pub _fpga_flag: u8, // only used before v4.1.0
    pub slot_2_offset: u16,
}
