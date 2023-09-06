/*
 * File: gain.rs
 * Project: stm
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3::driver::datagram::STMProps;
use autd3capi_def::{common::*, DatagramPtr, GainPtr, GainSTMMode, STMPropsPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTM(
    props: STMPropsPtr,
    gains: *const GainPtr,
    size: u32,
    mode: GainSTMMode,
) -> DatagramPtr {
    DatagramPtr::new(
        GainSTM::<DynamicTransducer, Box<dyn Gain<DynamicTransducer>>>::with_props_mode(
            *Box::from_raw(props.0 as *mut STMProps),
            mode.into(),
        )
        .add_gains_from_iter(
            (0..size as usize).map(|i| *Box::from_raw(gains.add(i).read().0 as *mut Box<G>)),
        ),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainSTMAddGain(stm: DatagramPtr, gain: GainPtr) -> DatagramPtr {
    DatagramPtr::new(
        Box::from_raw(stm.0 as *mut Box<GainSTM<DynamicTransducer, _>>)
            .add_gain(*Box::from_raw(gain.0 as *mut Box<G>)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{gain::null::AUTDGainNull, stm::*, tests::*, TransMode, *};
    use autd3capi_def::GainSTMMode;

    #[test]
    fn test_gain_stm() {
        unsafe {
            let cnt = create_controller();

            let props = AUTDSTMProps(1.);

            let g0 = AUTDGainNull();
            let g1 = AUTDGainNull();

            let gains = [g0, g1];

            let stm = AUTDGainSTM(
                props,
                gains.as_ptr(),
                gains.len() as u32,
                GainSTMMode::PhaseDutyFull,
            );

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    stm,
                    DatagramPtr(std::ptr::null()),
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }
}
