/*
 * File: drive.rs
 * Project: common
 * Created Date: 14/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::defined::float;

use super::Amplitude;

#[derive(Clone, Copy)]
pub struct Drive {
    /// Phase of ultrasound (from 0 to 2π)
    pub phase: float,
    /// Normalized amplitude of ultrasound
    pub amp: Amplitude,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive() {
        let d = Drive {
            phase: 0.1,
            amp: Amplitude::new_clamped(0.2),
        };

        let dc = Clone::clone(&d);
        assert_eq!(d.phase, dc.phase);
        assert_eq!(d.amp.value(), dc.amp.value());
    }
}
