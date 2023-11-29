/*
 * File: lib.rs
 * Project: src
 * Created Date: 27/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::{c_char, CStr};

use autd3capi_def::*;

use autd3_modulation_audio_file::{RawPCM, Wav};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWav(path: *const c_char) -> ResultModulation {
    let path = match CStr::from_ptr(path).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    match Wav::new(path) {
        Ok(v) => ResultModulation {
            result: ModulationPtr::new(v),
            err_len: 0,
            err: std::ptr::null_mut(),
        },
        Err(e) => {
            let err = e.to_string();
            ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWavWithSamplingConfig(
    m: ModulationPtr,
    config: SamplingConfiguration,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Wav).with_sampling_config(config.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationRawPCM(
    path: *const c_char,
    sample_rate: u32,
) -> ResultModulation {
    let path = match CStr::from_ptr(path).to_str() {
        Ok(v) => v,
        Err(e) => {
            let err = e.to_string();
            return ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            };
        }
    };
    match RawPCM::new(path, sample_rate) {
        Ok(v) => ResultModulation {
            result: ModulationPtr::new(v),
            err_len: 0,
            err: std::ptr::null_mut(),
        },
        Err(e) => {
            let err = e.to_string();
            ResultModulation {
                result: ModulationPtr(std::ptr::null()),
                err_len: err.as_bytes().len() as u32 + 1,
                err: Box::into_raw(Box::new(err)) as _,
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationRawPCMWithSamplingConfig(
    m: ModulationPtr,
    config: SamplingConfiguration,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, RawPCM).with_sampling_config(config.into()))
}
