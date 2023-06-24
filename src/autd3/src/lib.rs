/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

pub mod controller;
pub mod error;
pub mod gain;
pub mod link;
pub mod modulation;
pub mod prelude;
pub mod software_stm;

pub use autd3_core as core;
pub use autd3_traits as traits;
pub use controller::Controller;

#[cfg(test)]
mod tests {
    use autd3_core::{
        error::AUTDInternalError,
        float,
        geometry::{Device, Geometry, Transducer, UnitQuaternion, Vector3},
        METER,
    };
    use std::marker::PhantomData;

    pub struct GeometryBuilder<T: Transducer> {
        attenuation: float,
        sound_speed: float,
        transducers: Vec<(usize, Vector3, UnitQuaternion)>,
        device_map: Vec<usize>,
        phantom: PhantomData<T>,
    }

    impl<T: Transducer> Default for GeometryBuilder<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Transducer> GeometryBuilder<T> {
        pub fn new() -> GeometryBuilder<T> {
            GeometryBuilder::<T> {
                attenuation: 0.0,
                sound_speed: 340.0 * METER,
                transducers: vec![],
                device_map: vec![],
                phantom: PhantomData,
            }
        }

        pub fn add_device<D: Device>(&mut self, dev: D) -> &mut Self {
            let id = self.transducers.len();
            let mut t = dev.get_transducers(id);
            self.device_map.push(t.len());
            self.transducers.append(&mut t);
            self
        }

        pub fn build(&mut self) -> Result<Geometry<T>, AUTDInternalError> {
            Geometry::<T>::new(
                self.transducers
                    .iter()
                    .map(|&(id, pos, rot)| T::new(id, pos, rot))
                    .collect(),
                self.device_map.clone(),
                self.sound_speed,
                self.attenuation,
            )
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
