/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3capi_def::{common::*, ModulationPtr};

use autd3_modulation_audio_file::Wav;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWav(path: *const c_char, err: *mut c_char) -> ModulationPtr {
    let path = try_or_return!(CStr::from_ptr(path).to_str(), err, ModulationPtr(NULL));
    let m = try_or_return!(Wav::new(path), err, ModulationPtr(NULL));
    ModulationPtr::new(m)
}
