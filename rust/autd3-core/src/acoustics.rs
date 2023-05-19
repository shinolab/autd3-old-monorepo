/*
 * File: utils.rs
 * Project: src
 * Created Date: 06/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{float, PI};

use crate::geometry::Vector3;

pub type Complex = nalgebra::Complex<float>;

#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
static DIR_COEF_A: &[float] = &[
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
static DIR_COEF_B: &[float] = &[
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
static DIR_COEF_C: &[float] = &[
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
static DIR_COEF_D: &[float] = &[
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

pub trait Directivity {
    fn directivity(theta_deg: float) -> float;
}

pub struct T4010A1 {}

impl Directivity for T4010A1 {
    #[allow(clippy::many_single_char_names)]
    fn directivity(theta_deg: float) -> float {
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
            let x = theta_deg - (i as float - 1.0) * 10.0;
            ((d * x + c) * x + b) * x + a
        }
    }
}

pub struct Sphere {}

impl Directivity for Sphere {
    fn directivity(_: float) -> float {
        1.
    }
}

pub fn propagate<D: Directivity>(
    source_pos: &Vector3,
    source_dir: &Vector3,
    attenuation: float,
    wavenumber: float,
    target_pos: &Vector3,
) -> Complex {
    let diff = target_pos - source_pos;
    let dist = diff.norm();
    let theta = (source_dir.cross(&diff).norm()).atan2(source_dir.dot(&diff)) * 180. / PI;
    Complex::from_polar(
        D::directivity(theta) * (-dist * attenuation).exp() / dist,
        -wavenumber * dist,
    )
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::*;

    macro_rules! assert_complex_approx_eq {
        ($a:expr, $b:expr) => {
            assert_approx_eq!($a.re, $b.re);
            assert_approx_eq!($a.im, $b.im);
        };
    }

    #[test]
    fn directivity_t4010a1() {
        let expects = [
            1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
            0.994632, 0.987783, 0.979551, 0.970031, 0.95932, 0.947513, 0.934707, 0.920997,
            0.906479, 0.891251, 0.875394, 0.85894, 0.841907, 0.824312, 0.806173, 0.787508,
            0.768335, 0.748672, 0.728536, 0.707946, 0.686939, 0.665635, 0.644172, 0.622691,
            0.601329, 0.580226, 0.559521, 0.539353, 0.519863, 0.501187, 0.483432, 0.466559,
            0.450499, 0.435179, 0.420529, 0.406476, 0.392949, 0.379878, 0.367189, 0.354813,
            0.342697, 0.330862, 0.319348, 0.308198, 0.297451, 0.287148, 0.277329, 0.268036,
            0.259309, 0.251189, 0.243703, 0.236828, 0.230529, 0.22477, 0.219514, 0.214725,
            0.210368, 0.206407, 0.202805, 0.199526, 0.196537, 0.193806, 0.191306, 0.189007,
            0.18688, 0.184898, 0.183031, 0.18125, 0.179526, 0.177831,
        ];

        for (i, expect) in expects.iter().enumerate() {
            assert_approx_eq!(T4010A1::directivity(i as float), expect);
        }
    }

    #[test]
    fn propagate_test() {
        let wavenumber = 2.0 * PI / 2.0;

        assert_complex_approx_eq!(
            propagate::<T4010A1>(
                &Vector3::zeros(),
                &Vector3::z_axis(),
                0.0,
                wavenumber,
                &Vector3::new(0.0, 0.0, 1.0)
            ),
            Complex::new(-1., 0.)
        );

        assert_complex_approx_eq!(
            propagate::<T4010A1>(
                &Vector3::zeros(),
                &Vector3::z_axis(),
                0.0,
                wavenumber,
                &Vector3::new(0.0, 0.0, 2.0)
            ),
            Complex::new(0.5, 0.)
        );

        assert_complex_approx_eq!(
            propagate::<T4010A1>(
                &Vector3::zeros(),
                &Vector3::z_axis(),
                0.0,
                wavenumber,
                &Vector3::new(1.0, 0.0, 0.0)
            ),
            Complex::new(-0.177831, 0.)
        );

        assert_complex_approx_eq!(
            propagate::<T4010A1>(
                &Vector3::zeros(),
                &Vector3::z_axis(),
                0.0,
                wavenumber,
                &Vector3::new(0.0, 1.0, 0.0)
            ),
            Complex::new(-0.177831, 0.)
        );

        assert_complex_approx_eq!(
            propagate::<T4010A1>(
                &Vector3::zeros(),
                &Vector3::x_axis(),
                0.0,
                wavenumber,
                &Vector3::new(1.0, 0.0, 0.0)
            ),
            Complex::new(-1., 0.)
        );
    }
}
