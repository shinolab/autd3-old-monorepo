/*
 * File: greedy.rs
 * Project: combinational
 * Created Date: 03/06/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::{constraint::Constraint, impl_holo, Complex};
use autd3_derive::Gain;

use autd3_driver::{
    acoustics::{propagate, Sphere},
    defined::PI,
    derive::prelude::*,
    geometry::{Geometry, Vector3},
};

use nalgebra::ComplexField;
use rand::seq::SliceRandom;

/// Gain to produce multiple foci with greedy algorithm
///
/// Reference
/// * Suzuki, Shun, et al. "Radiation pressure field reconstruction for ultrasound midair haptics by Greedy algorithm with brute-force search." IEEE Transactions on Haptics 14.4 (2021): 914-921.
#[derive(Gain)]
pub struct Greedy {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    phase_div: usize,
    constraint: Constraint,
}

impl_holo!(Greedy);

impl Greedy {
    pub fn new() -> Self {
        Self {
            foci: vec![],
            amps: vec![],
            phase_div: 16,
            constraint: Constraint::Uniform(1.),
        }
    }

    pub fn with_phase_div(self, phase_div: usize) -> Self {
        Self { phase_div, ..self }
    }

    fn transfer_foci<T: Transducer>(
        trans: &T,
        sound_speed: float,
        attenuation: float,
        foci: &[Vector3],
        res: &mut [Complex],
    ) {
        res.iter_mut().zip(foci.iter()).for_each(|(r, f)| {
            *r = propagate::<Sphere, T>(trans, attenuation, sound_speed, f);
        });
    }

    pub fn phase_div(&self) -> usize {
        self.phase_div
    }
}

impl<T: Transducer> Gain<T> for Greedy {
    fn calc(
        &self,
        geometry: &Geometry<T>,
        filter: GainFilter,
    ) -> Result<HashMap<usize, Vec<Drive>>, AUTDInternalError> {
        let phase_candidates = (0..self.phase_div)
            .map(|i| Complex::new(0., 2.0 * PI * i as float / self.phase_div as float).exp())
            .collect::<Vec<_>>();

        let m = self.foci.len();

        let mut cache = vec![Complex::new(0., 0.); m];

        let amp = self.constraint.convert(1.0, 1.0);
        let mut res: HashMap<usize, Vec<Drive>> = geometry
            .devices()
            .map(|dev| {
                (
                    dev.idx(),
                    vec![Drive { amp, phase: 0.0 }; dev.num_transducers()],
                )
            })
            .collect();
        let mut indices: Vec<_> = match filter {
            GainFilter::All => geometry
                .devices()
                .flat_map(|dev| dev.iter().map(|tr| (dev.idx(), tr.local_idx())))
                .collect(),
            GainFilter::Filter(filter) => geometry
                .devices()
                .filter_map(|dev| {
                    filter.get(&dev.idx()).map(|filter| {
                        dev.iter().filter_map(|tr| {
                            if filter[tr.local_idx()] {
                                Some((dev.idx(), tr.local_idx()))
                            } else {
                                None
                            }
                        })
                    })
                })
                .flatten()
                .collect(),
        };

        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        let mut tmp = vec![Complex::new(0., 0.); m];
        indices.iter().for_each(|&(dev_idx, tr_idx)| {
            Self::transfer_foci(
                &geometry[dev_idx][tr_idx],
                geometry[dev_idx].sound_speed,
                geometry[dev_idx].attenuation,
                &self.foci,
                &mut tmp,
            );
            let (min_idx, _) = phase_candidates.iter().enumerate().fold(
                (0usize, float::INFINITY),
                |acc, (idx, &phase)| {
                    let v = cache.iter().enumerate().fold(0., |acc, (j, c)| {
                        acc + (self.amps[j] - (tmp[j] * phase + c).abs()).abs()
                    });
                    if v < acc.1 {
                        (idx, v)
                    } else {
                        acc
                    }
                },
            );
            let phase = phase_candidates[min_idx];
            cache.iter_mut().zip(tmp.iter()).for_each(|(c, a)| {
                *c += a * phase;
            });
            res.get_mut(&dev_idx).unwrap()[tr_idx].phase = phase.argument() + PI;
        });
        Ok(res)
    }
}

impl Default for Greedy {
    fn default() -> Self {
        Self::new()
    }
}
