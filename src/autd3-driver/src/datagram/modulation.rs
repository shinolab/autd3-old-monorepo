/*
 * File: modulation.rs
 * Project: datagram
 * Created Date: 29/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    common::{EmitIntensity, SamplingConfiguration},
    error::AUTDInternalError,
};

pub trait ModulationProperty {
    fn sampling_config(&self) -> SamplingConfiguration;
}

/// Modulation controls the amplitude modulation data.
///
/// Modulation has following restrictions:
/// * The buffer size is up to 65536.
/// * The sampling rate is [crate::FPGA_CLK_FREQ]/N, where N is a 32-bit unsigned integer and must be at least [crate::SAMPLING_FREQ_DIV_MIN].
/// * Modulation automatically loops. It is not possible to control only one loop, etc.
/// * The start/end timing of Modulation cannot be controlled.
#[allow(clippy::len_without_is_empty)]
pub trait Modulation: ModulationProperty {
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError>;
    fn len(&self) -> Result<usize, AUTDInternalError> {
        self.calc().map(|v| v.len())
    }
}

impl ModulationProperty for Box<dyn Modulation> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn sampling_config(&self) -> SamplingConfiguration {
        self.as_ref().sampling_config()
    }
}

impl Modulation for Box<dyn Modulation> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        self.as_ref().calc()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn len(&self) -> Result<usize, AUTDInternalError> {
        self.as_ref().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NullModulation {
        pub buf: Vec<EmitIntensity>,
        pub config: SamplingConfiguration,
    }

    impl ModulationProperty for NullModulation {
        fn sampling_config(&self) -> SamplingConfiguration {
            self.config
        }
    }

    impl Modulation for NullModulation {
        fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
            Ok(self.buf.clone())
        }
    }

    #[test]
    fn test_modulation_property() {
        let m = NullModulation {
            config: SamplingConfiguration::new_with_frequency_division(512).unwrap(),
            buf: vec![],
        };
        assert_eq!(m.sampling_config().frequency_division(), 512);
    }

    #[test]
    fn test_modulation_len() {
        assert_eq!(
            NullModulation {
                config: SamplingConfiguration::new_with_frequency_division(512).unwrap(),
                buf: vec![],
            }
            .len()
            .unwrap(),
            0
        );

        assert_eq!(
            NullModulation {
                config: SamplingConfiguration::new_with_frequency_division(512).unwrap(),
                buf: vec![EmitIntensity::new(0); 100],
            }
            .len()
            .unwrap(),
            100
        );
    }
}
