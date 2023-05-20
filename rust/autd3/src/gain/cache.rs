/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, gain::Gain, geometry::*, Drive};
use autd3_traits::Gain;

use std::ops::{Index, IndexMut};

#[derive(Gain)]
pub struct Cache {
    cache: Vec<Drive>,
}

impl Clone for Cache {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
        }
    }
}

impl Cache {
    /// constructor
    pub fn new<T: Transducer, G: Gain<T>>(
        mut gain: G,
        geometry: &Geometry<T>,
    ) -> Result<Self, AUTDInternalError> {
        Ok(Self {
            cache: gain.calc(geometry)?,
        })
    }

    pub fn drives(&self) -> &[Drive] {
        &self.cache
    }

    pub fn drives_mut(&mut self) -> &mut [Drive] {
        &mut self.cache
    }
}

impl<T: Transducer> Gain<T> for Cache {
    fn calc(&mut self, _geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        Ok(self.cache.clone())
    }
}

impl Index<usize> for Cache {
    type Output = Drive;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.cache[idx]
    }
}

impl IndexMut<usize> for Cache {
    fn index_mut(&mut self, idx: usize) -> &mut Drive {
        &mut self.cache[idx]
    }
}

impl<'a> IntoIterator for &'a Cache {
    type Item = &'a Drive;
    type IntoIter = std::slice::Iter<'a, Drive>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.iter()
    }
}

impl<'a> IntoIterator for &'a mut Cache {
    type Item = &'a mut Drive;
    type IntoIter = std::slice::IterMut<'a, Drive>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.iter_mut()
    }
}
