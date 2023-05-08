/*
 * File: fpga_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct FPGAControlFlags : u8 {
        const NONE            = 0;
        const LEGACY_MODE     = 1 << 0;
        const USE_FINISH_IDX  = 1 << 2;
        const USE_START_IDX   = 1 << 3;
        const FORCE_FAN       = 1 << 4;
        const STM_MODE        = 1 << 5;
        const STM_GAIN_MODE   = 1 << 6;
        const READS_FPGA_INFO = 1 << 7;
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn fpga_info() {
        assert_eq!(size_of::<FPGAControlFlags>(), 1);
    }
}
