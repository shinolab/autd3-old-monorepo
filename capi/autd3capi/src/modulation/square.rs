/*
 * File: square.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{
    common::{autd3::modulation::Square, *},
    take_mod, ModulationPtr, SamplingConfiguration,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquare(freq: u32) -> ModulationPtr {
    ModulationPtr::new(Square::new(freq as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithLow(m: ModulationPtr, low: u8) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_low(low))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithHigh(m: ModulationPtr, high: u8) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_high(high))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithDuty(
    m: ModulationPtr,
    duty: float,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_duty(duty))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithSamplingConfig(
    m: ModulationPtr,
    config: SamplingConfiguration,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_sampling_config(config.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_square() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSquare(150);
            let m = AUTDModulationSquareWithLow(m, 0);
            let m = AUTDModulationSquareWithHigh(m, 0xFF);
            let m = AUTDModulationSquareWithDuty(m, 0.5);
            let div = 10240;
            let m = AUTDModulationSquareWithSamplingConfig(
                m,
                AUTDSamplingConfigNewWithFrequencyDivision(div).result,
            );

            let m = AUTDModulationIntoDatagram(m);

            let r = AUTDControllerSend(cnt, m, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
