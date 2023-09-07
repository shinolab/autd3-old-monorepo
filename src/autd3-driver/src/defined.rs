/*
 * File: defined.rs
 * Project: src
 * Created Date: 05/12/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

#[cfg(feature = "single_float")]
mod float_def {
    pub use f32 as float;
    pub use std::f32::consts::PI;
}
#[cfg(not(feature = "single_float"))]
mod float_def {
    pub use f64 as float;
    pub use std::f64::consts::PI;
}

pub use float_def::*;

#[cfg(feature = "use_meter")]
mod unit {
    pub const METER: super::float = 1.0;
}
#[cfg(not(feature = "use_meter"))]
mod unit {
    pub const METER: super::float = 1000.0;
}
pub use unit::*;
pub const MILLIMETER: float = METER / 1000.0;

#[derive(Clone, Copy, Debug)]
pub struct Drive {
    /// Phase of ultrasound (from 0 to 2π)
    pub phase: float,
    /// Normalized amplitude of ultrasound (from 0 to 1)
    pub amp: float,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive() {
        let d = Drive {
            phase: 0.1,
            amp: 0.2,
        };
        let dc = d;
        assert_eq!(d.phase, dc.phase);
        assert_eq!(d.amp, dc.amp);

        dbg!(d);
    }
}
