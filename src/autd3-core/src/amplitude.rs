/*
 * File: amplitude.rs
 * Project: src
 * Created Date: 07/11/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{float, Drive};

use crate::{datagram::*, error::AUTDInternalError, geometry::*};

pub struct Amplitudes {
    amp: float,
}

impl Amplitudes {
    pub fn uniform(amp: float) -> Self {
        Self { amp }
    }

    pub fn none() -> Self {
        Self::uniform(0.0)
    }

    pub fn amp(&self) -> float {
        self.amp
    }
}

impl Datagram<AdvancedPhaseTransducer> for Amplitudes {
    type H = autd3_driver::NullHeader;
    type B = autd3_driver::GainAdvancedDuty;

    fn operation(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<(Self::H, Self::B), AUTDInternalError> {
        Ok((
            Self::H::default(),
            <Self::B as autd3_driver::operation::GainOp>::new(
                vec![
                    Drive {
                        phase: 0.0,
                        amp: self.amp,
                    };
                    geometry.num_transducers()
                ],
                || geometry.transducers().map(|tr| tr.cycle()).collect(),
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{autd3_device::AUTD3, geometry::tests::GeometryBuilder};

    use super::*;

    #[test]
    fn test_amplitudes() {
        let geometry = GeometryBuilder::<AdvancedPhaseTransducer>::new()
            .add_device(AUTD3::new(Vector3::new(0., 0., 0.), Vector3::zeros()))
            .build()
            .unwrap();

        let amp = 0.5;

        let mut datagram = Amplitudes::uniform(amp);
        assert_eq!(datagram.timeout(), None);
        datagram.operation(&geometry).unwrap();

        let mut datagram = Amplitudes::uniform(amp).with_timeout(Duration::from_millis(100));
        assert_eq!(datagram.timeout(), Some(Duration::from_millis(100)));
        datagram.operation(&geometry).unwrap();
    }
}
