/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3_backend_cuda::*;
use autd3capi_def::{
    common::*, create_holo, holo::*, take_gain, BackendPtr, ConstraintPtr, GainPtr,
};
use std::{ffi::c_char, rc::Rc};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDCUDABackend(err: *mut c_char) -> BackendPtr {
    match CUDABackend::new() {
        Ok(b) => BackendPtr(Box::into_raw(Box::new(b)) as _),
        Err(e) => {
            let msg = std::ffi::CString::new(e.to_string()).unwrap();
            libc::strcpy(err, msg.as_ptr());
            BackendPtr(std::ptr::null_mut())
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDCUDABackendDelete(backend: BackendPtr) {
    let _ = Box::from_raw(backend.0 as *mut Rc<CUDABackend>);
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDASDP(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(SDP, CUDABackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDASDPWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, SDP<CUDABackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDASDPWithAlpha(holo: GainPtr, alpha: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<CUDABackend>).with_alpha(alpha))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDASDPWithLambda(holo: GainPtr, lambda: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<CUDABackend>).with_lambda(lambda))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDASDPWithRepeat(holo: GainPtr, repeat: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<CUDABackend>).with_repeat(repeat as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAEVP(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(EVP, CUDABackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAEVPWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, EVP<CUDABackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAEVPWithGamma(holo: GainPtr, gamma: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, EVP<CUDABackend>).with_gamma(gamma))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGS(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(GS, CUDABackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGSWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, GS<CUDABackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGSWithRepeat(holo: GainPtr, repeat: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, GS<CUDABackend>).with_repeat(repeat as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGSPAT(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(GSPAT, CUDABackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGSPATWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, GSPAT<CUDABackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGSPATWithRepeat(holo: GainPtr, repeat: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, GSPAT<CUDABackend>).with_repeat(repeat as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDANaive(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(Naive, CUDABackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDANaiveWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, Naive<CUDABackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGreedy(
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(Greedy, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGreedyWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(take_gain!(holo, Greedy).with_constraint(*Box::from_raw(constraint.0 as _)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDAGreedyWithPhaseDiv(holo: GainPtr, div: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, Greedy).with_phase_div(div as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALM(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(LM, CUDABackend, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALMWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, LM<CUDABackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALMWithEps1(holo: GainPtr, eps: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<CUDABackend>).with_eps_1(eps))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALMWithEps2(holo: GainPtr, eps: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<CUDABackend>).with_eps_2(eps))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALMWithTau(holo: GainPtr, tau: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<CUDABackend>).with_tau(tau))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALMWithKMax(holo: GainPtr, k_max: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<CUDABackend>).with_k_max(k_max as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloCUDALMWithInitial(
    holo: GainPtr,
    initial_ptr: *const float,
    len: u64,
) -> GainPtr {
    let mut initial = vec![0.; len as usize];
    std::ptr::copy_nonoverlapping(initial_ptr, initial.as_mut_ptr(), len as usize);
    GainPtr::new(take_gain!(holo, LM<CUDABackend>).with_initial(initial))
}
