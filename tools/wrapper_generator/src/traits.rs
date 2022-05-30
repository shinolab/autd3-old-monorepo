/*
 * File: traits.rs
 * Project: src
 * Created Date: 25/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use std::io::Write;

use capi_header_parser::parse::Function;

pub trait Generator {
    fn print_header<W: Write>(w: &mut W, bin_name: &str) -> Result<()>;
    fn get_filename(name: &str) -> String;
    fn register_func<W: Write>(w: &mut W, function: &Function) -> Result<()>;
    fn print_footer<W: Write>(w: &mut W) -> Result<()>;
}
