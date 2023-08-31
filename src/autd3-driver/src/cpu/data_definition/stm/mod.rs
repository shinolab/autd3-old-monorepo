/*
 * File: mod.rs
 * Project: stm
 * Created Date: 30/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod focus;
mod gain;

use std::fmt;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct STMControlFlags : u8 {
        const NONE            = 0;
        const USE_START_IDX   = 1 << 1;
        const USE_FINISH_IDX  = 1 << 2;
    }
}

impl fmt::Display for STMControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(STMControlFlags::USE_START_IDX) {
            flags.push("USE_START_IDX")
        }
        if self.contains(STMControlFlags::USE_FINISH_IDX) {
            flags.push("USE_FINISH_IDX")
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
        assert_eq!(size_of::<STMControlFlags>(), 1);

        let mut f = STMControlFlags::NONE;
        assert_eq!(format!("{}", f), "NONE");
    }
}
