/*
 * File: mod.rs
 * Project: acoustics
 * Created Date: 04/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod directivity;

use crate::{
    defined::{float, Complex},
    geometry::{Transducer, Vector3},
};

use directivity::Directivity;

/// Calculate propagation of ultrasound wave
///
/// # Arguments
///
/// * `tr` - Source transducer
/// * `attenuation` - Attenuation coefficient
/// * `sound_speed` - Speed of sound
/// * `target_pos` - Position of target
///
pub fn propagate<D: Directivity, T: Transducer>(
    tr: &T,
    attenuation: float,
    sound_speed: float,
    target_pos: &Vector3,
) -> Complex {
    let diff = target_pos - tr.position();
    let dist = diff.norm();
    let r = D::directivity_from_tr(tr, &diff) * (-dist * attenuation).exp() / dist;
    let phase = -tr.wavenumber(sound_speed) * dist;
    Complex::new(r * phase.cos(), r * phase.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::Rng;

    use crate::geometry::UnitQuaternion;
    use directivity::tests::TestDirectivity;

    macro_rules! assert_complex_approx_eq {
        ($a:expr, $b:expr) => {
            assert_approx_eq::assert_approx_eq!($a.re, $b.re);
            assert_approx_eq::assert_approx_eq!($a.im, $b.im);
        };
    }

    #[test]
    fn propagate() {
        let mut rng = rand::thread_rng();

        let tr =
            crate::geometry::LegacyTransducer::new(0, Vector3::zeros(), UnitQuaternion::identity());

        let atten = rng.gen_range(0.0..1.0);
        let c = rng.gen_range(300e3..400e3);
        let target = Vector3::new(
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
        );

        let expect = {
            let dist = target.norm();
            let r =
                TestDirectivity::directivity_from_tr(&tr, &target) * (-dist * atten).exp() / dist;
            let phase = -tr.wavenumber(c) * dist;
            Complex::new(r * phase.cos(), r * phase.sin())
        };
        assert_complex_approx_eq!(
            expect,
            super::propagate::<TestDirectivity, _>(&tr, atten, c, &target)
        );
    }
}
