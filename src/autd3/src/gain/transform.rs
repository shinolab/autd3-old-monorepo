/*
 * File: transform.rs
 * Project: modulation
 * Created Date: 15/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, marker::PhantomData};

use autd3_driver::{
    derive::prelude::*,
    geometry::{Device, Geometry},
};

/// Gain to transform gain data
pub struct Transform<T: Transducer, G: Gain<T>, F: Fn(&Device<T>, &T, &Drive) -> Drive> {
    gain: G,
    f: F,
    phantom: std::marker::PhantomData<T>,
}

pub trait IntoTransform<T: Transducer, G: Gain<T>> {
    /// transform gain data
    ///
    /// # Arguments
    ///
    /// * `f` - transform function. The first argument is the device, the second argument is transducer, and the third argument is the original gain data.
    ///
    fn with_transform<F: Fn(&Device<T>, &T, &Drive) -> Drive>(self, f: F) -> Transform<T, G, F>;
}

impl<T: Transducer, G: Gain<T>> IntoTransform<T, G> for G {
    fn with_transform<F: Fn(&Device<T>, &T, &Drive) -> Drive>(self, f: F) -> Transform<T, G, F> {
        Transform {
            gain: self,
            f,
            phantom: PhantomData,
        }
    }
}

impl<
        T: Transducer + 'static,
        G: Gain<T> + 'static,
        F: Fn(&Device<T>, &T, &Drive) -> Drive + 'static,
    > autd3_driver::datagram::Datagram<T> for Transform<T, G, F>
where
    autd3_driver::operation::GainOp<T, Self>: autd3_driver::operation::Operation<T>,
{
    type O1 = autd3_driver::operation::GainOp<T, Self>;
    type O2 = autd3_driver::operation::NullOp;

    fn operation(self) -> Result<(Self::O1, Self::O2), autd3_driver::error::AUTDInternalError> {
        Ok((Self::O1::new(self), Self::O2::default()))
    }
}

impl<
        T: Transducer + 'static,
        G: Gain<T> + 'static,
        F: Fn(&Device<T>, &T, &Drive) -> Drive + 'static,
    > autd3_driver::datagram::GainAsAny for Transform<T, G, F>
{
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl<
        T: Transducer + 'static,
        G: Gain<T> + 'static,
        F: Fn(&Device<T>, &T, &Drive) -> Drive + 'static,
    > Gain<T> for Transform<T, G, F>
{
    fn calc(
        &self,
        geometry: &Geometry<T>,
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
