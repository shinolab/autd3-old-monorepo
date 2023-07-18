/*
 * File: device.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use super::{UnitQuaternion, Vector3};

pub trait Device {
    /// Generate transducers of the device
    fn get_transducers(&self, start_id: usize) -> Vec<(usize, Vector3, UnitQuaternion)>;
}
