/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3capi_def::{common::*, take_mod, ModulationPtr, ResultModulationPtr};

use autd3_modulation_audio_file::{RawPCM, Wav};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWav(path: *const c_char) -> ResultModulationPtr {
    let path = match CStr::from_ptr(path).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = std::ffi::CString::new(e.to_string()).unwrap();
            return ResultModulationPtr {
                result: ModulationPtr(NULL),
                err_len: err.as_bytes_with_nul().len() as u32,
                err: err.into_raw(),
            };
        }
    };
    match Wav::new(path) {
        Ok(v) => ResultModulationPtr {
            result: ModulationPtr::new(v),
            err_len: 0,
            err: std::ptr::null(),
        },
        Err(e) => {
            let err = std::ffi::CString::new(e.to_string()).unwrap();
            ResultModulationPtr {
                result: ModulationPtr(NULL),
                err_len: err.as_bytes_with_nul().len() as u32,
                err: err.into_raw(),
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWavWithSamplingFrequencyDivision(
    m: ModulationPtr,
    div: u32,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Wav).with_sampling_frequency_division(div))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationRawPCM(
    path: *const c_char,
    sample_rate: u32,
) -> ResultModulationPtr {
    let path = match CStr::from_ptr(path).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = std::ffi::CString::new(e.to_string()).unwrap();
            return ResultModulationPtr {
                result: ModulationPtr(NULL),
                err_len: err.as_bytes_with_nul().len() as u32,
                err: err.into_raw(),
            };
        }
    };
    match RawPCM::new(path, sample_rate) {
        Ok(v) => ResultModulationPtr {
            result: ModulationPtr::new(v),
            err_len: 0,
            err: std::ptr::null(),
        },
        Err(e) => {
            let err = std::ffi::CString::new(e.to_string()).unwrap();
            ResultModulationPtr {
                result: ModulationPtr(NULL),
                err_len: err.as_bytes_with_nul().len() as u32,
                err: err.into_raw(),
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationRawPCMWithSamplingFrequencyDivision(
    m: ModulationPtr,
    div: u32,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, RawPCM).with_sampling_frequency_division(div))
}
