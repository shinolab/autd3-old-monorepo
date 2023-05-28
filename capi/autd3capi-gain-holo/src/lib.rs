#![allow(clippy::missing_safety_doc)]

mod backend;

use autd3_gain_holo::*;
use autd3capi_common::*;
use backend::DynamicBackend;

macro_rules! holo_cast {
    ($holo:ident, $ty:ty) => {
        (($holo as *mut Box<G> as *mut Box<GainWrap>)
            .as_mut()
            .unwrap()
            .gain_mut() as *mut _ as *mut $ty)
            .as_mut()
            .unwrap()
    };
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDDefaultBackend() -> ConstPtr {
    let backend: Box<Box<dyn Backend>> = Box::new(Box::new(NalgebraBackend::new()));
    Box::into_raw(backend) as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloSDP(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(GainWrap::new(SDP::new(DynamicBackend::new(
            *Box::from_raw(backend as _),
        )))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPAlpha(holo: ConstPtr, alpha: float) {
    unsafe {
        holo_cast!(holo, SDP<DynamicBackend>).alpha = alpha;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPLambda(holo: ConstPtr, lambda: float) {
    unsafe {
        holo_cast!(holo, SDP<DynamicBackend>).lambda = lambda;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPRepeat(holo: ConstPtr, repeat: u32) {
    unsafe {
        holo_cast!(holo, SDP<DynamicBackend>).repeat = repeat as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPAdd(
    holo: ConstPtr,
    x: float,
    y: float,
    z: float,
    amp: float,
) {
    unsafe {
        holo_cast!(holo, SDP<DynamicBackend>).add_focus(Vector3::new(x, y, z), amp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPSetDotCareConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, SDP<DynamicBackend>).set_constraint(Constraint::DontCare);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPSetNormalizeConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, SDP<DynamicBackend>).set_constraint(Constraint::Normalize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPSetUniformConstraint(holo: ConstPtr, value: float) {
    unsafe {
        holo_cast!(holo, SDP<DynamicBackend>).set_constraint(Constraint::Uniform(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPSetClampConstraint(holo: ConstPtr, min: float, max: float) {
    unsafe {
        holo_cast!(holo, SDP<DynamicBackend>).set_constraint(Constraint::Clamp(min, max));
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloEVP(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(GainWrap::new(EVP::new(DynamicBackend::new(
            *Box::from_raw(backend as _),
        )))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloEVPGamma(holo: ConstPtr, gamma: float) {
    unsafe {
        holo_cast!(holo, EVP<DynamicBackend>).gamma = gamma;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloEVPAdd(
    holo: ConstPtr,
    x: float,
    y: float,
    z: float,
    amp: float,
) {
    unsafe {
        holo_cast!(holo, EVP<DynamicBackend>).add_focus(Vector3::new(x, y, z), amp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloEVPSetDotCareConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, EVP<DynamicBackend>).set_constraint(Constraint::DontCare);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloEVPSetNormalizeConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, EVP<DynamicBackend>).set_constraint(Constraint::Normalize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloEVPSetUniformConstraint(holo: ConstPtr, value: float) {
    unsafe {
        holo_cast!(holo, EVP<DynamicBackend>).set_constraint(Constraint::Uniform(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloEVPSetClampConstraint(holo: ConstPtr, min: float, max: float) {
    unsafe {
        holo_cast!(holo, EVP<DynamicBackend>).set_constraint(Constraint::Clamp(min, max));
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGS(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(GainWrap::new(GS::new(DynamicBackend::new(*Box::from_raw(
            backend as _,
        ))))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSRepeat(holo: ConstPtr, repeat: u32) {
    unsafe {
        holo_cast!(holo, GS<DynamicBackend>).repeat = repeat as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSAdd(
    holo: ConstPtr,
    x: float,
    y: float,
    z: float,
    amp: float,
) {
    unsafe {
        holo_cast!(holo, GS<DynamicBackend>).add_focus(Vector3::new(x, y, z), amp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSSetDotCareConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, GS<DynamicBackend>).set_constraint(Constraint::DontCare);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSSetNormalizeConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, GS<DynamicBackend>).set_constraint(Constraint::Normalize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSSetUniformConstraint(holo: ConstPtr, value: float) {
    unsafe {
        holo_cast!(holo, GS<DynamicBackend>).set_constraint(Constraint::Uniform(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSSetClampConstraint(holo: ConstPtr, min: float, max: float) {
    unsafe {
        holo_cast!(holo, GS<DynamicBackend>).set_constraint(Constraint::Clamp(min, max));
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPAT(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(GainWrap::new(GSPAT::new(DynamicBackend::new(
            *Box::from_raw(backend as _),
        )))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSPATRepeat(holo: ConstPtr, repeat: u32) {
    unsafe {
        holo_cast!(holo, GSPAT<DynamicBackend>).repeat = repeat as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSPATAdd(
    holo: ConstPtr,
    x: float,
    y: float,
    z: float,
    amp: float,
) {
    unsafe {
        holo_cast!(holo, GSPAT<DynamicBackend>).add_focus(Vector3::new(x, y, z), amp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSPATSetDotCareConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, GSPAT<DynamicBackend>).set_constraint(Constraint::DontCare);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSPATSetNormalizeConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, GSPAT<DynamicBackend>).set_constraint(Constraint::Normalize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSPATSetUniformConstraint(holo: ConstPtr, value: float) {
    unsafe {
        holo_cast!(holo, GSPAT<DynamicBackend>).set_constraint(Constraint::Uniform(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSPATSetClampConstraint(
    holo: ConstPtr,
    min: float,
    max: float,
) {
    unsafe {
        holo_cast!(holo, GSPAT<DynamicBackend>).set_constraint(Constraint::Clamp(min, max));
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaive(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(GainWrap::new(Naive::new(DynamicBackend::new(
            *Box::from_raw(backend as _),
        )))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloNaiveAdd(
    holo: ConstPtr,
    x: float,
    y: float,
    z: float,
    amp: float,
) {
    unsafe {
        holo_cast!(holo, Naive<DynamicBackend>).add_focus(Vector3::new(x, y, z), amp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloNaiveSetDotCareConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, Naive<DynamicBackend>).set_constraint(Constraint::DontCare);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloNaiveSetNormalizeConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, Naive<DynamicBackend>).set_constraint(Constraint::Normalize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloNaiveSetUniformConstraint(holo: ConstPtr, value: float) {
    unsafe {
        holo_cast!(holo, Naive<DynamicBackend>).set_constraint(Constraint::Uniform(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloNaiveSetClampConstraint(
    holo: ConstPtr,
    min: float,
    max: float,
) {
    unsafe {
        holo_cast!(holo, Naive<DynamicBackend>).set_constraint(Constraint::Clamp(min, max));
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedy() -> ConstPtr {
    Box::into_raw(GainWrap::new(Greedy::new())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGreedyPhaseDiv(holo: ConstPtr, div: u32) {
    unsafe {
        holo_cast!(holo, Greedy).phase_div = div as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGreedyAdd(
    holo: ConstPtr,
    x: float,
    y: float,
    z: float,
    amp: float,
) {
    unsafe {
        holo_cast!(holo, Greedy).add_focus(Vector3::new(x, y, z), amp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGreedySetDotCareConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, Greedy).set_constraint(Constraint::DontCare);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGreedySetNormalizeConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, Greedy).set_constraint(Constraint::Normalize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGreedySetUniformConstraint(holo: ConstPtr, value: float) {
    unsafe {
        holo_cast!(holo, Greedy).set_constraint(Constraint::Uniform(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGreedySetClampConstraint(
    holo: ConstPtr,
    min: float,
    max: float,
) {
    unsafe {
        holo_cast!(holo, Greedy).set_constraint(Constraint::Clamp(min, max));
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLM(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(GainWrap::new(LM::new(DynamicBackend::new(*Box::from_raw(
            backend as _,
        ))))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMEps1(holo: ConstPtr, eps_1: float) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).eps_1 = eps_1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMEps2(holo: ConstPtr, eps_2: float) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).eps_2 = eps_2;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMTau(holo: ConstPtr, tau: float) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).tau = tau;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMKMax(holo: ConstPtr, k_max: u32) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).k_max = k_max as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMInitial(holo: ConstPtr, ptr: *const float, len: u64) {
    unsafe {
        let mut initial = vec![0.; len as _];
        std::ptr::copy_nonoverlapping(ptr, initial.as_mut_ptr(), len as _);
        holo_cast!(holo, LM<DynamicBackend>).initial = initial;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMAdd(
    holo: ConstPtr,
    x: float,
    y: float,
    z: float,
    amp: float,
) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).add_focus(Vector3::new(x, y, z), amp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMSetDotCareConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).set_constraint(Constraint::DontCare);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMSetNormalizeConstraint(holo: ConstPtr) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).set_constraint(Constraint::Normalize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMSetUniformConstraint(holo: ConstPtr, value: float) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).set_constraint(Constraint::Uniform(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMSetClampConstraint(holo: ConstPtr, min: float, max: float) {
    unsafe {
        holo_cast!(holo, LM<DynamicBackend>).set_constraint(Constraint::Clamp(min, max));
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::{c_char, CStr};

    use super::*;

    use autd3capi::*;

    #[test]
    fn holo_gain() {
        unsafe {
            let geo_builder = AUTDCreateGeometryBuilder();
            AUTDAddDevice(geo_builder, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let mut err = vec![c_char::default(); 256];
            let geometry = AUTDBuildGeometry(geo_builder, err.as_mut_ptr());

            let link: Box<Box<L>> = Box::new(Box::new(NullLink {}));
            let link = Box::into_raw(link) as ConstPtr;

            let cnt = AUTDOpenController(geometry, link, err.as_mut_ptr());

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloSDP(backend);
                AUTDGainHoloSDPAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloSDPSetDotCareConstraint(holo);
                AUTDGainHoloSDPSetNormalizeConstraint(holo);
                AUTDGainHoloSDPSetUniformConstraint(holo, 1.);
                AUTDGainHoloSDPSetClampConstraint(holo, 0., 1.);
                AUTDGainHoloSDPAlpha(holo, 1.);
                AUTDGainHoloSDPLambda(holo, 1.);
                AUTDGainHoloSDPRepeat(holo, 1);

                if AUTDSend(
                    cnt,
                    autd3capi::TransMode::Legacy,
                    std::ptr::null(),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGain(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloEVP(backend);
                AUTDGainHoloEVPAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloEVPSetDotCareConstraint(holo);
                AUTDGainHoloEVPSetNormalizeConstraint(holo);
                AUTDGainHoloEVPSetUniformConstraint(holo, 1.);
                AUTDGainHoloEVPSetClampConstraint(holo, 0., 1.);
                AUTDGainHoloEVPGamma(holo, 1.);

                if AUTDSend(
                    cnt,
                    autd3capi::TransMode::Legacy,
                    std::ptr::null(),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGain(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloNaive(backend);
                AUTDGainHoloNaiveAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloNaiveSetDotCareConstraint(holo);
                AUTDGainHoloNaiveSetNormalizeConstraint(holo);
                AUTDGainHoloNaiveSetUniformConstraint(holo, 1.);
                AUTDGainHoloNaiveSetClampConstraint(holo, 0., 1.);
                if AUTDSend(
                    cnt,
                    autd3capi::TransMode::Legacy,
                    std::ptr::null(),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGain(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloGS(backend);
                AUTDGainHoloGSAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloGSSetDotCareConstraint(holo);
                AUTDGainHoloGSSetNormalizeConstraint(holo);
                AUTDGainHoloGSSetUniformConstraint(holo, 1.);
                AUTDGainHoloGSSetClampConstraint(holo, 0., 1.);
                AUTDGainHoloGSRepeat(holo, 1);

                if AUTDSend(
                    cnt,
                    autd3capi::TransMode::Legacy,
                    std::ptr::null(),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGain(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloGSPAT(backend);
                AUTDGainHoloGSPATAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloGSPATSetDotCareConstraint(holo);
                AUTDGainHoloGSPATSetNormalizeConstraint(holo);
                AUTDGainHoloGSPATSetUniformConstraint(holo, 1.);
                AUTDGainHoloGSPATSetClampConstraint(holo, 0., 1.);
                AUTDGainHoloGSPATRepeat(holo, 1);

                if AUTDSend(
                    cnt,
                    autd3capi::TransMode::Legacy,
                    std::ptr::null(),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGain(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloLM(backend);
                AUTDGainHoloLMAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloLMSetDotCareConstraint(holo);
                AUTDGainHoloLMSetNormalizeConstraint(holo);
                AUTDGainHoloLMSetUniformConstraint(holo, 1.);
                AUTDGainHoloLMSetClampConstraint(holo, 0., 1.);
                AUTDGainHoloLMEps1(holo, 1.);
                AUTDGainHoloLMEps2(holo, 1.);
                AUTDGainHoloLMTau(holo, 1.);
                AUTDGainHoloLMKMax(holo, 1);
                let init = vec![0.; 1];
                AUTDGainHoloLMInitial(holo, init.as_ptr(), init.len() as _);

                if AUTDSend(
                    cnt,
                    autd3capi::TransMode::Legacy,
                    std::ptr::null(),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGain(holo);
            }

            {
                let holo = AUTDGainHoloGreedy();
                AUTDGainHoloGreedyAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloGreedySetDotCareConstraint(holo);
                AUTDGainHoloGreedySetNormalizeConstraint(holo);
                AUTDGainHoloGreedySetUniformConstraint(holo, 1.);
                AUTDGainHoloGreedySetClampConstraint(holo, 0., 1.);
                AUTDGainHoloGreedyPhaseDiv(holo, 1);

                if AUTDSend(
                    cnt,
                    autd3capi::TransMode::Legacy,
                    std::ptr::null(),
                    holo,
                    -1,
                    err.as_mut_ptr(),
                ) == ERR
                {
                    eprintln!("{}", CStr::from_ptr(err.as_ptr()).to_str().unwrap());
                }

                AUTDDeleteGain(holo);
            }
        }
    }
}
