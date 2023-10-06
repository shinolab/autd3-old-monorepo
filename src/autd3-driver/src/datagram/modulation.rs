/*
 * File: modulation.rs
 * Project: datagram
 * Created Date: 29/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{defined::float, error::AUTDInternalError, fpga::FPGA_SUB_CLK_FREQ};

pub trait ModulationProperty {
    fn sampling_frequency(&self) -> float {
        FPGA_SUB_CLK_FREQ as float / self.sampling_frequency_division() as float
    }
    fn sampling_frequency_division(&self) -> u32;
    fn sampling_period(&self) -> std::time::Duration {
        std::time::Duration::from_nanos((1000000000. / self.sampling_frequency()) as u64)
    }
}

/// Modulation controls the amplitude modulation data.
///
/// Modulation has following restrictions:
/// * The buffer size is up to 65536.
/// * The sampling rate is [crate::FPGA_SUB_CLK_FREQ]/N, where N is a 32-bit unsigned integer and must be at least [crate::SAMPLING_FREQ_DIV_MIN].
/// * Modulation automatically loops. It is not possible to control only one loop, etc.
/// * The start/end timing of Modulation cannot be controlled.
#[allow(clippy::len_without_is_empty)]
pub trait Modulation: ModulationProperty {
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError>;
    fn len(&self) -> Result<usize, AUTDInternalError> {
        self.calc().map(|v| v.len())
    }
}

impl ModulationProperty for Box<dyn Modulation> {
    #[cfg_attr(coverage_nightly, no_coverage)]
    fn sampling_frequency(&self) -> float {
        self.as_ref().sampling_frequency()
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn sampling_frequency_division(&self) -> u32 {
        self.as_ref().sampling_frequency_division()
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn sampling_period(&self) -> std::time::Duration {
        self.as_ref().sampling_period()
    }
}

impl Modulation for Box<dyn Modulation> {
    #[cfg_attr(coverage_nightly, no_coverage)]
    fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
        self.as_ref().calc()
    }

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn len(&self) -> Result<usize, AUTDInternalError> {
        self.as_ref().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NullModulation {
        pub buf: Vec<float>,
        pub freq_div: u32,
    }

    impl ModulationProperty for NullModulation {
        fn sampling_frequency_division(&self) -> u32 {
            self.freq_div
        }
    }

    impl Modulation for NullModulation {
        fn calc(&self) -> Result<Vec<float>, AUTDInternalError> {
            Ok(self.buf.clone())
        }
    }

    #[test]
    fn test_modulation_property() {
        let m = NullModulation {
            freq_div: 512,
            buf: vec![],
        };
        assert_eq!(m.sampling_frequency_division(), 512);
        assert_approx_eq::assert_approx_eq!(m.sampling_frequency(), 40e3);
        assert_eq!(m.sampling_period(), std::time::Duration::from_micros(25));
    }

    #[test]
    fn test_modulation_len() {
        assert_eq!(
            NullModulation {
                freq_div: 512,
                buf: vec![],
            }
            .len()
            .unwrap(),
            0
        );

        assert_eq!(
            NullModulation {
                freq_div: 512,
                buf: vec![0.0; 100],
            }
            .len()
            .unwrap(),
            100
        );
    }
}
