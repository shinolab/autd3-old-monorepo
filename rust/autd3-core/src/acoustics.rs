/*
 * File: utils.rs
 * Project: src
 * Created Date: 06/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_A: &[f64] = &[
    1.0,
    1.0,
    1.0,
    0.891250938,
    0.707945784,
    0.501187234,
    0.354813389,
    0.251188643,
    0.199526231,
];

#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_B: &[f64] = &[
    0.,
    0.,
    -0.00459648054721,
    -0.0155520765675,
    -0.0208114779827,
    -0.0182211227016,
    -0.0122437497109,
    -0.00780345575475,
    -0.00312857467007,
];

#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_C: &[f64] = &[
    0.,
    0.,
    -0.000787968093807,
    -0.000307591508224,
    -0.000218348633296,
    0.00047738416141,
    0.000120353137658,
    0.000323676257958,
    0.000143850511,
];

#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_D: &[f64] = &[
    0.,
    0.,
    1.60125528528e-05,
    2.9747624976e-06,
    2.31910931569e-05,
    -1.1901034125e-05,
    6.77743734332e-06,
    -5.99548024824e-06,
    -4.79372835035e-06,
];

#[allow(clippy::many_single_char_names)]
pub fn directivity_t4010a1(theta_deg: f64) -> f64 {
    let mut theta_deg = theta_deg.abs();

    while theta_deg > 90.0 {
        theta_deg = (180.0 - theta_deg).abs();
    }

    let i = (theta_deg / 10.0).ceil() as usize;

    if i == 0 {
        1.0
    } else {
        let a = DIR_COEF_A[i - 1];
        let b = DIR_COEF_B[i - 1];
        let c = DIR_COEF_C[i - 1];
        let d = DIR_COEF_D[i - 1];
        let x = theta_deg - (i as f64 - 1.0) * 10.0;
        a + b * x + c * x * x + d * x * x * x
    }
}
