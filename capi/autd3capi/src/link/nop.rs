/*
 * File: nop.rs
 * Project: link
 * Created Date: 06/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::link::Nop, LinkBuilderPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkNop() -> LinkBuilderPtr {
    LinkBuilderPtr::new(Nop::builder())
}
