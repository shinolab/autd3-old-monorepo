/*
 * File: dynamic_modulation.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::{FocusSTM, GainSTM};

use crate::{DynamicSendable, DynamicTransducer, GainWrap};

pub trait DynamicFocusSTM: DynamicSendable {
    fn stm(&self) -> &FocusSTM;
    fn stm_mut(&mut self) -> &mut FocusSTM;
}

pub struct FocusSTMWrap {
    stm: FocusSTM,
}

impl FocusSTMWrap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<Box<dyn DynamicFocusSTM>> {
        Box::new(Box::new(Self {
            stm: FocusSTM::new(),
        }))
    }
}

impl DynamicFocusSTM for FocusSTMWrap {
    fn stm(&self) -> &FocusSTM {
        &self.stm
    }
    fn stm_mut(&mut self) -> &mut FocusSTM {
        &mut self.stm
    }
}

impl DynamicSendable for FocusSTMWrap {
    fn operation(
        &mut self,
        mode: crate::TransMode,
        geometry: &autd3::prelude::Geometry<crate::DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        autd3::core::error::AUTDInternalError,
    > {
        DynamicSendable::operation(&mut self.stm, mode, geometry)
    }

    fn timeout(&self) -> Option<std::time::Duration> {
        DynamicSendable::timeout(&self.stm)
    }
}

pub trait DynamicGainSTM: DynamicSendable {
    fn stm(&self) -> &GainSTM<'static, DynamicTransducer>;
    fn stm_mut(&mut self) -> &mut GainSTM<'static, DynamicTransducer>;
    fn add(&mut self, gain: Box<GainWrap>);
}

pub struct GainSTMWrap {
    stm: GainSTM<'static, DynamicTransducer>,
}

impl GainSTMWrap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<Box<dyn DynamicGainSTM>> {
        Box::new(Box::new(Self {
            stm: GainSTM::new(),
        }))
    }
}

impl DynamicGainSTM for GainSTMWrap {
    fn stm(&self) -> &GainSTM<'static, DynamicTransducer> {
        &self.stm
    }

    fn stm_mut(&mut self) -> &mut GainSTM<'static, DynamicTransducer> {
        &mut self.stm
    }

    fn add(&mut self, gain: Box<GainWrap>) {
        let gain = gain.gain;
        self.stm.add_boxed(gain)
    }
}

impl DynamicSendable for GainSTMWrap {
    fn operation(
        &mut self,
        mode: crate::TransMode,
        geometry: &autd3::prelude::Geometry<crate::DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        autd3::core::error::AUTDInternalError,
    > {
        DynamicSendable::operation(&mut self.stm, mode, geometry)
    }

    fn timeout(&self) -> Option<std::time::Duration> {
        DynamicSendable::timeout(&self.stm)
    }
}
