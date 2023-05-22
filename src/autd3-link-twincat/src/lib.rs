/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

mod error;

#[cfg(feature = "local")]
mod local;
#[cfg(feature = "remote")]
mod remote;

#[cfg(feature = "local")]
pub use local::TwinCAT;

#[cfg(feature = "remote")]
pub use remote::RemoteTwinCAT;
