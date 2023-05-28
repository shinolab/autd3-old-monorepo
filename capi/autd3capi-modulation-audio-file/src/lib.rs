#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3capi_common::*;

use autd3_modulation_audio_file::Wav;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWav(path: *const c_char, err: *mut c_char) -> ConstPtr {
    let path = try_or_return!(CStr::from_ptr(path).to_str(), err, NULL);
    let m = try_or_return!(Wav::new(path), err, NULL);
    Box::into_raw(ModulationWrap::new(m)) as _
}
