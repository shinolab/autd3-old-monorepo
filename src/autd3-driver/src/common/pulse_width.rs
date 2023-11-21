/*
 * File: pulse_width.rs
 * Project: common
 * Created Date: 21/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::EmitIntensity;
use crate::defined::{float, PI};

pub fn to_pulse_width(a: EmitIntensity, b: EmitIntensity) -> u16 {
    let a = a.value() as float / 255.0;
    let b = b.value() as float / 255.0;
    ((a * b).asin() / PI * 512.0).round() as u16
}

#[cfg(test)]

mod tests {
    use super::*;

    static ASIN_TABLE: &[u8; 65536] = include_bytes!("asin.dat");

    fn to_pulse_width_actual(a: u8, b: u8) -> u16 {
        let r = ASIN_TABLE[a as usize * b as usize];
        let full_width = a == 0xFF && b == 0xFF;
        if full_width {
            r as u16 | 0x0100
        } else {
            r as u16
        }
    }

    #[test]
    fn test_to_pulse_width() {
        for a in 0x00..0xFF {
            for b in 0x00..0xFF {
                assert_eq!(
                    to_pulse_width_actual(a, b),
                    to_pulse_width(EmitIntensity::new(a), EmitIntensity::new(b))
                );
            }
        }
    }
}
