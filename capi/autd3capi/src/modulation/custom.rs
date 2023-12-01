/*
 * File: custom.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{CustomModulation, ModulationPtr, SamplingConfiguration};

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDModulationCustom(
    config: SamplingConfiguration,
    ptr: *const u8,
    len: u64,
) -> ModulationPtr {
    let mut buf = Vec::<autd3capi_def::driver::common::EmitIntensity>::with_capacity(len as _);
    buf.set_len(len as _);
    std::ptr::copy_nonoverlapping(ptr as _, buf.as_mut_ptr(), len as _);
    ModulationPtr::new(CustomModulation {
        config: config.into(),
        buf,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_custom_modulation() {
        unsafe {
            let cnt = create_controller();

            let buf = [0xFF, 0xFF];
            let m = AUTDModulationCustom(
                AUTDSamplingConfigFromFrequencyDivision(5120).result,
                buf.as_ptr(),
                buf.len() as _,
            );
            let m = AUTDModulationIntoDatagram(m);

            let r = AUTDControllerSend(cnt, m, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
