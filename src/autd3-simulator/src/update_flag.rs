/*
 * File: update_flag.rs
 * Project: src
 * Created Date: 26/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

bitflags::bitflags! {
    pub struct UpdateFlag: u32 {
        const UPDATE_SOURCE_DRIVE = 1 << 0;
        const UPDATE_COLOR_MAP = 1 << 1;
        const UPDATE_SLICE_POS = 1 << 3;
        const UPDATE_SLICE_SIZE = 1 << 4;
        const UPDATE_SOURCE_ALPHA = 1 << 5;
        const UPDATE_SOURCE_FLAG = 1 << 6;
        const SAVE_IMAGE = 1 << 7;
        const UPDATE_DEVICE_INFO = 1 << 8;
    }
}
