/*
 * File: greedy.rs
 * Project: combinational
 * Created Date: 03/06/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{constraint::Constraint, impl_holo, Complex};
use autd3_core::{
    acoustics::{propagate_tr, Sphere},
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive, PI,
};
use autd3_traits::Gain;
use nalgebra::ComplexField;
use rand::seq::SliceRandom;

/// Gain to produce multiple foci with greedy algorithm
///
/// Reference
/// * Shun Suzuki, Masahiro Fujiwara, Yasutoshi Makino, and Hiroyuki Shinoda, “Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search,” in IEEE Transactions on Haptics, doi: 10.1109/TOH.2021.3076489
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
            *r = propagate_tr::<Sphere, T>(trans, attenuation, sound_speed, f);
        });
    }

    pub fn phase_div(&self) -> usize {
        self.phase_div
    }
}

impl<T: Transducer> Gain<T> for Greedy {
    fn calc(&self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let phase_candidates = (0..self.phase_div)
            .map(|i| Complex::new(0., 2.0 * PI * i as float / self.phase_div as float).exp())
            .collect::<Vec<_>>();

        let m = self.foci.len();

        let mut cache = vec![Complex::new(0., 0.); m];

        let amp = self.constraint.convert(1.0, 1.0);
        let mut res = vec![Drive { amp, phase: 0.0 }; geometry.num_transducers()];
        let mut tr_idx: Vec<_> = (0..geometry.num_transducers()).collect();
        let mut rng = rand::thread_rng();
        tr_idx.shuffle(&mut rng);

        let mut tmp = vec![Complex::new(0., 0.); m];
        tr_idx.iter().for_each(|&i| {
            Self::transfer_foci(
                &geometry[i],
                geometry.sound_speed,
                geometry.attenuation,
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
            res[i].phase = phase.argument() + PI;
        });
        Ok(res)
    }
}

impl Default for Greedy {
    fn default() -> Self {
        Self::new()
    }
}
