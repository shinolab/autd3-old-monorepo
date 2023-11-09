/*
 * File: nalgebra_backend.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::rc::Rc;

use autd3capi_def::{holo::*, BackendPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDNalgebraBackend() -> BackendPtr {
    BackendPtr(Box::into_raw(Box::new(NalgebraBackend::new().unwrap())) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteNalgebraBackend(backend: BackendPtr) {
    let _ = Box::from_raw(backend.0 as *mut Rc<NalgebraBackend>);
}
