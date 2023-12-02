/*
 * File: drive.rs
 * Project: common
 * Created Date: 14/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::{EmitIntensity, Phase};

#[derive(Clone, Copy)]
pub struct Drive {
    /// Phase of ultrasound
    pub phase: Phase,
    /// emission intensity
    pub intensity: EmitIntensity,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive() {
        let d = Drive {
            phase: Phase::new(1),
            intensity: EmitIntensity::new(1),
        };

        let dc = Clone::clone(&d);
        assert_eq!(d.phase, dc.phase);
        assert_eq!(d.intensity, dc.intensity);
    }
}
