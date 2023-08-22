/*
 * File: custom.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{common::autd3::core::gain::GainFilter, GainPtr};

use autd3capi_def::common::{
    core::{
        error::AUTDInternalError,
        geometry::{Geometry, Transducer},
        Drive,
    },
    traits::Gain,
    Gain,
};

#[derive(Gain)]
pub struct CustomGain {
    pub drives: Vec<Drive>,
}

impl<T: Transducer> autd3capi_def::common::core::gain::Gain<T> for CustomGain {
    fn calc(
        &self,
        _geometry: &Geometry<T>,
        _filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(self.drives.clone())
    }
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainCustom(ptr: *const Drive, len: u64) -> GainPtr {
    let mut drives = Vec::<autd3_core::Drive>::with_capacity(len as _);
    drives.set_len(len as _);
    std::ptr::copy_nonoverlapping(ptr as *const _, drives.as_mut_ptr(), len as _);
    GainPtr::new(CustomGain { drives })
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;
    use crate::{gain::*, tests::*, *};

    use autd3capi_def::{common::autd3::core::Drive, DatagramHeaderPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_custom_gain() {
        unsafe {
            let cnt = create_controller();

            let geo = AUTDGetGeometry(cnt);
            let num_transducers = AUTDNumTransducers(geo);

            let drives = vec![Drive { amp: 1., phase: 0. }; num_transducers as _];
            let g = AUTDGainCustom(drives.as_ptr(), drives.len() as _);
            let g = AUTDGainIntoDatagram(g);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );
        }
    }
}
