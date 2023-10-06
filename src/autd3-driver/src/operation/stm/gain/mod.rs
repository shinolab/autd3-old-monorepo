/*
 * File: mod.rs
 * Project: gain
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod advanced;
mod advanced_phase;
mod legacy;

pub use advanced::GainSTMAdvancedOp;
pub use advanced_phase::GainSTMAdvancedPhaseOp;
pub use legacy::GainSTMLegacyOp;

use std::fmt;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GainSTMControlFlags {
    bits: u8,
}

impl GainSTMControlFlags {
    pub const NONE: Self = Self { bits: 0 };
    pub const LEGACY: Self = Self { bits: 1 << 0 };
    pub const DUTY: Self = Self { bits: 1 << 1 };
    pub const STM_BEGIN: Self = Self { bits: 1 << 2 };
    pub const STM_END: Self = Self { bits: 1 << 3 };
    pub const USE_START_IDX: Self = Self { bits: 1 << 4 };
    pub const USE_FINISH_IDX: Self = Self { bits: 1 << 5 };

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

impl std::ops::BitOr for GainSTMControlFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl fmt::Display for GainSTMControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(GainSTMControlFlags::LEGACY) {
            flags.push("LEGACY")
        }
        if self.contains(GainSTMControlFlags::DUTY) {
            flags.push("DUTY")
        }
        if self.contains(GainSTMControlFlags::STM_BEGIN) {
            flags.push("STM_BEGIN")
        }
        if self.contains(GainSTMControlFlags::STM_END) {
            flags.push("STM_END")
        }
        if self.contains(GainSTMControlFlags::USE_START_IDX) {
            flags.push("USE_START_IDX")
        }
        if self.contains(GainSTMControlFlags::USE_FINISH_IDX) {
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

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GainSTMMode {
    PhaseDutyFull = 0,
    PhaseFull = 1,
    PhaseHalf = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_stm_controll_flag() {
        assert_eq!(std::mem::size_of::<GainSTMControlFlags>(), 1);

        let flags = GainSTMControlFlags::LEGACY | GainSTMControlFlags::DUTY;

        let flagsc = flags.clone();
        assert_eq!(flagsc.bits(), flags.bits());
    }

    #[test]
    fn gain_stm_controll_flag_contains() {
        let flags = GainSTMControlFlags::LEGACY | GainSTMControlFlags::DUTY;

        assert!(flags.contains(GainSTMControlFlags::LEGACY));
        assert!(flags.contains(GainSTMControlFlags::DUTY));
        assert!(!flags.contains(GainSTMControlFlags::STM_BEGIN));
        assert!(!flags.contains(GainSTMControlFlags::STM_END));
        assert!(!flags.contains(GainSTMControlFlags::USE_START_IDX));
        assert!(!flags.contains(GainSTMControlFlags::USE_FINISH_IDX));
    }

    #[test]
    fn gain_stm_controll_flag_set() {
        let mut flags = GainSTMControlFlags::NONE;
        flags.set(GainSTMControlFlags::LEGACY);
        assert!(flags.contains(GainSTMControlFlags::LEGACY));
        assert!(!flags.contains(GainSTMControlFlags::DUTY));
        assert!(!flags.contains(GainSTMControlFlags::STM_BEGIN));
        assert!(!flags.contains(GainSTMControlFlags::STM_END));
        assert!(!flags.contains(GainSTMControlFlags::USE_START_IDX));
        assert!(!flags.contains(GainSTMControlFlags::USE_FINISH_IDX));
    }

    #[test]
    fn gain_stm_controll_flag_clear() {
        let mut flags = GainSTMControlFlags::LEGACY | GainSTMControlFlags::DUTY;
        flags.clear(GainSTMControlFlags::LEGACY);
        assert!(!flags.contains(GainSTMControlFlags::LEGACY));
        assert!(flags.contains(GainSTMControlFlags::DUTY));
        assert!(!flags.contains(GainSTMControlFlags::STM_BEGIN));
        assert!(!flags.contains(GainSTMControlFlags::STM_END));
        assert!(!flags.contains(GainSTMControlFlags::USE_START_IDX));
        assert!(!flags.contains(GainSTMControlFlags::USE_FINISH_IDX));
    }

    #[test]
    fn gain_stm_controll_flag_set_by() {
        let mut flags = GainSTMControlFlags::NONE;
        flags.set_by(GainSTMControlFlags::LEGACY, true);
        assert!(flags.contains(GainSTMControlFlags::LEGACY));
        assert!(!flags.contains(GainSTMControlFlags::DUTY));
        assert!(!flags.contains(GainSTMControlFlags::STM_BEGIN));
        assert!(!flags.contains(GainSTMControlFlags::STM_END));
        assert!(!flags.contains(GainSTMControlFlags::USE_START_IDX));
        assert!(!flags.contains(GainSTMControlFlags::USE_FINISH_IDX));

        flags.set_by(GainSTMControlFlags::LEGACY, false);
        assert!(!flags.contains(GainSTMControlFlags::LEGACY));
        assert!(!flags.contains(GainSTMControlFlags::DUTY));
        assert!(!flags.contains(GainSTMControlFlags::STM_BEGIN));
        assert!(!flags.contains(GainSTMControlFlags::STM_END));
        assert!(!flags.contains(GainSTMControlFlags::USE_START_IDX));
        assert!(!flags.contains(GainSTMControlFlags::USE_FINISH_IDX));
    }

    #[test]
    fn gain_stm_controll_flag_fmt() {
        assert_eq!(format!("{}", GainSTMControlFlags::NONE), "NONE");
        assert_eq!(format!("{}", GainSTMControlFlags::LEGACY), "LEGACY");
        assert_eq!(format!("{}", GainSTMControlFlags::DUTY), "DUTY");
        assert_eq!(format!("{}", GainSTMControlFlags::STM_BEGIN), "STM_BEGIN");
        assert_eq!(format!("{}", GainSTMControlFlags::STM_END), "STM_END");
        assert_eq!(
            format!("{}", GainSTMControlFlags::USE_START_IDX),
            "USE_START_IDX"
        );
        assert_eq!(
            format!("{}", GainSTMControlFlags::USE_FINISH_IDX),
            "USE_FINISH_IDX"
        );

        assert_eq!(
            format!(
                "{}",
                GainSTMControlFlags::LEGACY
                    | GainSTMControlFlags::DUTY
                    | GainSTMControlFlags::STM_BEGIN
                    | GainSTMControlFlags::STM_END
                    | GainSTMControlFlags::USE_START_IDX
                    | GainSTMControlFlags::USE_FINISH_IDX
            ),
            "LEGACY | DUTY | STM_BEGIN | STM_END | USE_START_IDX | USE_FINISH_IDX"
        );
    }

    #[test]
    fn gain_stm_mode() {
        assert_eq!(std::mem::size_of::<GainSTMMode>(), 2);

        assert_eq!(GainSTMMode::PhaseDutyFull as u16, 0);
        assert_eq!(GainSTMMode::PhaseFull as u16, 1);
        assert_eq!(GainSTMMode::PhaseHalf as u16, 2);

        let mode = GainSTMMode::PhaseDutyFull;
        let modec = mode.clone();

        assert_eq!(modec, mode);

        assert_eq!(format!("{:?}", GainSTMMode::PhaseDutyFull), "PhaseDutyFull");
        assert_eq!(format!("{:?}", GainSTMMode::PhaseFull), "PhaseFull");
        assert_eq!(format!("{:?}", GainSTMMode::PhaseHalf), "PhaseHalf");
    }
}
