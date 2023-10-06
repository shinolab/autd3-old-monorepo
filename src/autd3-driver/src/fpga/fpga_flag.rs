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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FPGAControlFlags {
    bits: u8,
}

impl FPGAControlFlags {
    pub const NONE: Self = Self { bits: 0 };
    pub const FORCE_FAN: Self = Self { bits: 1 << 0 };
    pub const READS_FPGA_INFO: Self = Self { bits: 1 << 1 };

    pub fn contains(&self, other: Self) -> bool {
        self.bits & other.bits != 0
    }

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn set_by(&mut self, other: Self, value: bool) {
        if value {
            self.set(other)
        } else {
            self.clear(other)
        }
    }

    pub fn set(&mut self, other: Self) {
        self.bits |= other.bits
    }

    pub fn clear(&mut self, other: Self) {
        self.bits &= !other.bits
    }
}

impl std::ops::BitOr for FPGAControlFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl std::fmt::Display for FPGAControlFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn contains() {
        assert!(FPGAControlFlags::FORCE_FAN.contains(FPGAControlFlags::FORCE_FAN));
        assert!(!FPGAControlFlags::FORCE_FAN.contains(FPGAControlFlags::READS_FPGA_INFO));
    }

    #[test]
    fn set() {
        let mut flags = FPGAControlFlags::NONE;
        flags.set(FPGAControlFlags::FORCE_FAN);
        assert!(flags.contains(FPGAControlFlags::FORCE_FAN));
        assert!(!flags.contains(FPGAControlFlags::READS_FPGA_INFO));
    }

    #[test]
    fn clear() {
        let mut flags = FPGAControlFlags::FORCE_FAN;
        flags.clear(FPGAControlFlags::FORCE_FAN);
        assert!(!flags.contains(FPGAControlFlags::FORCE_FAN));
        assert!(!flags.contains(FPGAControlFlags::READS_FPGA_INFO));
    }

    #[test]
    fn set_by() {
        let mut flags = FPGAControlFlags::NONE;
        flags.set_by(FPGAControlFlags::FORCE_FAN, true);
        assert!(flags.contains(FPGAControlFlags::FORCE_FAN));
        assert!(!flags.contains(FPGAControlFlags::READS_FPGA_INFO));

        flags.set_by(FPGAControlFlags::FORCE_FAN, false);
        assert!(!flags.contains(FPGAControlFlags::FORCE_FAN));
        assert!(!flags.contains(FPGAControlFlags::READS_FPGA_INFO));
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
