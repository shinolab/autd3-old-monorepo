/*
 * File: silencer.rs
 * Project: datagram
 * Created Date: 01/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{
    datagram::*,
    error::AUTDInternalError,
    fpga::{SILENCER_STEP_DEFAULT, SILENCER_STEP_MAX, SILENCER_STEP_MIN},
};

/// Datagram for configure silencer
#[derive(Debug, Clone, Copy)]
pub struct Silencer {
    step_intensity: u16,
    step_phase: u16,
}

impl Silencer {
    /// constructor
    ///
    /// # Arguments
    /// * `step` - The update step of silencer. The lower the value, the stronger the silencer effect.
    pub fn new(step_intensity: u16, step_phase: u16) -> Result<Self, AUTDInternalError> {
        if !(SILENCER_STEP_MIN..=SILENCER_STEP_MAX).contains(&step_intensity) {
            return Err(AUTDInternalError::SilencerStepOutOfRange(step_intensity));
        }
        if !(SILENCER_STEP_MIN..=SILENCER_STEP_MAX).contains(&step_phase) {
            return Err(AUTDInternalError::SilencerStepOutOfRange(step_phase));
        }
        Ok(Self {
            step_intensity,
            step_phase,
        })
    }

    /// Disable silencer
    pub const fn disable() -> Self {
        Self {
            step_intensity: SILENCER_STEP_MAX,
            step_phase: SILENCER_STEP_MAX,
        }
    }

    pub const fn step_intensity(&self) -> u16 {
        self.step_intensity
    }

    pub const fn step_phase(&self) -> u16 {
        self.step_phase
    }
}

impl Default for Silencer {
    fn default() -> Self {
        Self {
            step_intensity: SILENCER_STEP_DEFAULT,
            step_phase: SILENCER_STEP_DEFAULT,
        }
    }
}

impl Datagram for Silencer {
    type O1 = crate::operation::ConfigSilencerOp;
    type O2 = crate::operation::NullOp;

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((
            Self::O1::new(self.step_intensity, self.step_phase),
            Self::O2::default(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silencer() {
        let silencer = Silencer::new(10, 20).unwrap();
        assert_eq!(silencer.step_intensity(), 10);
        assert_eq!(silencer.step_phase(), 20);
        assert_eq!(silencer.step_intensity(), silencer.clone().step_intensity());
        assert_eq!(silencer.step_phase(), silencer.clone().step_phase());

        let silencer = Silencer::new(0, 1);
        assert_eq!(
            silencer.unwrap_err(),
            AUTDInternalError::SilencerStepOutOfRange(0)
        );

        let silencer = Silencer::new(1, 0);
        assert_eq!(
            silencer.unwrap_err(),
            AUTDInternalError::SilencerStepOutOfRange(0)
        );
    }

    #[test]
    fn test_silencer_debug() {
        let silencer = Silencer::new(10, 20).unwrap();
        assert_eq!(
            format!("{:?}", silencer),
            "Silencer { step_intensity: 10, step_phase: 20 }"
        );
    }

    #[test]
    fn test_silencer_disable() {
        let silencer = Silencer::disable();
        assert_eq!(silencer.step_intensity(), SILENCER_STEP_MAX);
        assert_eq!(silencer.step_phase(), SILENCER_STEP_MAX);
    }

    #[test]
    fn test_silencer_default() {
        let silencer = Silencer::default();
        assert_eq!(silencer.step_intensity(), SILENCER_STEP_DEFAULT);
        assert_eq!(silencer.step_phase(), SILENCER_STEP_DEFAULT);
    }

    #[test]
    fn test_silencer_timeout() {
        let silencer = Silencer::new(1, 2).unwrap();
        let timeout = <Silencer as Datagram>::timeout(&silencer);
        assert!(timeout.is_some());
        assert!(timeout.unwrap() > Duration::ZERO);
    }

    #[test]
    fn test_silencer_operation() {
        let silencer = Silencer::new(1, 2).unwrap();
        let r = <Silencer as Datagram>::operation(silencer);
        assert!(r.is_ok());
        let _: (crate::operation::ConfigSilencerOp, crate::operation::NullOp) = r.unwrap();
    }
}
