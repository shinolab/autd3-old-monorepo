/*
 * File: amp.rs
 * Project: src
 * Created Date: 22/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::defined::{float, ABSOLUTE_THRESHOLD_OF_HEARING};

#[allow(non_camel_case_types)]
pub struct dB;
pub struct Pascal;

#[derive(Clone, Copy, Debug)]
pub struct Amplitude {
    // Amplitude in Pascal
    pub(crate) value: float,
}

impl std::ops::Mul<dB> for float {
    type Output = Amplitude;

    fn mul(self, _rhs: dB) -> Self::Output {
        Self::Output {
            value: ABSOLUTE_THRESHOLD_OF_HEARING * float::powf(10.0, self / 20.0),
        }
    }
}

impl std::ops::Mul<Pascal> for float {
    type Output = Amplitude;

    fn mul(self, _rhs: Pascal) -> Self::Output {
        Self::Output { value: self }
    }
}

impl std::ops::Mul<Amplitude> for float {
    type Output = Amplitude;

    fn mul(self, rhs: Amplitude) -> Self::Output {
        Self::Output {
            value: self * rhs.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db() {
        let amp = 121.5 * dB;

        assert_eq!(amp.value, 23.77004454874038);
    }

    #[test]
    fn test_pascal() {
        let amp = 23.77004454874038 * Pascal;

        assert_eq!(amp.value, 23.77004454874038);
    }
}
