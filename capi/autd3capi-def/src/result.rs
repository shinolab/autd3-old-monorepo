/*
 * File: result.rs
 * Project: src
 * Created Date: 10/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, ffi::c_char};

use autd3capi_common::{libc, AUTDError, AUTDInternalError, ConstPtr, Controller, Drive, L, NULL};

use crate::{ControllerPtr, ModulationPtr, AUTD3_ERR, AUTD3_FALSE, AUTD3_TRUE};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultI32 {
    pub result: i32,
    pub err_len: u32,
    pub err: *const c_char,
}

impl From<Result<(), AUTDInternalError>> for ResultI32 {
    fn from(r: Result<(), AUTDInternalError>) -> Self {
        match r {
            Ok(_) => Self {
                result: AUTD3_TRUE,
                err_len: 0,
                err: std::ptr::null(),
            },
            Err(e) => {
                let err = std::ffi::CString::new(e.to_string()).unwrap();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes_with_nul().len() as u32,
                    err: err.into_raw(),
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
                err: std::ptr::null(),
            },
            Err(e) => {
                let err = std::ffi::CString::new(e.to_string()).unwrap();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes_with_nul().len() as u32,
                    err: err.into_raw(),
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
                err: std::ptr::null(),
            },
            Err(e) => {
                let err = std::ffi::CString::new(e.to_string()).unwrap();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes_with_nul().len() as u32,
                    err: err.into_raw(),
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
                err: std::ptr::null(),
            },
            Err(e) => {
                let err = std::ffi::CString::new(e.to_string()).unwrap();
                Self {
                    result: AUTD3_ERR,
                    err_len: err.as_bytes_with_nul().len() as u32,
                    err: err.into_raw(),
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDResultI32GetErr(r: ResultI32, err: *mut c_char) {
    let err_ = std::ffi::CString::from_raw(r.err as *mut c_char);
    libc::strcpy(err, err_.as_ptr());
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultControllerPtr {
    pub result: ControllerPtr,
    pub err_len: u32,
    pub err: *const c_char,
}

impl From<Result<Controller<Box<L>>, AUTDError>> for ResultControllerPtr {
    fn from(r: Result<Controller<Box<L>>, AUTDError>) -> Self {
        match r {
            Ok(v) => Self {
                result: ControllerPtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null(),
            },
            Err(e) => {
                let err = std::ffi::CString::new(e.to_string()).unwrap();
                Self {
                    result: ControllerPtr(NULL),
                    err_len: err.as_bytes_with_nul().len() as u32,
                    err: err.into_raw(),
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDResultControllerPtrGetErr(r: ResultControllerPtr, err: *mut c_char) {
    let err_ = std::ffi::CString::from_raw(r.err as *mut c_char);
    libc::strcpy(err, err_.as_ptr());
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultGainCalcDrivesMapPtr {
    pub result: ConstPtr,
    pub err_len: u32,
    pub err: *const c_char,
}

impl From<Result<HashMap<usize, Vec<Drive>>, AUTDInternalError>> for ResultGainCalcDrivesMapPtr {
    fn from(r: Result<HashMap<usize, Vec<Drive>>, AUTDInternalError>) -> Self {
        match r {
            Ok(v) => Self {
                result: Box::into_raw(Box::new(v)) as _,
                err_len: 0,
                err: std::ptr::null(),
            },
            Err(e) => {
                let err = std::ffi::CString::new(e.to_string()).unwrap();
                Self {
                    result: NULL,
                    err_len: err.as_bytes_with_nul().len() as u32,
                    err: err.into_raw(),
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDResultGainCalcDrivesMapPtrGetErr(
    r: ResultGainCalcDrivesMapPtr,
    err: *mut c_char,
) {
    let err_ = std::ffi::CString::from_raw(r.err as *mut c_char);
    libc::strcpy(err, err_.as_ptr());
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultModulationPtr {
    pub result: ModulationPtr,
    pub err_len: u32,
    pub err: *const c_char,
}

#[no_mangle]
pub unsafe extern "C" fn AUTDResultModulationPtrGetErr(r: ResultModulationPtr, err: *mut c_char) {
    let err_ = std::ffi::CString::from_raw(r.err as *mut c_char);
    libc::strcpy(err, err_.as_ptr());
}
