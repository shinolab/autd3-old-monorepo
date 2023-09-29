/*
 * File: lm.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 29/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::rc::Rc;

use autd3capi_def::{
    common::*, create_holo, holo::*, take_gain, BackendPtr, ConstraintPtr, GainPtr,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLM(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(LM, NalgebraBackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, LM<NalgebraBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithEps1(holo: GainPtr, eps: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<NalgebraBackend>).with_eps_1(eps))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithEps2(holo: GainPtr, eps: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<NalgebraBackend>).with_eps_2(eps))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithTau(holo: GainPtr, tau: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<NalgebraBackend>).with_tau(tau))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithKMax(holo: GainPtr, k_max: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<NalgebraBackend>).with_k_max(k_max as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithInitial(
    holo: GainPtr,
    initial_ptr: *const float,
    len: u64,
) -> GainPtr {
    let mut initial = vec![0.; len as usize];
    std::ptr::copy_nonoverlapping(initial_ptr, initial.as_mut_ptr(), len as usize);
    GainPtr::new(take_gain!(holo, LM<NalgebraBackend>).with_initial(initial))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{constraint::*, nalgebra_backend::*};

    #[test]
    fn test_holo_lm() {
        unsafe {
            let backend = AUTDNalgebraBackend();

            let size = 2;
            let points = [10., 20., 30., 40., 50., 60.];
            let amps = vec![1.; size];

            let holo = AUTDGainHoloLM(backend, points.as_ptr(), amps.as_ptr(), size as _);
            let constraint = AUTDGainHoloConstraintDotCare();
            let holo = AUTDGainHoloLMWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintNormalize();
            let holo = AUTDGainHoloLMWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintUniform(1.);
            let holo = AUTDGainHoloLMWithConstraint(holo, constraint);
            let constraint = AUTDGainHoloConstraintClamp(0., 1.);
            let holo = AUTDGainHoloLMWithConstraint(holo, constraint);

            let holo = AUTDGainHoloLMWithEps1(holo, 1e-3);
            let holo = AUTDGainHoloLMWithEps2(holo, 1e-3);
            let holo = AUTDGainHoloLMWithTau(holo, 1e-3);
            let holo = AUTDGainHoloLMWithKMax(holo, 5);
            let initial = [0.; 1];
            let _ = AUTDGainHoloLMWithInitial(holo, initial.as_ptr(), initial.len() as _);

            AUTDDeleteNalgebraBackend(backend);
        }
    }
}
