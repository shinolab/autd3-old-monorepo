/*
 * File: fpga_defined.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
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
        const FORCE_FAN       = 1 << 0;
        const READS_FPGA_INFO = 1 << 1;
    }
}

impl fmt::Display for FPGAControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(FPGAControlFlags::FORCE_FAN) {
            flags.push("FORCE_FAN")
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

    use super::*;

    #[test]
    fn fpga_info() {
        assert_eq!(std::mem::size_of::<FPGAControlFlags>(), 1);

        let flags = FPGAControlFlags::FORCE_FAN;

        let flagsc = flags.clone();
        assert_eq!(flagsc.bits(), flags.bits());
    }

    #[test]
    fn fmt() {
        assert_eq!(format!("{}", FPGAControlFlags::NONE), "NONE");
        assert_eq!(format!("{}", FPGAControlFlags::FORCE_FAN), "FORCE_FAN");
        assert_eq!(
            format!("{}", FPGAControlFlags::READS_FPGA_INFO),
            "READS_FPGA_INFO"
        );
        assert_eq!(
            format!(
                "{}",
                FPGAControlFlags::FORCE_FAN | FPGAControlFlags::READS_FPGA_INFO
            ),
            "FORCE_FAN | READS_FPGA_INFO"
        );
    }
}
