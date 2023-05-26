/*
 * File: dynamic_gain.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use autd3::{
    core::{gain::Gain, GainOp},
    prelude::{Geometry, Transducer},
};

use crate::{dynamic_transducer::TransMode, DynamicDatagram, DynamicTransducer};

pub trait DynamicGain: DynamicDatagram {
    fn gain(&self) -> &dyn Gain<DynamicTransducer>;
    fn gain_mut(&mut self) -> &mut dyn Gain<DynamicTransducer>;
}

pub struct GainWrap {
    pub gain: Box<dyn Gain<DynamicTransducer>>,
}

impl GainWrap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<G: 'static + Gain<DynamicTransducer>>(gain: G) -> Box<Box<dyn DynamicGain>> {
        Box::new(Box::new(Self {
            gain: Box::new(gain),
        }))
    }
}

impl DynamicGain for GainWrap {
    fn gain(&self) -> &dyn Gain<DynamicTransducer> {
        &*self.gain
    }

    fn gain_mut(&mut self) -> &mut dyn Gain<DynamicTransducer> {
        &mut *self.gain
    }
}

impl DynamicDatagram for GainWrap {
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
                    self.gain.calc(geometry)?,
                    || geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
            TransMode::Advanced => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::GainAdvanced::new(
                    self.gain.calc(geometry)?,
                    || geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
            TransMode::AdvancedPhase => Ok((
                Box::<autd3::core::NullHeader>::default(),
                Box::new(autd3::core::GainAdvancedPhase::new(
                    self.gain.calc(geometry)?,
                    || geometry.transducers().map(|tr| tr.cycle()).collect(),
                )),
            )),
        }
    }

    fn timeout(&self) -> Option<Duration> {
        None
    }
}
