/*
 * File: cache.rs
 * Project: modulation
 * Created Date: 21/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::c_char;

use autd3capi_def::{
    common::{autd3::modulation::ModulationCache, *},
    ModulationPtr,
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ModulationCachePtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithCache(
    m: ModulationPtr,
    err: *mut c_char,
) -> ModulationCachePtr {
    try_or_return!(
        Box::from_raw(m.0 as *mut Box<M>)
            .with_cache()
            .and_then(|m| Ok(ModulationCachePtr(Box::into_raw(Box::new(m)) as _))),
        err,
        ModulationCachePtr(std::ptr::null())
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCacheGetBufferSize(m: ModulationCachePtr) -> u32 {
    cast!(m.0, ModulationCache).buffer().len() as u32
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheGetBuffer(m: ModulationCachePtr, buf: *mut float) {
    let cache = cast!(m.0, ModulationCache);
    std::ptr::copy_nonoverlapping(cache.buffer().as_ptr(), buf, cache.buffer().len());
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCacheIntoModulation(m: ModulationCachePtr) -> ModulationPtr {
    ModulationPtr::new(cast!(m.0, ModulationCache).clone())
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheDelete(m: ModulationCachePtr) {
    let _ = Box::from_raw(m.0 as *mut ModulationCache);
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::{super::sine::AUTDModulationSine, *};

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_modulation_cache() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSine(150);

            let mut err = vec![c_char::default(); 256];
            let cache = AUTDModulationWithCache(m, err.as_mut_ptr());
            assert!(!cache.0.is_null());
            let m = AUTDModulationIntoDatagram(AUTDModulationCacheIntoModulation(cache));

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDModulationCacheDelete(cache);
            AUTDFreeController(cnt);
        }
    }
}
