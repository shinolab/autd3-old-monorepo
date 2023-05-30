/*
 * File: mod.rs
 * Project: remote
 * Created Date: 21/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod link_soem_remote;

pub use link_soem_remote::{Empty, Filled, RemoteSOEM, RemoteSOEMBuilder};
