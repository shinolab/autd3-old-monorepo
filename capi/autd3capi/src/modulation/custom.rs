/*
 * File: custom.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::ModulationPtr;

use autd3capi_def::common::{
    core::error::AUTDInternalError, float, traits::Modulation, Modulation,
};

#[derive(Modulation)]
pub struct CustomModulation {
    pub buf: Vec<float>,
    pub freq_div: u32,
}

impl autd3capi_def::common::core::modulation::Modulation for CustomModulation {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustom(
    freq_div: u32,
    ptr: *const float,
    len: u64,
) -> ModulationPtr {
    let mut buf = Vec::<float>::with_capacity(len as _);
    buf.set_len(len as _);
    std::ptr::copy_nonoverlapping(ptr, buf.as_mut_ptr(), len as _);
    ModulationPtr::new(CustomModulation { freq_div, buf })
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramBodyPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_custom_modulation() {
        unsafe {
            let cnt = create_controller();

            let buf = vec![1., 1.];
            let m = AUTDModulationCustom(5000, buf.as_ptr(), buf.len() as _);
            let m = AUTDModulationIntoDatagram(m);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    m,
                    DatagramBodyPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }
}
