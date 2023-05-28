/*
 * File: traits.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use std::path::Path;

use crate::parse::{Const, Enum, Function};

pub trait Generator {
    fn new() -> Self;
    fn register_func(self, function: Vec<Function>) -> Self;
    fn register_const(self, constant: Vec<Const>) -> Self;
    fn register_enum(self, e: Vec<Enum>) -> Self;
    fn write<P: AsRef<Path>>(self, path: P, crate_name: &str) -> Result<()>;
}
