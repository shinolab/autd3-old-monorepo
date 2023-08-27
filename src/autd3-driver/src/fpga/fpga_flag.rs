/*
 * File: fpga_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt;

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

impl fmt::Display for FPGAControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(FPGAControlFlags::LEGACY_MODE) {
            flags.push("LEGACY_MODE")
        }
        if self.contains(FPGAControlFlags::USE_FINISH_IDX) {
            flags.push("USE_FINISH_IDX")
        }
        if self.contains(FPGAControlFlags::USE_START_IDX) {
            flags.push("USE_START_IDX")
        }
        if self.contains(FPGAControlFlags::FORCE_FAN) {
            flags.push("FORCE_FAN")
        }
        if self.contains(FPGAControlFlags::STM_MODE) {
            flags.push("STM_MODE")
        }
        if self.contains(FPGAControlFlags::STM_GAIN_MODE) {
            flags.push("STM_GAIN_MODE")
        }
        if self.contains(FPGAControlFlags::READS_FPGA_INFO) {
            flags.push("READS_FPGA_INFO")
        }
        if self.is_empty() {
            flags.push("NONE")
        }
        write!(
            f,
            "{}",
            flags
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" | ")
        )
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
