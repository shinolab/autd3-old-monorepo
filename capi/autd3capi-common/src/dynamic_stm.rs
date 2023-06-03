/*
 * File: dynamic_modulation.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::{FocusSTM, GainSTM};
use autd3_core::stm::ControlPoint;

use crate::{float, DynamicDatagram, DynamicTransducer, GainWrap};

// impl FocusSTMProps {
//     pub fn new(freq: float) -> Self {
//         Self {
//             control_points: vec![],
//             freq_div: None,
//             freq,
//             start_idx: None,
//             finish_idx: None,
//         }
//     }

//     pub fn with_sampling_freq_div(freq_div: u32) -> Self {
//         Self {
//             control_points: vec![],
//             freq_div: Some(freq_div),
//             freq: 0.,
//             start_idx: None,
//             finish_idx: None,
//         }
//     }

//     pub fn with_sampling_freq(freq: float) -> Self {
//         Self {
//             control_points: vec![],
//             freq_div: Some((FPGA_SUB_CLK_FREQ as float / freq) as u32),
//             freq: 0.,
//             start_idx: None,
//             finish_idx: None,
//         }
//     }
// }

// impl DynamicDatagram for FocusSTM {
//     fn operation(
//         &mut self,
//         mode: crate::dynamic_transducer::TransMode,
//         geometry: &autd3::prelude::Geometry<crate::DynamicTransducer>,
//     ) -> Result<
//         (
//             Box<dyn autd3::core::Operation>,
//             Box<dyn autd3::core::Operation>,
//         ),
//         autd3::core::error::AUTDInternalError,
//     > {
//         DynamicDatagram::operation(&mut self.stm, mode, geometry)
//     }

//     fn timeout(&self) -> Option<std::time::Duration> {
//         DynamicDatagram::timeout(&self.stm)
//     }
// }

pub trait DynamicGainSTM: DynamicDatagram {
    fn stm(&self) -> &GainSTM<'static, DynamicTransducer>;
    fn stm_mut(&mut self) -> &mut GainSTM<'static, DynamicTransducer>;
    fn add(&mut self, gain: Box<GainWrap>);
}

pub struct GainSTMWrap {
    stm: GainSTM<'static, DynamicTransducer>,
}

impl GainSTMWrap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(freq: float) -> Box<Box<dyn DynamicGainSTM>> {
        Box::new(Box::new(Self {
            stm: GainSTM::new(freq),
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

impl DynamicDatagram for GainSTMWrap {
    fn operation(
        &mut self,
        mode: crate::dynamic_transducer::TransMode,
        geometry: &autd3::prelude::Geometry<crate::DynamicTransducer>,
    ) -> Result<
        (
            Box<dyn autd3::core::Operation>,
            Box<dyn autd3::core::Operation>,
        ),
        autd3::core::error::AUTDInternalError,
    > {
        DynamicDatagram::operation(&mut self.stm, mode, geometry)
    }

    fn timeout(&self) -> Option<std::time::Duration> {
        DynamicDatagram::timeout(&self.stm)
    }
}
