/*
 * File: gain.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

use bitvec::prelude::*;

use autd3_driver::Drive;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub enum GainFilter<'a> {
    All,
    Filter(&'a BitVec<usize, Lsb0>),
}

pub trait GainAsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Gain controls amplitude and phase of each transducer.
pub trait Gain<T: Transducer>: GainAsAny {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError>;
    fn transform<F: Fn(&T) -> Drive + Sync + Send>(
        geometry: &Geometry<T>,
        filter: GainFilter,
        f: F,
    ) -> Vec<Drive>
    where
        Self: Sized,
    {
        #[cfg(feature = "parallel")]
        {
            match filter {
                GainFilter::All => geometry.transducers().par_iter().map(f).collect(),
                GainFilter::Filter(filter) => geometry
                    .transducers()
                    .par_iter()
                    .map(|tr| {
                        if filter[tr.idx()] {
                            f(tr)
                        } else {
                            Drive { phase: 0., amp: 0. }
                        }
                    })
                    .collect(),
            }
        }
        #[cfg(not(feature = "parallel"))]
        {
            match filter {
                GainFilter::All => geometry.transducers().map(f).collect(),
                GainFilter::Filter(filter) => geometry
                    .transducers()
                    .map(|tr| {
                        if filter[tr.idx()] {
                            f(tr)
                        } else {
                            Drive { phase: 0., amp: 0. }
                        }
                    })
                    .collect(),
            }
        }
    }
}

impl<'a, T: Transducer> GainAsAny for Box<dyn Gain<T> + 'a> {
    fn as_any(&self) -> &dyn std::any::Any {
        self.as_ref().as_any()
    }
}

impl<'a, T: Transducer> Gain<T> for Box<dyn Gain<T> + 'a> {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<Vec<Drive>, AUTDInternalError> {
        self.as_ref().calc(geometry, filter)
    }
}
