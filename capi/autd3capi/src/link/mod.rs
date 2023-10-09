/*
 * File: mod.rs
 * Project: link
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{
    common::{cast, Cnt},
    ControllerPtr, LinkPtr,
};

pub mod audit;
pub mod nop;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkGet(cnt: ControllerPtr) -> LinkPtr {
    LinkPtr(cast!(cnt.0, Cnt).link() as *const _ as _)
}
