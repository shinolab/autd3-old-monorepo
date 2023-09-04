/*
 * File: amplitude.rs
 * Project: src
 * Created Date: 07/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, defined::float, error::AUTDInternalError, geometry::*};

/// Amplitudes settings for AdvancedPhase mode
pub struct Amplitudes {
    amp: float,
}

impl Amplitudes {
    /// Constructor. Set amplitude uniformally.
    ///
    /// # Arguments
    ///
    /// * `amp` - Amplitude
    ///
    pub const fn uniform(amp: float) -> Self {
        Self { amp }
    }

    /// Constructor. Set amplitude to 0.
    pub const fn none() -> Self {
        Self::uniform(0.0)
    }

    pub const fn amp(&self) -> float {
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
