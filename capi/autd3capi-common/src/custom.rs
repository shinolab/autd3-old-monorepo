/*
 * File: custom.rs
 * Project: src
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    core::{
        error::AUTDInternalError,
        float,
        gain::GainFilter,
        geometry::{Geometry, Transducer},
        Drive,
    },
    derive::{Gain, Modulation},
    Gain, Modulation,
};

#[derive(Gain)]
pub struct CustomGain {
    pub drives: Vec<Drive>,
}

impl<T: Transducer> crate::core::gain::Gain<T> for CustomGain {
    fn calc(
        &self,
        _geometry: &Geometry<T>,
        _filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(self.drives.clone())
    }
}

#[derive(Modulation)]
pub struct CustomModulation {
    pub buf: Vec<float>,
    pub freq_div: u32,
}

impl crate::core::modulation::Modulation for CustomModulation {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.buf.clone())
    }
}

#[cfg(test)]
mod tests {
    use autd3_core::geometry::{Geometry, LegacyTransducer};

    use crate::DynamicTransducer;

    use super::*;

    #[test]
    fn test_custom_gain() {
        let drives = vec![
            Drive {
                amp: 0.9,
                phase: 0.8
            };
            10
        ];
        let g = CustomGain { drives };

        let geometry = Geometry::<DynamicTransducer>::new(vec![], vec![], 0., 0.);
        assert!(geometry.is_ok());
        let geometry = geometry.unwrap();
        let d = g.calc(&geometry, GainFilter::All);
        assert!(d.is_ok());
    }

    #[test]
    fn test_custom_modulation() {
        let buf = vec![0.9; 10];
        let m = CustomModulation { buf, freq_div: 1 };

        let d = m.calc();
        assert!(d.is_ok());

        d.unwrap().iter().for_each(|&d| {
            assert_eq!(d, 0.9);
        })
    }
}
