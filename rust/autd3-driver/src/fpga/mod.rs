/*
 * File: mod.rs
 * Project: fpga
 * Created Date: 02/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod error;
mod fpga_defined;

pub use error::FPGAError;
pub use fpga_defined::*;
