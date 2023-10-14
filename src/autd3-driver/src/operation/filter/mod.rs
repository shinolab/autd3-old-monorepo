/*
 * File: mod.rs
 * Project: filter
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod amp;
mod phase;

pub use amp::ConfigureAmpFilterOp;
pub use phase::ConfigurePhaseFilterOp;

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum FilterType {
    AddPhase = 0x00,
    AddDuty = 0x01,
}
