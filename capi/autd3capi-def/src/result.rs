/*
 * File: result.rs
 * Project: src
 * Created Date: 10/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
};

use autd3capi_common::{
    driver::common::Drive, libc, AUTDError, AUTDInternalError, ConstPtr, Controller,
    DynamicDatagram, L, NULL,
};

use crate::{
    BackendPtr, ControllerPtr, DatagramPtr, GainCalcDrivesMapPtr, ModulationPtr,
    SamplingConfiguration, AUTD3_ERR, AUTD3_FALSE, AUTD3_TRUE,
};

#[no_mangle]
pub unsafe extern "C" fn AUTDGetErr(src: ConstPtr, dst: *mut c_char) {
    let src = Box::from_raw(src as *mut String);
    let c_string: CString = CString::new(src.as_str()).unwrap();
    let c_str: &CStr = c_string.as_c_str();
    libc::strcpy(dst, c_str.as_ptr());
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultI32 {
    pub result: i32,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<(), AUTDInternalError>> for ResultI32 {
    fn from(r: Result<(), AUTDInternalError>) -> Self {
        match r {
            Ok(_) => Self {
                result: AUTD3_TRUE,
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

impl From<Result<bool, AUTDError>> for ResultI32 {
    fn from(r: Result<bool, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: if v { AUTD3_TRUE } else { AUTD3_FALSE },
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

impl From<Result<bool, AUTDInternalError>> for ResultI32 {
    fn from(r: Result<bool, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: if v { AUTD3_TRUE } else { AUTD3_FALSE },
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

impl From<Result<usize, AUTDInternalError>> for ResultI32 {
    fn from(r: Result<usize, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: v as i32,
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultSamplingConfig {
    pub result: SamplingConfiguration,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<autd3capi_common::driver::common::SamplingConfiguration, AUTDInternalError>>
    for ResultSamplingConfig
{
    fn from(
        r: Result<autd3capi_common::driver::common::SamplingConfiguration, AUTDInternalError>,
    ) -> Self {
        match r {
            Ok(result) => Self {
                result: result.into(),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: SamplingConfiguration { div: 0 },
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultController {
    pub result: ControllerPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<Controller<Box<L>>, AUTDError>> for ResultController {
    fn from(r: Result<Controller<Box<L>>, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: ControllerPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: ControllerPtr(NULL),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultGainCalcDrivesMap {
    pub result: GainCalcDrivesMapPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<HashMap<usize, Vec<Drive>>, AUTDInternalError>> for ResultGainCalcDrivesMap {
    fn from(r: Result<HashMap<usize, Vec<Drive>>, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: GainCalcDrivesMapPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: GainCalcDrivesMapPtr(NULL),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultModulation {
    pub result: ModulationPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultBackend {
    pub result: BackendPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultDatagramPtr {
    pub result: DatagramPtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl<T: DynamicDatagram> From<Result<T, AUTDInternalError>> for ResultDatagramPtr {
    fn from(r: Result<T, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: DatagramPtr::new(v),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: DatagramPtr(NULL),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}
