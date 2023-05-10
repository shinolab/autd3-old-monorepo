/*
 * File: gain.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    error::AUTDInternalError,
    geometry::{Geometry, Transducer},
};

use autd3_driver::Drive;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Gain contains amplitude and phase of each transducer in the AUTD.
/// Note that the amplitude means duty ratio of Pulse Width Modulation, respectively.
pub trait Gain<T: Transducer> {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError>;
    fn transform<F: Fn(&T) -> Drive + Sync + Send>(geometry: &Geometry<T>, f: F) -> Vec<Drive>
    where
        Self: Sized,
    {
        #[cfg(feature = "parallel")]
        {
            geometry.transducers.par_iter().map(|t| f(t)).collect()
        }
        #[cfg(not(feature = "parallel"))]
        {
            geometry.transducers().map(|t| f(t)).collect()
        }
    }
}
