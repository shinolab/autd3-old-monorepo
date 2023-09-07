/*
 * File: mod.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

mod advanced_phase_transducer;
mod advanced_transducer;
pub(crate) mod device;
mod legacy_transducer;
mod transducer;

use crate::defined::float;

pub type Vector3 = nalgebra::Vector3<float>;
pub type UnitVector3 = nalgebra::UnitVector3<float>;
pub type Vector4 = nalgebra::Vector4<float>;
pub type Quaternion = nalgebra::Quaternion<float>;
pub type UnitQuaternion = nalgebra::UnitQuaternion<float>;
pub type Matrix3 = nalgebra::Matrix3<float>;
pub type Matrix4 = nalgebra::Matrix4<float>;
pub type Affine = nalgebra::Affine3<float>;

pub use advanced_phase_transducer::*;
pub use advanced_transducer::*;
pub use device::*;
pub use legacy_transducer::*;
pub use transducer::*;
