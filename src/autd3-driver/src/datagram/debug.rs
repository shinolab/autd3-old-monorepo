/*
 * File: debug.rs
 * Project: datagram
 * Created Date: 21/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, time::Duration};

use crate::{datagram::*, derive::prelude::Transducer, error::AUTDInternalError, geometry::Device};

/// Datagram for configure debug_output_idx
#[derive(Default)]
pub struct ConfigureDebugOutoutIdx {
    idx: HashMap<usize, usize>,
}

impl ConfigureDebugOutoutIdx {
    /// constructor
    pub fn new() -> Self {
        Self {
            idx: Default::default(),
        }
    }

    pub fn set(self, tr: &Transducer) -> Self {
        let Self { mut idx } = self;
        idx.insert(tr.dev_idx(), tr.tr_idx());
        Self { idx }
    }

    pub const fn idx(&self) -> &HashMap<usize, usize> {
        &self.idx
    }
}

impl std::ops::Index<&Device> for ConfigureDebugOutoutIdx {
    type Output = usize;

    fn index(&self, dev: &Device) -> &Self::Output {
        &self.idx[&dev.idx()]
    }
}

impl Datagram for ConfigureDebugOutoutIdx {
    type O1 = crate::operation::DebugOutIdxOp;
    type O2 = crate::operation::NullOp;

    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(200))
    }

    fn operation(self) -> Result<(Self::O1, Self::O2), AUTDInternalError> {
        Ok((Self::O1::new(self.idx), Self::O2::default()))
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{UnitQuaternion, Vector3};

    use super::*;

    #[test]
    fn test_debug_output_idx() {
        let debug_output_idx = ConfigureDebugOutoutIdx::new();
        assert!(debug_output_idx.idx().is_empty());

        let device = Device::new(
            0,
            vec![Transducer::new(
                0,
                0,
                Vector3::zeros(),
                UnitQuaternion::identity(),
            )],
        );
        let debug_output_idx = debug_output_idx.set(&device[0]);

        assert!(!debug_output_idx.idx().is_empty());
        assert_eq!(debug_output_idx[&device], 0);
    }

    #[test]
    fn test_debug_output_idx_default() {
        let debug_output_idx = ConfigureDebugOutoutIdx::default();
        assert!(debug_output_idx.idx().is_empty());
    }

    #[test]
    fn test_debug_output_idx_timeout() {
        let debug_output_idx = ConfigureDebugOutoutIdx::new();
        let timeout = <ConfigureDebugOutoutIdx as Datagram>::timeout(&debug_output_idx);
        assert!(timeout.is_some());
        assert!(timeout.unwrap() > Duration::ZERO);
    }

    #[test]
    fn test_debug_output_idx_operation() {
        let debug_output_idx = ConfigureDebugOutoutIdx::new();
        let r = <ConfigureDebugOutoutIdx as Datagram>::operation(debug_output_idx);
        assert!(r.is_ok());
        let _: (crate::operation::DebugOutIdxOp, crate::operation::NullOp) = r.unwrap();
    }
}
