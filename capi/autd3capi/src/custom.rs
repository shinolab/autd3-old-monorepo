/*
 * File: custom.rs
 * Project: src
 * Created Date: 19/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, modulation::ModulationProperty, Drive};
use autd3_traits::*;
use autd3capi_common::*;

#[derive(Gain)]
pub struct CustomGain {
    amp: Vec<float>,
    phase: Vec<float>,
}

impl CustomGain {
    pub fn new(amp_: *const float, phase_: *const float, size: u64) -> Self {
        let mut amp = vec![0.0; size as _];
        let mut phase = vec![0.0; size as _];

        unsafe {
            std::ptr::copy_nonoverlapping(amp_, amp.as_mut_ptr(), size as _);
            std::ptr::copy_nonoverlapping(phase_, phase.as_mut_ptr(), size as _);
        }

        Self { amp, phase }
    }
}

impl<T: Transducer> Gain<T> for CustomGain {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(Self::transform(geometry, |tr| {
            let amp = self.amp[tr.idx()];
            let phase = self.phase[tr.idx()];
            Drive { phase, amp }
        }))
    }
}

impl_sendable_for_gain!(CustomGain);

#[derive(Modulation)]
pub struct CustomModulation {
    buf: Vec<float>,
    freq_div: u32,
}

impl CustomModulation {
    pub fn new(data: *const float, size: u64, freq_div: u32) -> Self {
        let mut buf = vec![0.0; size as _];
        unsafe {
            std::ptr::copy_nonoverlapping(data, buf.as_mut_ptr(), size as _);
        }
        Self { buf, freq_div }
    }
}

impl Modulation for CustomModulation {
    fn calc(&mut self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}

impl_sendable_for_modulation!(CustomModulation);
