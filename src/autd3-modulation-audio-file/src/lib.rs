/*
 * File: lib.rs
 * Project: src
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod error;
mod rawpcm;
mod wav;

pub use rawpcm::RawPCM;
pub use wav::Wav;
