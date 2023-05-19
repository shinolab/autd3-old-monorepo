/*
 * File: dynamic_gain.rs
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

use autd3::core::gain::Gain;

use crate::{DynamicSendable, DynamicTransducer};

pub trait DynamicGain: DynamicSendable + Gain<DynamicTransducer> {}

impl<T: Gain<DynamicTransducer> + DynamicSendable> DynamicGain for T {}
