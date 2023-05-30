/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3capi_def::common::*;

use autd3_modulation_audio_file::Wav;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWav(path: *const c_char, err: *mut c_char) -> ConstPtr {
    let path = try_or_return!(CStr::from_ptr(path).to_str(), err, NULL);
    let m = try_or_return!(Wav::new(path), err, NULL);
    Box::into_raw(ModulationWrap::new(m)) as _
}
