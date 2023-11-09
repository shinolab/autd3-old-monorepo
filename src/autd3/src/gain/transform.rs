/*
 * File: transform.rs
 * Project: modulation
 * Created Date: 15/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    derive::prelude::*,
    geometry::{Device, Geometry},
};

/// Gain to transform gain data
pub struct Transform<G: Gain, F: Fn(&Device, &Transducer, &Drive) -> Drive> {
    gain: G,
    f: F,
}

pub trait IntoTransform<G: Gain> {
    /// transform gain data
    ///
    /// # Arguments
    ///
    /// * `f` - transform function. The first argument is the device, the second is transducer, and the third is the original drive data.
    ///
    fn with_transform<F: Fn(&Device, &Transducer, &Drive) -> Drive>(self, f: F) -> Transform<G, F>;
}

impl<G: Gain> IntoTransform<G> for G {
    fn with_transform<F: Fn(&Device, &Transducer, &Drive) -> Drive>(self, f: F) -> Transform<G, F> {
        Transform { gain: self, f }
    }
}

impl<G: Gain + 'static, F: Fn(&Device, &Transducer, &Drive) -> Drive + 'static>
    autd3_driver::datagram::Datagram for Transform<G, F>
where
    autd3_driver::operation::GainOp<Self>: autd3_driver::operation::Operation,
{
    type O1 = autd3_driver::operation::GainOp<Self>;
    type O2 = autd3_driver::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
        Ok((Self::O1::new(self), Self::O2::default()))
    }
}

impl<G: Gain + 'static, F: Fn(&Device, &Transducer, &Drive) -> Drive + 'static>
    autd3_driver::datagram::GainAsAny for Transform<G, F>
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<G: Gain + 'static, F: Fn(&Device, &Transducer, &Drive) -> Drive + 'static> Gain
    for Transform<G, F>
{
    fn calc(
        &self,
        geometry: &Geometry,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        let mut g = self.gain.calc(geometry, filter)?;
        g.iter_mut().for_each(|(&dev_idx, d)| {
            d.iter_mut().enumerate().for_each(|(i, d)| {
                *d = (self.f)(&geometry[dev_idx], &geometry[dev_idx][i], d);
            });
        });
        Ok(g)
    }
}
