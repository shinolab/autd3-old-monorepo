/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/09/2023
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
// pub mod software_stm;

pub use autd3_derive as derive;
pub use autd3_driver as driver;

pub use controller::Controller;

#[cfg(test)]
mod tests {
    use autd3_driver::{
        defined::float,
        geometry::{Device, Geometry, IntoDevice, Transducer, Vector3},
    };

    pub struct GeometryBuilder<T: Transducer> {
        devices: Vec<Device<T>>,
    }

    impl<T: Transducer> Default for GeometryBuilder<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Transducer> GeometryBuilder<T> {
        pub fn new() -> GeometryBuilder<T> {
            GeometryBuilder::<T> { devices: vec![] }
        }

        pub fn add_device<D: IntoDevice<T>>(&mut self, dev: D) -> &mut Self {
            self.devices.push(dev.into_device(
                self.devices.len(),
                self.devices.iter().map(|dev| dev.num_transducers()).sum(),
            ));
            self
        }

        pub fn build(mut self) -> Geometry<T> {
            Geometry::<T>::new(self.devices)
        }
    }

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
