/*
 * File: silencer.rs
 * Project: datagram
 * Created Date: 01/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::time::Duration;

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram for configure silencer
pub struct Silencer {
    step: u16,
}

impl Silencer {
    /// constructor
    ///
    /// # Arguments
    /// * `step` - The update step of silencer. The lower the value, the stronger the silencer effect.
    pub const fn new(step: u16) -> Self {
        Self { step }
    }

    /// Disable silencer
    pub const fn disable() -> Self {
        Self { step: 0xFFFF }
    }

    pub const fn step(&self) -> u16 {
        self.step
    }
}

impl Default for Silencer {
    fn default() -> Self {
        Self::new(10)
    }
}

impl<T: Transducer> Datagram<T> for Silencer {
    type O1 = crate::operation::ConfigSilencerOp;
    type O2 = crate::operation::NullOp;

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(self.step), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silencer() {
        let silencer = Silencer::new(20);
        assert_eq!(silencer.step(), 20);
    }

    #[test]
    fn test_silencer_disable() {
        let silencer = Silencer::disable();
        assert_eq!(silencer.step(), 0xFFFF);
    }

    #[test]
    fn test_silencer_default() {
        let silencer = Silencer::default();
        assert_eq!(silencer.step(), 10);
    }

    #[test]
    fn test_silencer_timeout() {
        let silencer = Silencer::new(10);
        let timeout = <Silencer as Datagram<LegacyTransducer>>::timeout(&silencer);
        assert!(timeout.is_some());
        assert!(timeout.unwrap() > Duration::ZERO);
    }

    #[test]
    fn test_silencer_operation() {
        let silencer = Silencer::new(10);
        let r = <Silencer as Datagram<LegacyTransducer>>::operation(silencer);
        assert!(r.is_ok());
        let _: (crate::operation::ConfigSilencerOp, crate::operation::NullOp) = r.unwrap();
    }
}
