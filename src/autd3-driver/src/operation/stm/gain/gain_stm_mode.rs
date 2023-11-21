/*
 * File: gain_stm_mode.rs
 * Project: gain
 * Created Date: 08/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GainSTMMode {
    PhaseIntensityFull = 0,
    PhaseFull = 1,
    PhaseHalf = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_stm_mode() {
        assert_eq!(std::mem::size_of::<GainSTMMode>(), 2);

        assert_eq!(GainSTMMode::PhaseIntensityFull as u16, 0);
        assert_eq!(GainSTMMode::PhaseFull as u16, 1);
        assert_eq!(GainSTMMode::PhaseHalf as u16, 2);

        let mode = GainSTMMode::PhaseIntensityFull;

        let modec = Clone::clone(&mode);
        assert_eq!(modec, mode);
        assert_eq!(
            format!("{:?}", GainSTMMode::PhaseIntensityFull),
            "PhaseIntensityFull"
        );
        assert_eq!(format!("{:?}", GainSTMMode::PhaseFull), "PhaseFull");
        assert_eq!(format!("{:?}", GainSTMMode::PhaseHalf), "PhaseHalf");
    }
}
