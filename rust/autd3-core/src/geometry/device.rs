/*
 * File: device.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use super::Transducer;

pub trait Device<T: Transducer> {
    fn get_transducers(start_id: usize) -> Vec<T>;
}
