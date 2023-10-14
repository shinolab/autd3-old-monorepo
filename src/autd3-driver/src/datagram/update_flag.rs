/*
 * File: update_flag.rs
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

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram to update flags (Force fan flag and reads FPGA info flag)
#[derive(Default)]
pub struct UpdateFlags {}

impl UpdateFlags {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<T: Transducer> Datagram<T> for UpdateFlags {
    type O1 = crate::operation::UpdateFlagsOp;
    type O2 = crate::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::default(), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::operation::{NullOp, UpdateFlagsOp};

    use super::*;

    #[test]
    fn test_mod_delay_timeout() {
        let delay = UpdateFlags::new();
        let timeout = <UpdateFlags as Datagram<LegacyTransducer>>::timeout(&delay);
        assert!(timeout.is_none());
    }

    #[test]
    fn test_mod_delay_operation() {
        let delay = UpdateFlags::default();
        let r = <UpdateFlags as Datagram<LegacyTransducer>>::operation(delay);
        assert!(r.is_ok());
        let _: (UpdateFlagsOp, NullOp) = r.unwrap();
    }
}
