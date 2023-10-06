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

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct GainSTMControlFlags : u8 {
        const NONE            = 0;
        const LEGACY          = 1 << 0;
        const DUTY            = 1 << 1;
        const STM_BEGIN       = 1 << 2;
        const STM_END         = 1 << 3;
        const USE_START_IDX   = 1 << 4;
        const USE_FINISH_IDX  = 1 << 5;
        const _RESERVED_0     = 1 << 6;
        const _RESERVED_1     = 1 << 7;
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
