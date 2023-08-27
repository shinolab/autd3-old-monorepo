/*
 * File: silencer_config.rs
 * Project: src
 * Created Date: 28/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

/// Datagram to configure silencer
pub struct SilencerConfig {
    step: u16,
}

impl SilencerConfig {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `step` - Update step of silencer. The smaller `step` is, the quieter the output is.  
    ///
    pub const fn new(step: u16) -> Self {
        SilencerConfig { step }
    }

    /// Disable silencer
    pub const fn none() -> Self {
        Self::new(0xFFFF)
    }

    pub const fn step(&self) -> u16 {
        self.step
    }
}

impl<T: Transducer> Datagram<T> for SilencerConfig {
    type H = autd3_driver::ConfigSilencer;
    type B = autd3_driver::NullBody;

    fn operation(&self, _: &Geometry<T>) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((Self::H::new(self.step), Self::B::default()))
    }
}

impl Default for SilencerConfig {
    fn default() -> Self {
        Self::new(10)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{autd3_device::AUTD3, geometry::tests::GeometryBuilder};

    use super::*;

    #[test]
    fn test_silencer() {
        let geometry = GeometryBuilder::<LegacyTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .build()
            .unwrap();

        let datagram = SilencerConfig::default();
        assert_eq!(
            <SilencerConfig as Datagram<LegacyTransducer>>::timeout(&datagram),
            None
        );
        datagram.operation(&geometry).unwrap();

        let datagram = SilencerConfig::default().with_timeout(Duration::from_millis(100));
        assert_eq!(datagram.timeout(), Some(Duration::from_millis(100)));
        datagram.operation(&geometry).unwrap();
    }
}
