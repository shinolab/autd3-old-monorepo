#![allow(clippy::missing_safety_doc)]

mod backend;

use std::time::Duration;

use autd3::core::GainOp;
use autd3_gain_holo::*;
use autd3capi_common::{dynamic_transducer::TransMode, *};
use backend::DynamicBackend;

pub type HG = dyn DynamicHolo;

pub trait DynamicHolo: DynamicDatagram {}

struct HoloWrap {
    pub holo: Box<dyn Holo<DynamicTransducer>>,
}

impl HoloWrap {
    pub fn cast<H>(&mut self) -> &mut H {
        let h: &mut dyn Holo<DynamicTransducer> = &mut *self.holo;
        unsafe { (h as *mut _ as *mut H).as_mut().unwrap() }
    }
}

impl HoloWrap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<H: 'static + Holo<DynamicTransducer>>(holo: H) -> Box<Box<HG>> {
        Box::new(Box::new(HoloWrap {
            holo: Box::new(holo),
        }))
    }
}

impl DynamicHolo for HoloWrap {}

impl DynamicDatagram for HoloWrap {
    fn operation(
        &mut self,
        mode: TransMode,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        autd3::core::error::AUTDInternalError,
    > {
        match mode {
            TransMode::Legacy => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::GainLegacy::new(
                    self.holo.calc(geometry)?,
                    || geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
            TransMode::Advanced => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::GainAdvanced::new(
                    self.holo.calc(geometry)?,
                    || geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
            TransMode::AdvancedPhase => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::GainAdvancedPhase::new(
                    self.holo.calc(geometry)?,
                    || geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
        }
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
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
        Box::into_raw(HoloWrap::new(SDP::new(DynamicBackend::new(
            *Box::from_raw(backend as _),
        )))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPAlpha(holo: ConstPtr, alpha: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<SDP<DynamicBackend>>()
            .alpha = alpha;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPLambda(holo: ConstPtr, lambda: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<SDP<DynamicBackend>>()
            .lambda = lambda;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDPRepeat(holo: ConstPtr, repeat: u32) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<SDP<DynamicBackend>>()
            .repeat = repeat as _;
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloEVP(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(HoloWrap::new(EVP::new(DynamicBackend::new(
            *Box::from_raw(backend as _),
        )))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloEVPGamma(holo: ConstPtr, gamma: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<EVP<DynamicBackend>>()
            .gamma = gamma;
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGS(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(HoloWrap::new(GS::new(DynamicBackend::new(*Box::from_raw(
            backend as _,
        ))))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSRepeat(holo: ConstPtr, repeat: u32) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<GS<DynamicBackend>>()
            .repeat = repeat as _;
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGSPAT(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(HoloWrap::new(GSPAT::new(DynamicBackend::new(
            *Box::from_raw(backend as _),
        )))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGSPATRepeat(holo: ConstPtr, repeat: u32) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<GSPAT<DynamicBackend>>()
            .repeat = repeat as _;
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloNaive(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(HoloWrap::new(Naive::new(DynamicBackend::new(
            *Box::from_raw(backend as _),
        )))) as _
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloGreedy() -> ConstPtr {
    Box::into_raw(HoloWrap::new(Greedy::new())) as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloGreedyPhaseDiv(holo: ConstPtr, div: u32) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<Greedy>()
            .phase_div = div as _;
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainHoloLM(backend: ConstPtr) -> ConstPtr {
    unsafe {
        Box::into_raw(HoloWrap::new(LM::new(DynamicBackend::new(*Box::from_raw(
            backend as _,
        ))))) as _
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMEps1(holo: ConstPtr, eps_1: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<LM<DynamicBackend>>()
            .eps_1 = eps_1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMEps2(holo: ConstPtr, eps_2: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<LM<DynamicBackend>>()
            .eps_2 = eps_2;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMTau(holo: ConstPtr, tau: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<LM<DynamicBackend>>()
            .tau = tau;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMKMax(holo: ConstPtr, k_max: u32) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<LM<DynamicBackend>>()
            .k_max = k_max as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloLMInitial(holo: ConstPtr, ptr: *const float, len: u64) {
    unsafe {
        let mut initial = vec![0.; len as _];
        std::ptr::copy_nonoverlapping(ptr, initial.as_mut_ptr(), len as _);
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .cast::<LM<DynamicBackend>>()
            .initial = initial;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloAdd(holo: ConstPtr, x: float, y: float, z: float, amp: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .holo
            .add_focus(Vector3::new(x, y, z), amp);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSetDotCareConstraint(holo: ConstPtr) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .holo
            .set_constraint(Constraint::DontCare);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSetNormalizeConstraint(holo: ConstPtr) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .holo
            .set_constraint(Constraint::Normalize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSetUniformConstraint(holo: ConstPtr, value: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .holo
            .set_constraint(Constraint::Uniform(value));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSetClampConstraint(holo: ConstPtr, min: float, max: float) {
    unsafe {
        (holo as *mut Box<HG> as *mut Box<HoloWrap>)
            .as_mut()
            .unwrap()
            .holo
            .set_constraint(Constraint::Clamp(min, max));
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDDeleteGainHolo(holo: ConstPtr) {
    unsafe {
        let _ = Box::from_raw(holo as *mut Box<HG>);
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
                AUTDGainHoloAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloSetDotCareConstraint(holo);
                AUTDGainHoloSetNormalizeConstraint(holo);
                AUTDGainHoloSetUniformConstraint(holo, 1.);
                AUTDGainHoloSetClampConstraint(holo, 0., 1.);
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

                AUTDDeleteGainHolo(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloEVP(backend);
                AUTDGainHoloAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloSetDotCareConstraint(holo);
                AUTDGainHoloSetNormalizeConstraint(holo);
                AUTDGainHoloSetUniformConstraint(holo, 1.);
                AUTDGainHoloSetClampConstraint(holo, 0., 1.);
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

                AUTDDeleteGainHolo(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloNaive(backend);
                AUTDGainHoloAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloSetDotCareConstraint(holo);
                AUTDGainHoloSetNormalizeConstraint(holo);
                AUTDGainHoloSetUniformConstraint(holo, 1.);
                AUTDGainHoloSetClampConstraint(holo, 0., 1.);

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

                AUTDDeleteGainHolo(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloGS(backend);
                AUTDGainHoloAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloSetDotCareConstraint(holo);
                AUTDGainHoloSetNormalizeConstraint(holo);
                AUTDGainHoloSetUniformConstraint(holo, 1.);
                AUTDGainHoloSetClampConstraint(holo, 0., 1.);
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

                AUTDDeleteGainHolo(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloGSPAT(backend);
                AUTDGainHoloAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloSetDotCareConstraint(holo);
                AUTDGainHoloSetNormalizeConstraint(holo);
                AUTDGainHoloSetUniformConstraint(holo, 1.);
                AUTDGainHoloSetClampConstraint(holo, 0., 1.);
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

                AUTDDeleteGainHolo(holo);
            }

            {
                let backend = AUTDDefaultBackend();
                let holo = AUTDGainHoloLM(backend);
                AUTDGainHoloAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloSetDotCareConstraint(holo);
                AUTDGainHoloSetNormalizeConstraint(holo);
                AUTDGainHoloSetUniformConstraint(holo, 1.);
                AUTDGainHoloSetClampConstraint(holo, 0., 1.);
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

                AUTDDeleteGainHolo(holo);
            }

            {
                let holo = AUTDGainHoloGreedy();
                AUTDGainHoloAdd(holo, 10., 20., 30., 1.);
                AUTDGainHoloSetDotCareConstraint(holo);
                AUTDGainHoloSetNormalizeConstraint(holo);
                AUTDGainHoloSetUniformConstraint(holo, 1.);
                AUTDGainHoloSetClampConstraint(holo, 0., 1.);
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

                AUTDDeleteGainHolo(holo);
            }
        }
    }
}
