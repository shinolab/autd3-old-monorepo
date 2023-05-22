/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

#[cfg(feature = "local")]
mod local;
#[cfg(feature = "local")]
pub use local::*;

#[cfg(feature = "remote")]
mod remote;
#[cfg(feature = "remote")]
pub use remote::*;
