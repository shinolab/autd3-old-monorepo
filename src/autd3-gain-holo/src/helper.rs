/*
 * File: helper.rs
 * Project: src
 * Created Date: 03/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    gain::GainFilter,
    geometry::{Geometry, Transducer},
    Drive, PI,
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
    geometry: &Geometry<T>,
    q: VectorXc,
    constraint: &Constraint,
    filter: GainFilter,
) -> Result<Vec<Drive>, AUTDInternalError> {
    let max_coefficient = q.camax().abs();
    match filter {
        GainFilter::All => Ok(geometry
            .transducers()
            .map(|tr| {
                let phase = q[tr.idx()].argument() + PI;
                let amp = constraint.convert(q[tr.idx()].abs(), max_coefficient);
                Drive { amp, phase }
            })
            .collect()),
        GainFilter::Filter(filter) => {
            let mut result = Vec::with_capacity(geometry.num_transducers());
            unsafe {
                result.set_len(geometry.num_transducers());
            }
            let mut idx = 0;
            geometry.transducers().for_each(|tr| {
                if !filter[idx] {
                    return;
                }
                let phase = q[idx].argument() + PI;
                let amp = constraint.convert(q[idx].abs(), max_coefficient);
                result[tr.idx()] = Drive { amp, phase };
                idx += 1;
            });
            Ok(result)
        }
    }
}
