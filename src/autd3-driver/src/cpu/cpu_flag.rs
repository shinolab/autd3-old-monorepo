/*
 * File: cpu_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct CPUControlFlags : u8 {
        const NONE            = 0;
        const MOD             = 1 << 0;
        const MOD_BEGIN       = 1 << 1;
        const MOD_END         = 1 << 2;
        const CONFIG_EN_N     = 1 << 0;
        const CONFIG_SILENCER = 1 << 1;
        const CONFIG_SYNC     = 1 << 2;
        const WRITE_BODY      = 1 << 3;
        const STM_BEGIN       = 1 << 4;
        const STM_END         = 1 << 5;
        const IS_DUTY         = 1 << 6;
        const MOD_DELAY       = 1 << 7;
    }
}

impl fmt::Display for CPUControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(CPUControlFlags::MOD) {
            if self.contains(CPUControlFlags::MOD_BEGIN) {
                flags.push("MOD_BEGIN")
            }
            if self.contains(CPUControlFlags::MOD_END) {
                flags.push("MOD_END")
            }
        } else {
            if self.contains(CPUControlFlags::CONFIG_SILENCER) {
                flags.push("CONFIG_SILENCER")
            }
            if self.contains(CPUControlFlags::CONFIG_SYNC) {
                flags.push("CONFIG_SYNC")
            }
        };
        if self.contains(CPUControlFlags::WRITE_BODY) {
            flags.push("WRITE_BODY")
        }
        if self.contains(CPUControlFlags::STM_BEGIN) {
            flags.push("STM_BEGIN")
        }
        if self.contains(CPUControlFlags::STM_END) {
            flags.push("STM_END")
        }
        if self.contains(CPUControlFlags::IS_DUTY) {
            flags.push("IS_DUTY")
        }
        if self.contains(CPUControlFlags::MOD_DELAY) {
            flags.push("MOD_DELAY")
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
        assert_eq!(size_of::<CPUControlFlags>(), 1);
    }
}
