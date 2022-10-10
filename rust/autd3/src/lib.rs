/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod controller;
mod error;
pub mod gain;
pub mod modulation;
pub mod prelude;

pub use autd3_core;
pub use controller::Controller;
