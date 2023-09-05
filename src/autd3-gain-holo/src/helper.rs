/*
 * File: helper.rs
 * Project: src
 * Created Date: 03/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3_driver::{
    datagram::GainFilter,
    defined::{Drive, PI},
    error::AUTDInternalError,
    geometry::{Device, Transducer},
};
use nalgebra::ComplexField;

use crate::{Constraint, VectorXc};

#[doc(hidden)]
#[macro_export]
macro_rules! impl_holo {
    ($backend:tt, $t:ty) => {
        impl<$backend> $t
        where
            $backend: $crate::LinAlgBackend,
        {
            /// Add focus
            pub fn add_focus(self, focus: Vector3, amp: float) -> Self {
                let mut foci = self.foci;
                let mut amps = self.amps;
                foci.push(focus);
                amps.push(amp);
                Self { foci, amps, ..self }
            }

            /// Set constraint
            pub fn with_constraint(self, constraint: Constraint) -> Self {
                Self { constraint, ..self }
            }

            /// Add foci
            pub fn add_foci_from_iter<I: IntoIterator<Item = (Vector3, float)>>(
                self,
                iter: I,
            ) -> Self {
                let mut foci = self.foci;
                let mut amps = self.amps;
                for (focus, amp) in iter {
                    foci.push(focus);
                    amps.push(amp);
                }
                Self { foci, amps, ..self }
            }

            pub fn foci(
                &self,
            ) -> std::iter::Zip<std::slice::Iter<'_, Vector3>, std::slice::Iter<'_, float>> {
                self.foci.iter().zip(self.amps.iter())
            }

            pub const fn constraint(&self) -> &Constraint {
                &self.constraint
            }
        }
    };

    ($t:ty) => {
        impl $t {
            /// Add focus
            pub fn add_focus(self, focus: Vector3, amp: float) -> Self {
                let mut foci = self.foci;
                let mut amps = self.amps;
                foci.push(focus);
                amps.push(amp);
                Self { foci, amps, ..self }
            }

            /// Set constraint
            pub fn with_constraint(self, constraint: Constraint) -> Self {
                Self { constraint, ..self }
            }

            /// Add foci
            pub fn add_foci_from_iter<I: IntoIterator<Item = (Vector3, float)>>(
                self,
                iter: I,
            ) -> Self {
                let mut foci = self.foci;
                let mut amps = self.amps;
                for (focus, amp) in iter {
                    foci.push(focus);
                    amps.push(amp);
                }
                Self { foci, amps, ..self }
            }

            pub fn foci(
                &self,
            ) -> std::iter::Zip<std::slice::Iter<'_, Vector3>, std::slice::Iter<'_, float>> {
                self.foci.iter().zip(self.amps.iter())
            }

            pub const fn constraint(&self) -> &Constraint {
                &self.constraint
            }
        }
    };
}

#[allow(clippy::uninit_vec)]
pub fn generate_result<T: Transducer>(
    devices: &[&Device<T>],
    q: VectorXc,
    constraint: &Constraint,
    filter: GainFilter,
) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
    let max_coefficient = q.camax().abs();
    let mut idx = 0;
    match filter {
        GainFilter::All => Ok(devices
            .iter()
            .map(|dev| {
                (
                    dev.idx(),
                    dev.iter()
                        .map(|_| {
                            let phase = q[idx].argument() + PI;
                            let amp = constraint.convert(q[idx].abs(), max_coefficient);
                            idx += 1;
                            Drive { amp, phase }
                        })
                        .collect(),
                )
            })
            .collect()),
        GainFilter::Filter(filter) => Ok(devices
            .iter()
            .map(|dev| {
                if let Some(filter) = filter.get(&dev.idx()) {
                    (
                        dev.idx(),
                        dev.iter()
                            .filter(|tr| filter[tr.local_idx()])
                            .map(|_| {
                                let phase = q[idx].argument() + PI;
                                let amp = constraint.convert(q[idx].abs(), max_coefficient);
                                idx += 1;
                                Drive { amp, phase }
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
            .collect()),
    }
}
