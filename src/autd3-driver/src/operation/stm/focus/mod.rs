/*
 * File: mod.rs
 * Project: focus
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod control_point;
mod focus_stm_op;

pub use control_point::ControlPoint;
pub use focus_stm_op::FocusSTMOp;

use std::fmt;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FocusSTMControlFlags {
    bits: u8,
}

impl FocusSTMControlFlags {
    pub const NONE: Self = Self { bits: 0 };
    pub const STM_BEGIN: Self = Self { bits: 1 << 0 };
    pub const STM_END: Self = Self { bits: 1 << 1 };
    pub const USE_START_IDX: Self = Self { bits: 1 << 2 };
    pub const USE_FINISH_IDX: Self = Self { bits: 1 << 3 };

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

impl std::ops::BitOr for FocusSTMControlFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl fmt::Display for FocusSTMControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Vec::new();
        if self.contains(FocusSTMControlFlags::STM_BEGIN) {
            flags.push("STM_BEGIN")
        }
        if self.contains(FocusSTMControlFlags::STM_END) {
            flags.push("STM_END")
        }
        if self.contains(FocusSTMControlFlags::USE_START_IDX) {
            flags.push("USE_START_IDX")
        }
        if self.contains(FocusSTMControlFlags::USE_FINISH_IDX) {
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
    use super::*;

    #[test]
    fn focus_stm_controll_flag() {
        assert_eq!(std::mem::size_of::<FocusSTMControlFlags>(), 1);

        let flags = FocusSTMControlFlags::STM_BEGIN | FocusSTMControlFlags::STM_END;

        let flagsc = flags.clone();

        assert!(flagsc.contains(FocusSTMControlFlags::STM_BEGIN));
        assert!(flagsc.contains(FocusSTMControlFlags::STM_END));
        assert!(!flagsc.contains(FocusSTMControlFlags::USE_START_IDX));
        assert!(!flagsc.contains(FocusSTMControlFlags::USE_FINISH_IDX));
    }

    #[test]
    fn focus_stm_controll_flag_fmt() {
        assert_eq!(format!("{}", FocusSTMControlFlags::NONE), "NONE");
        assert_eq!(format!("{}", FocusSTMControlFlags::STM_BEGIN), "STM_BEGIN");
        assert_eq!(format!("{}", FocusSTMControlFlags::STM_END), "STM_END");
        assert_eq!(
            format!("{}", FocusSTMControlFlags::USE_START_IDX),
            "USE_START_IDX"
        );
        assert_eq!(
            format!("{}", FocusSTMControlFlags::USE_FINISH_IDX),
            "USE_FINISH_IDX"
        );

        assert_eq!(
            format!(
                "{}",
                FocusSTMControlFlags::STM_BEGIN
                    | FocusSTMControlFlags::STM_END
                    | FocusSTMControlFlags::USE_START_IDX
                    | FocusSTMControlFlags::USE_FINISH_IDX
            ),
            "STM_BEGIN | STM_END | USE_START_IDX | USE_FINISH_IDX"
        );
    }
}
