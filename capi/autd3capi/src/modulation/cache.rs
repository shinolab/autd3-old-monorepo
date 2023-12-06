/*
 * File: cache.rs
 * Project: modulation
 * Created Date: 21/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    autd3::modulation::{IntoCache, ModulationCache},
    *,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultCache {
    pub result: CachePtr,
    pub err_len: u32,
    pub err: ConstPtr,
}

impl From<Result<autd3capi_def::autd3::modulation::ModulationCache, AUTDInternalError>>
    for ResultCache
{
    fn from(
        r: Result<autd3capi_def::autd3::modulation::ModulationCache, AUTDInternalError>,
    ) -> Self {
        match r {
            Ok(v) => Self {
                result: CachePtr(Box::into_raw(Box::new(v)) as _),
                err_len: 0,
                err: std::ptr::null_mut(),
            },
            Err(e) => {
                let err = e.to_string();
                Self {
                    result: CachePtr(std::ptr::null()),
                    err_len: err.as_bytes().len() as u32 + 1,
                    err: Box::into_raw(Box::new(err)) as _,
                }
            }
        }
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithCache(m: ModulationPtr) -> ResultCache {
    Box::from_raw(m.0 as *mut Box<M>).with_cache().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheGetBufferLen(m: CachePtr) -> u32 {
    cast!(m.0, ModulationCache).buffer().len() as u32
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheGetBuffer(m: CachePtr, buf: *mut u8) {
    let cache = cast!(m.0, ModulationCache);
    std::ptr::copy_nonoverlapping(cache.buffer().as_ptr() as _, buf, cache.buffer().len());
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCacheIntoModulation(m: CachePtr) -> ModulationPtr {
    ModulationPtr::new(cast!(m.0, ModulationCache).clone())
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheDelete(m: CachePtr) {
    let _ = Box::from_raw(m.0 as *mut ModulationCache);
}
