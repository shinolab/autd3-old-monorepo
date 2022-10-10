/*
 * File: hardware.rs
 * Project: src
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use num::FromPrimitive;

pub const NUM_TRANS_IN_UNIT: usize = 249;
pub const NUM_TRANS_X: usize = 18;
pub const NUM_TRANS_Y: usize = 14;
pub const TRANS_SPACING_MM: f64 = 10.16;
pub const DEVICE_WIDTH: f64 = 192.0;
pub const DEVICE_HEIGHT: f64 = 151.4;

pub fn is_missing_transducer<T1, T2>(x: T1, y: T2) -> bool
where
    T1: FromPrimitive + PartialEq<T1>,
    T2: FromPrimitive + PartialEq<T2>,
{
    y == FromPrimitive::from_u8(1).unwrap()
        && (x == FromPrimitive::from_u8(1).unwrap()
            || x == FromPrimitive::from_u8(2).unwrap()
            || x == FromPrimitive::from_u8(16).unwrap())
}
