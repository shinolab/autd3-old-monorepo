/*
 * File: lib.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::rc::Rc;

use autd3capi_def::common::dynamic_backend::DynamicBackend;
use autd3capi_def::{common::*, holo::*, take_gain, BackendPtr, GainPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDefaultBackend() -> BackendPtr {
    let backend: Box<Rc<dyn Backend>> = Box::new(NalgebraBackend::new());
    BackendPtr(Box::into_raw(backend) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteBackend(backend: BackendPtr) {
    let _ = Box::from_raw(backend.0 as *mut Rc<dyn Backend>);
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConstraintPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloDotCareConstraint() -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::DontCare)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNormalizeConstraint() -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Normalize)) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloUniformConstraint(value: float) -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Uniform(value))) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloClampConstraint(min_v: float, max_v: float) -> ConstraintPtr {
    ConstraintPtr(Box::into_raw(Box::new(Constraint::Clamp(min_v, max_v))) as _)
}

#[macro_export]
macro_rules! create_holo {
    ($type:tt, $backend:expr, $points:expr, $amps:expr, $size:expr) => {
        GainPtr::new(
            $type::new(DynamicBackend::new(
                ($backend.0 as *const Rc<dyn Backend>)
                    .as_ref()
                    .unwrap()
                    .clone(),
            ))
            .add_foci_from_iter((0..$size as usize).map(|i| {
                let p = Vector3::new(
                    $points.add(i * 3).read(),
                    $points.add(i * 3 + 1).read(),
                    $points.add(i * 3 + 2).read(),
                );
                let amp = *$amps.add(i);
                (p, amp)
            })),
        )
    };

    ($type:tt, $points:expr, $amps:expr, $size:expr) => {
        GainPtr::new(
            $type::new().add_foci_from_iter((0..$size as usize).map(|i| {
                let p = Vector3::new(
                    $points.add(i * 3).read(),
                    $points.add(i * 3 + 1).read(),
                    $points.add(i * 3 + 2).read(),
                );
                let amp = *$amps.add(i);
                (p, amp)
            })),
        )
    };
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDP(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(SDP, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, SDP<DynamicBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPWithAlpha(holo: GainPtr, alpha: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<DynamicBackend>).with_alpha(alpha))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPWithLambda(holo: GainPtr, lambda: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<DynamicBackend>).with_lambda(lambda))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDPWithRepeat(holo: GainPtr, repeat: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, SDP<DynamicBackend>).with_repeat(repeat as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloEVP(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(EVP, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloEVPWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, EVP<DynamicBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloEVPWithGamma(holo: GainPtr, gamma: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, EVP<DynamicBackend>).with_gamma(gamma))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGS(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(GS, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, GS<DynamicBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSWithRepeat(holo: GainPtr, repeat: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, GS<DynamicBackend>).with_repeat(repeat as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPAT(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(GSPAT, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, GSPAT<DynamicBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPATWithRepeat(holo: GainPtr, repeat: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, GSPAT<DynamicBackend>).with_repeat(repeat as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaive(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(Naive, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaiveWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, Naive<DynamicBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedy(
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(Greedy, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(take_gain!(holo, Greedy).with_constraint(*Box::from_raw(constraint.0 as _)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedyWithPhaseDiv(holo: GainPtr, div: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, Greedy).with_phase_div(div as _))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLM(
    backend: BackendPtr,
    points: *const float,
    amps: *const float,
    size: u64,
) -> GainPtr {
    create_holo!(LM, backend, points, amps, size)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithConstraint(
    holo: GainPtr,
    constraint: ConstraintPtr,
) -> GainPtr {
    GainPtr::new(
        take_gain!(holo, LM<DynamicBackend>).with_constraint(*Box::from_raw(constraint.0 as _)),
    )
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithEps1(holo: GainPtr, eps: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<DynamicBackend>).with_eps_1(eps))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithEps2(holo: GainPtr, eps: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<DynamicBackend>).with_eps_2(eps))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithTau(holo: GainPtr, tau: float) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<DynamicBackend>).with_tau(tau))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLMWithKMax(holo: GainPtr, k_max: u32) -> GainPtr {
    GainPtr::new(take_gain!(holo, LM<DynamicBackend>).with_k_max(k_max as _))
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
    GainPtr::new(take_gain!(holo, LM<DynamicBackend>).with_initial(initial))
}

#[cfg(test)]
mod tests {
    use std::ffi::{c_char, CStr};

    use super::*;

    use autd3capi::*;
    use autd3capi_def::{DatagramHeaderPtr, LinkPtr};

    #[test]
    fn holo_gain() {
        unsafe {
            let builder = AUTDCreateControllerBuilder();
            let builder = AUTDAddDevice(builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

            let link: Box<Box<L>> = Box::new(Box::new(NullLink {}));
            let link = LinkPtr(Box::into_raw(link) as ConstPtr);

            let mut err = vec![c_char::default(); 256];
            let cnt = AUTDControllerOpenWith(builder, link, err.as_mut_ptr());

            let backend = AUTDDefaultBackend();

            {
                let size = 2;
                let points = vec![10., 20., 30., 40., 50., 60.];
                let amps = vec![1.; size];

                let holo = AUTDGainHoloSDP(backend, points.as_ptr(), amps.as_ptr(), size as _);
                let constraint = AUTDGainHoloDotCareConstraint();
                let holo = AUTDGainHoloSDPWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloNormalizeConstraint();
                let holo = AUTDGainHoloSDPWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloUniformConstraint(1.);
                let holo = AUTDGainHoloSDPWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloClampConstraint(0., 1.);
                let holo = AUTDGainHoloSDPWithConstraint(holo, constraint);

                let holo = AUTDGainHoloSDPWithAlpha(holo, 1.);
                let holo = AUTDGainHoloSDPWithLambda(holo, 1.);
                let holo = AUTDGainHoloSDPWithRepeat(holo, 1);

                let holo = AUTDGainIntoDatagram(holo);

                if AUTDSend(
                    cnt,
                    autd3capi_def::TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == autd3capi_def::AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let backend = AUTDDefaultBackend();

                let size = 2;
                let points = vec![10., 20., 30., 40., 50., 60.];
                let amps = vec![1.; size];

                let holo = AUTDGainHoloEVP(backend, points.as_ptr(), amps.as_ptr(), size as _);
                let constraint = AUTDGainHoloDotCareConstraint();
                let holo = AUTDGainHoloEVPWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloNormalizeConstraint();
                let holo = AUTDGainHoloEVPWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloUniformConstraint(1.);
                let holo = AUTDGainHoloEVPWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloClampConstraint(0., 1.);
                let holo = AUTDGainHoloEVPWithConstraint(holo, constraint);

                let holo = AUTDGainHoloEVPWithGamma(holo, 1.);

                let holo = AUTDGainIntoDatagram(holo);

                if AUTDSend(
                    cnt,
                    autd3capi_def::TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == autd3capi_def::AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let backend = AUTDDefaultBackend();

                let size = 2;
                let points = vec![10., 20., 30., 40., 50., 60.];
                let amps = vec![1.; size];

                let holo = AUTDGainHoloNaive(backend, points.as_ptr(), amps.as_ptr(), size as _);
                let constraint = AUTDGainHoloDotCareConstraint();
                let holo = AUTDGainHoloNaiveWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloNormalizeConstraint();
                let holo = AUTDGainHoloNaiveWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloUniformConstraint(1.);
                let holo = AUTDGainHoloNaiveWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloClampConstraint(0., 1.);
                let holo = AUTDGainHoloNaiveWithConstraint(holo, constraint);

                let holo = AUTDGainIntoDatagram(holo);

                if AUTDSend(
                    cnt,
                    autd3capi_def::TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == autd3capi_def::AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let size = 2;
                let points = vec![10., 20., 30., 40., 50., 60.];
                let amps = vec![1.; size];

                let holo = AUTDGainHoloGS(backend, points.as_ptr(), amps.as_ptr(), size as _);
                let constraint = AUTDGainHoloDotCareConstraint();
                let holo = AUTDGainHoloGSWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloNormalizeConstraint();
                let holo = AUTDGainHoloGSWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloUniformConstraint(1.);
                let holo = AUTDGainHoloGSWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloClampConstraint(0., 1.);
                let holo = AUTDGainHoloGSWithConstraint(holo, constraint);

                let holo = AUTDGainHoloGSWithRepeat(holo, 100);

                let holo = AUTDGainIntoDatagram(holo);

                if AUTDSend(
                    cnt,
                    autd3capi_def::TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == autd3capi_def::AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let backend = AUTDDefaultBackend();

                let size = 2;
                let points = vec![10., 20., 30., 40., 50., 60.];
                let amps = vec![1.; size];

                let holo = AUTDGainHoloGSPAT(backend, points.as_ptr(), amps.as_ptr(), size as _);
                let constraint = AUTDGainHoloDotCareConstraint();
                let holo = AUTDGainHoloGSPATWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloNormalizeConstraint();
                let holo = AUTDGainHoloGSPATWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloUniformConstraint(1.);
                let holo = AUTDGainHoloGSPATWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloClampConstraint(0., 1.);
                let holo = AUTDGainHoloGSPATWithConstraint(holo, constraint);

                let holo = AUTDGainHoloGSPATWithRepeat(holo, 100);

                let holo = AUTDGainIntoDatagram(holo);

                if AUTDSend(
                    cnt,
                    autd3capi_def::TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == autd3capi_def::AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let backend = AUTDDefaultBackend();

                let size = 2;
                let points = vec![10., 20., 30., 40., 50., 60.];
                let amps = vec![1.; size];

                let holo = AUTDGainHoloLM(backend, points.as_ptr(), amps.as_ptr(), size as _);
                let constraint = AUTDGainHoloDotCareConstraint();
                let holo = AUTDGainHoloLMWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloNormalizeConstraint();
                let holo = AUTDGainHoloLMWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloUniformConstraint(1.);
                let holo = AUTDGainHoloLMWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloClampConstraint(0., 1.);
                let holo = AUTDGainHoloLMWithConstraint(holo, constraint);

                let holo = AUTDGainHoloLMWithEps1(holo, 1e-3);
                let holo = AUTDGainHoloLMWithEps2(holo, 1e-3);
                let holo = AUTDGainHoloLMWithTau(holo, 1e-3);
                let holo = AUTDGainHoloLMWithKMax(holo, 5);
                let initial = vec![0.; 1];
                let holo = AUTDGainHoloLMWithInitial(holo, initial.as_ptr(), initial.len() as _);

                let holo = AUTDGainIntoDatagram(holo);

                if AUTDSend(
                    cnt,
                    autd3capi_def::TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == autd3capi_def::AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            {
                let size = 2;
                let points = vec![10., 20., 30., 40., 50., 60.];
                let amps = vec![1.; size];

                let holo = AUTDGainHoloGreedy(points.as_ptr(), amps.as_ptr(), size as _);
                let constraint = AUTDGainHoloDotCareConstraint();
                let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloNormalizeConstraint();
                let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloUniformConstraint(1.);
                let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);
                let constraint = AUTDGainHoloClampConstraint(0., 1.);
                let holo = AUTDGainHoloGreedyWithConstraint(holo, constraint);

                let holo = AUTDGainHoloGreedyWithPhaseDiv(holo, 5);

                let holo = AUTDGainIntoDatagram(holo);

                if AUTDSend(
                    cnt,
                    autd3capi_def::TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == autd3capi_def::AUTD3_ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }
            }

            AUTDDeleteBackend(backend);
        }
    }
}
