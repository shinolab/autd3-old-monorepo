/*
 * File: traits.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use std::io::Write;

use crate::parse::{Const, Enum, Function};

pub trait Generator {
    fn print_header<W: Write>(w: &mut W, bin_name: &str) -> Result<()>;
    fn get_filename(name: &str) -> String;
    fn register_func<W: Write>(w: &mut W, function: &Function) -> Result<()>;
    fn register_const<W: Write>(w: &mut W, constant: &Const) -> Result<()>;
    fn register_enum<W: Write>(w: &mut W, e: &Enum) -> Result<()>;
    fn start_other_types<W: Write>(w: &mut W) -> Result<()>;
    fn print_footer<W: Write>(w: &mut W) -> Result<()>;
}
