/*
 * File: sphere.rs
 * Project: acoustics
 * Created Date: 04/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use super::*;

/// Directivity of spherical wave
pub struct Sphere {}

impl Directivity for Sphere {
    fn directivity(_: float) -> float {
        1.
    }

    fn directivity_from_tr<T: Transducer>(_: &T, _: &Vector3) -> float {
        1.
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Quaternion, UnitQuaternion};

    use super::*;

    use rand::prelude::*;

    #[test]
    fn directivity() {
        let mut rng = rand::thread_rng();
        assert_eq!(1.0, Sphere::directivity(rng.gen()));
    }

    #[test]
    fn directivity_from_tr() {
        let mut rng = rand::thread_rng();

        let tr = crate::geometry::LegacyTransducer::new(
            rng.gen(),
            Vector3::new(rng.gen(), rng.gen(), rng.gen()),
            UnitQuaternion::from_quaternion(Quaternion::new(
                rng.gen(),
                rng.gen(),
                rng.gen(),
                rng.gen(),
            )),
        );

        assert_eq!(
            1.0,
            Sphere::directivity_from_tr(&tr, &Vector3::new(rng.gen(), rng.gen(), rng.gen()))
        );
    }
}
