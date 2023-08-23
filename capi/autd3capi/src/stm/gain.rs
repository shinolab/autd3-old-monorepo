/*
 * File: gain.rs
 * Project: stm
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3_core::stm::STMProps;
use autd3capi_def::{common::*, DatagramBodyPtr, GainPtr, GainSTMMode, STMPropsPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMWithMode(
    props: STMPropsPtr,
    mode: GainSTMMode,
) -> DatagramBodyPtr {
    DatagramBodyPtr::new(
        GainSTM::<DynamicTransducer, Box<dyn Gain<DynamicTransducer>>>::with_props_mode(
            *Box::from_raw(props.0 as *mut STMProps),
            mode.into(),
        ),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTM(props: STMPropsPtr) -> DatagramBodyPtr {
    AUTDGainSTMWithMode(props, GainSTMMode::PhaseDutyFull)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMAddGain(
    stm: DatagramBodyPtr,
    gain: GainPtr,
) -> DatagramBodyPtr {
    DatagramBodyPtr::new(
        Box::from_raw(stm.0 as *mut Box<GainSTM<DynamicTransducer, _>>)
            .add_gain(*Box::from_raw(gain.0 as *mut Box<G>)),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::{gain::null::AUTDGainNull, stm::*, tests::*, *};

    #[test]
    fn test_gain_stm() {
        unsafe {
            let cnt = create_controller();

            let props = AUTDSTMProps(1.);

            let g0 = AUTDGainNull();
            let g1 = AUTDGainNull();

            let stm = AUTDGainSTM(props);
            let stm = AUTDGainSTMAddGain(stm, g0);
            let stm = AUTDGainSTMAddGain(stm, g1);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    stm,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }
}
