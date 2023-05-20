#![allow(clippy::missing_safety_doc)]

mod backend;

use std::time::Duration;

use autd3::core::GainOp;
use autd3_gain_holo::{Backend, Holo, NalgebraBackend, SDP};
use autd3capi_common::*;
use backend::DynamicBackend;

struct DynamicHolo<H: Holo + Gain<DynamicTransducer>> {
    holo: H,
}

impl<H: Holo + Gain<DynamicTransducer>> DynamicGain for DynamicHolo<H> {
    fn gain(&self) -> &dyn Gain<DynamicTransducer> {
        &self.holo
    }

    fn gain_mut(&mut self) -> &mut dyn Gain<DynamicTransducer> {
        &mut self.holo
    }
}

impl<H: Holo + Gain<DynamicTransducer>> Gain<DynamicTransducer> for DynamicHolo<H> {
    fn calc(
        &mut self,
        geometry: &Geometry<DynamicTransducer>,
    ) -> Result<Vec<autd3::core::Drive>, autd3::core::error::AUTDInternalError> {
        self.holo.calc(geometry)
    }
}

impl<H: Holo + Gain<DynamicTransducer>> DynamicSendable for DynamicHolo<H> {
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
pub unsafe extern "C" fn AUTDDefaultBackend(out: *mut ConstPtr) {
    unsafe {
        let backend: Box<Box<dyn Backend>> = Box::new(Box::new(NalgebraBackend::new()));
        *out = Box::into_raw(backend) as _;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainHoloSDP(out: *mut ConstPtr, backend: ConstPtr) {
    unsafe {
        let gain: Box<Box<G>> = Box::new(Box::new(DynamicHolo {
            holo: SDP::new(DynamicBackend::new(*Box::from_raw(backend as _))),
        }));
        *out = Box::into_raw(gain) as _;
    }
}
