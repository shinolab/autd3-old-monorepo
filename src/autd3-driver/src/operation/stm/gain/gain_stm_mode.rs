/*
 * File: gain_stm_mode.rs
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
    fn gain_stm_mode() {
        assert_eq!(std::mem::size_of::<GainSTMMode>(), 2);

        assert_eq!(GainSTMMode::PhaseDutyFull as u16, 0);
        assert_eq!(GainSTMMode::PhaseFull as u16, 1);
        assert_eq!(GainSTMMode::PhaseHalf as u16, 2);

        let mode = GainSTMMode::PhaseDutyFull;

        let modec = Clone::clone(&mode);
        assert_eq!(modec, mode);
        assert_eq!(format!("{:?}", GainSTMMode::PhaseDutyFull), "PhaseDutyFull");
        assert_eq!(format!("{:?}", GainSTMMode::PhaseFull), "PhaseFull");
        assert_eq!(format!("{:?}", GainSTMMode::PhaseHalf), "PhaseHalf");
    }
}
