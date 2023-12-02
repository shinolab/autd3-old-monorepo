/*
 * File: gain.rs
 * Project: stm
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3_driver::datagram::STMProps;
use autd3capi_def::{driver::datagram::GainSTM, *};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGain(
    props: STMPropsPtr,
    gains: *const GainPtr,
    size: u32,
    mode: GainSTMMode,
) -> ResultDatagram {
    GainSTM::<Box<dyn Gain>>::from_props_mode(*Box::from_raw(props.0 as *mut STMProps), mode.into())
        .add_gains_from_iter(
            (0..size as usize).map(|i| *Box::from_raw(gains.add(i).read().0 as *mut Box<G>)),
        )
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMGainAddGain(stm: DatagramPtr, gain: GainPtr) -> ResultDatagram {
    Box::from_raw(stm.0 as *mut Box<GainSTM<_>>)
        .add_gain(*Box::from_raw(gain.0 as *mut Box<G>))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{gain::null::AUTDGainNull, stm::*, tests::*, *};
    use autd3capi_def::{GainSTMMode, AUTD3_TRUE};

    #[test]
    fn test_gain_stm() {
        unsafe {
            let cnt = create_controller();

            let props = AUTDSTMPropsNew(1.);

            let g0 = AUTDGainNull();
            let g1 = AUTDGainNull();

            let gains = [g0, g1];

            let stm = AUTDSTMGain(
                props,
                gains.as_ptr(),
                gains.len() as u32,
                GainSTMMode::PhaseIntensityFull,
            )
            .result;

            let r = AUTDControllerSend(cnt, stm, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
