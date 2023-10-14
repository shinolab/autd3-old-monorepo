/*
 * File: mod.rs
 * Project: directivity
 * Created Date: 08/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod sphere;
mod t4010a1;

use crate::{
    defined::{float, PI},
    geometry::{Transducer, Vector3},
};

pub use sphere::Sphere;
pub use t4010a1::T4010A1;

/// Directivity
pub trait Directivity: Send + Sync {
    fn directivity(theta_deg: float) -> float;
    fn directivity_from_tr<T: Transducer>(tr: &T, target: &Vector3) -> float {
        let dir = tr.z_direction();
        Self::directivity((dir.cross(target).norm()).atan2(dir.dot(target)) * 180. / PI)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::geometry::UnitQuaternion;

    pub struct TestDirectivity {}

    impl Directivity for TestDirectivity {
        fn directivity(t: float) -> float {
            t
        }
    }

    #[test]
    fn directivity_from_tr() {
        let tr =
            crate::geometry::LegacyTransducer::new(0, Vector3::zeros(), UnitQuaternion::identity());

        assert_approx_eq::assert_approx_eq!(
            0.,
            TestDirectivity::directivity_from_tr(&tr, &tr.z_direction())
        );
        assert_approx_eq::assert_approx_eq!(
            90.,
            TestDirectivity::directivity_from_tr(&tr, &tr.x_direction())
        );
    }
}
