/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3_backend_cuda::*;
use autd3capi_def::{common::holo::Backend, BackendPtr};
use std::{ffi::c_char, rc::Rc};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCUDABackend(err: *mut c_char) -> BackendPtr {
    match CUDABackend::new() {
        Ok(b) => {
            let backend: Box<Rc<dyn Backend>> = Box::new(b);
            BackendPtr(Box::into_raw(backend) as _)
        }
        Err(e) => {
            let msg = std::ffi::CString::new(e.to_string()).unwrap();
            libc::strcpy(err, msg.as_ptr());
            BackendPtr(std::ptr::null_mut())
        }
    }
}
