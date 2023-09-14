/*
 * File: filter.rs
 * Project: datagram
 * Created Date: 04/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram to set amplitude filter
#[derive(Default)]
pub struct ConfigureAmpFilter {}

impl ConfigureAmpFilter {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for ConfigureAmpFilter {
    type O1 = crate::operation::ConfigureAmpFilterOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::default(), Self::O2::default()))
    }
}

/// Datagram to set phase filter
#[derive(Default)]
pub struct ConfigurePhaseFilter {}

impl ConfigurePhaseFilter {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for ConfigurePhaseFilter {
    type O1 = crate::operation::ConfigurePhaseFilterOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::default(), Self::O2::default()))
    }
}
