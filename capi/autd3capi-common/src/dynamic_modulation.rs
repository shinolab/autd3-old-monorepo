/*
 * File: dynamic_modulation.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::core::modulation::Modulation;

use crate::DynamicSendable;

pub trait DynamicModulation: DynamicSendable + Modulation {}

impl<T: Modulation + DynamicSendable> DynamicModulation for T {}
