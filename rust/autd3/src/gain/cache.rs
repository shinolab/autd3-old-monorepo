/*
 * File: Cache.rs
 * Project: gain
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    gain::Gain,
    geometry::{
        AdvancedPhaseTransducer, AdvancedTransducer, Geometry, LegacyTransducer, Transducer,
    },
    sendable::Sendable,
    Drive,
};

use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub struct Cache<T: Transducer, G: Gain<T> + Clone> {
    pub gain: G,
    cache: Vec<Drive>,
    phantom: PhantomData<T>,
}

impl<T: Transducer, G: Gain<T> + Clone> Clone for Cache<T, G> {
    fn clone(&self) -> Self {
        Self {
            gain: self.gain.clone(),
            cache: self.cache.clone(),
            phantom: PhantomData,
        }
    }
}

impl<T: Transducer, G: Gain<T> + Clone> Cache<T, G> {
    /// constructor
    pub fn new(gain: G) -> Self {
        Self {
            gain,
            cache: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn drives(&self) -> &[Drive] {
        &self.cache
    }

    pub fn drives_mut(&mut self) -> &mut [Drive] {
        &mut self.cache
    }
}

impl<T: Transducer, G: Gain<T> + Clone> Gain<T> for Cache<T, G> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        if self.cache.is_empty() {
            self.cache = self.gain.calc(geometry)?;
        }
        Ok(self.cache.clone())
    }
}

impl<T: Transducer, G: Gain<T> + Clone> Cache<T, G> {
    pub fn recalc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        self.cache.clear();
        self.calc(geometry)
    }
}

impl<T: Transducer, G: Gain<T> + Clone> Index<usize> for Cache<T, G> {
    type Output = Drive;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.cache[idx]
    }
}

impl<T: Transducer, G: Gain<T> + Clone> IndexMut<usize> for Cache<T, G> {
    fn index_mut(&mut self, idx: usize) -> &mut Drive {
        &mut self.cache[idx]
    }
}

impl<'a, T: Transducer, G: Gain<T> + Clone> IntoIterator for &'a Cache<T, G> {
    type Item = &'a Drive;
    type IntoIter = std::slice::Iter<'a, Drive>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.iter()
    }
}

impl<'a, T: Transducer, G: Gain<T> + Clone> IntoIterator for &'a mut Cache<T, G> {
    type Item = &'a mut Drive;
    type IntoIter = std::slice::IterMut<'a, Drive>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.iter_mut()
    }
}

impl<G: Gain<LegacyTransducer> + Clone> Sendable<LegacyTransducer> for Cache<LegacyTransducer, G> {
    type H = autd3_core::NullHeader;
    type B = autd3_core::GainLegacy;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(
        &mut self,
        geometry: &Geometry<LegacyTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(self.calc(geometry)?))
    }
}

impl<G: Gain<AdvancedTransducer> + Clone> Sendable<AdvancedTransducer>
    for Cache<AdvancedTransducer, G>
{
    type H = autd3_core::NullHeader;
    type B = autd3_core::GainAdvanced;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(
        &mut self,
        geometry: &Geometry<AdvancedTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(
            self.calc(geometry)?,
            geometry.transducers().map(|tr| tr.cycle()).collect(),
        ))
    }
}

impl<G: Gain<AdvancedPhaseTransducer> + Clone> Sendable<AdvancedPhaseTransducer>
    for Cache<AdvancedPhaseTransducer, G>
{
    type H = autd3_core::NullHeader;
    type B = autd3_core::GainAdvancedPhase;

    fn header_operation(&mut self) -> Result<Self::H, AUTDInternalError> {
        Ok(Self::H::default())
    }

    fn body_operation(
        &mut self,
        geometry: &Geometry<AdvancedPhaseTransducer>,
    ) -> Result<Self::B, AUTDInternalError> {
        Ok(Self::B::new(
            self.calc(geometry)?,
            geometry.transducers().map(|tr| tr.cycle()).collect(),
        ))
    }
}
