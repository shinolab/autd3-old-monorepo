/*
 * File: amplitude.rs
 * Project: datagram
 * Created Date: 29/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{common::Amplitude, datagram::*, error::AUTDInternalError, geometry::*};

/// Amplitudes settings for AdvancedPhase mode
pub struct Amplitudes {
    amp: Amplitude,
}

impl Amplitudes {
    /// Constructor. Set amplitude uniformally.
    ///
    /// # Arguments
    ///
    /// * `amp` - Amplitude
    ///
    pub fn uniform<A: Into<Amplitude>>(amp: A) -> Self {
        Self { amp: amp.into() }
    }

    /// Constructor. Set amplitude to 0.
    pub const fn disable() -> Self {
        Self {
            amp: Amplitude::MIN,
        }
    }

    pub const fn amp(&self) -> Amplitude {
        self.amp
    }
}

impl Datagram<AdvancedPhaseTransducer> for Amplitudes {
    type O1 = crate::operation::AmplitudeOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(self.amp), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{AmplitudeOp, NullOp};

    use super::*;

    #[test]
    fn test_amplitudes() {
        let amp = Amplitudes::uniform(1.0);
        assert_eq!(amp.amp().value(), 1.0);
    }

    #[test]
    fn test_amplitudes_disable() {
        let amp = Amplitudes::disable();
        assert_eq!(amp.amp().value(), 0.0);
    }

    #[test]
    fn test_amplitudes_timeout() {
        let amp = Amplitudes::uniform(1.0);
        let timeout = <Amplitudes as Datagram<AdvancedPhaseTransducer>>::timeout(&amp);
        assert!(timeout.is_none());
    }

    #[test]
    fn test_amplitudes_operation() {
        let amp = Amplitudes::uniform(1.0);
        let r = amp.operation();
        assert!(r.is_ok());
        let _: (AmplitudeOp, NullOp) = r.unwrap();
    }
}
