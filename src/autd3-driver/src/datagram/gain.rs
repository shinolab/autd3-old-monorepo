/*
 * File: gain.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{
    defined::Drive,
    error::AUTDInternalError,
    geometry::{Device, Transducer},
};

use bitvec::prelude::*;

pub enum GainFilter<'a> {
    All,
    Filter(&'a HashMap<usize, BitVec<usize, Lsb0>>),
}

pub trait GainAsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Gain controls amplitude and phase of each transducer.
pub trait Gain<T: Transducer>: GainAsAny {
    fn calc(
        &self,
        devices: &[&Device<T>],
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError>;
    fn transform<F: Fn(&Device<T>, &T) -> Drive + Sync + Send>(
        devices: &[&Device<T>],
        filter: GainFilter,
        f: F,
    ) -> HashMap<usize, Vec<Drive>>
    where
        Self: Sized,
    {
        match filter {
            GainFilter::All => devices
                .iter()
                .map(|dev| (dev.idx(), dev.iter().map(|tr| f(dev, tr)).collect()))
                .collect(),
            GainFilter::Filter(filter) => devices
                .iter()
                .map(|dev| {
                    if let Some(filter) = filter.get(&dev.idx()) {
                        (
                            dev.idx(),
                            dev.iter()
                                .map(|tr| {
                                    if filter[tr.local_idx()] {
                                        f(dev, tr)
                                    } else {
                                        Drive { phase: 0., amp: 0. }
                                    }
                                })
                                .collect(),
                        )
                    } else {
                        (
                            dev.idx(),
                            dev.iter().map(|_| Drive { phase: 0., amp: 0. }).collect(),
                        )
                    }
                })
                .collect(),
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
        devices: &[&Device<T>],
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        self.as_ref().calc(devices, filter)
    }
}
