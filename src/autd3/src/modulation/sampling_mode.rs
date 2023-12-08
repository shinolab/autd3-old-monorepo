/*
 * File: sampling_mode.rs
 * Project: modulation
 * Created Date: 08/12/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SamplingMode {
    ExactFrequency,
    SizeOptimized,
}
