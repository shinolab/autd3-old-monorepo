/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod autd3_device;
pub mod controller;
pub mod error;
pub mod gain;
pub mod link;
pub mod modulation;
pub mod prelude;
pub mod software_stm;

pub use autd3_derive as derive;

pub use controller::Controller;

#[cfg(test)]
mod tests {
    use autd3_driver::{defined::float, geometry::Vector3};

    pub fn random_vector3(
        range_x: std::ops::Range<float>,
        range_y: std::ops::Range<float>,
        range_z: std::ops::Range<float>,
    ) -> Vector3 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Vector3::new(
            rng.gen_range(range_x),
            rng.gen_range(range_y),
            rng.gen_range(range_z),
        )
    }
}
