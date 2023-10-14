/*
 * File: gain_control_flags.rs
 * Project: gain
 * Created Date: 08/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::fmt;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct GainControlFlags : u8 {
        const NONE    = 0;
        const LEGACY  = 1 << 0;
        const DUTY    = 1 << 1;
    }
}

impl fmt::Display for GainControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(GainControlFlags::LEGACY) {
            flags.push("LEGACY")
        }
        if self.contains(GainControlFlags::DUTY) {
            flags.push("DUTY")
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
    use super::*;

    use std::mem::size_of;

    #[test]
    fn gain_control_flags() {
        assert_eq!(size_of::<GainControlFlags>(), 1);
        let flags = GainControlFlags::LEGACY;

        let flagsc = Clone::clone(&flags);

        assert!(flagsc.contains(GainControlFlags::LEGACY));
        assert!(!flagsc.contains(GainControlFlags::DUTY));
    }

    #[test]
    fn gain_control_flags_fmt() {
        assert_eq!(format!("{}", GainControlFlags::NONE), "NONE");
        assert_eq!(format!("{}", GainControlFlags::LEGACY), "LEGACY");
        assert_eq!(format!("{}", GainControlFlags::DUTY), "DUTY");
        assert_eq!(
            format!("{}", GainControlFlags::LEGACY | GainControlFlags::DUTY),
            "LEGACY | DUTY"
        );
    }
}
