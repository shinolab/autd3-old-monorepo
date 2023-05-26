/*
 * File: greedy.rs
 * Project: combinational
 * Created Date: 03/06/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::{constraint::Constraint, impl_holo, Complex};
use autd3_core::{
    acoustics::{propagate, Sphere},
    error::AUTDInternalError,
    float,
    gain::Gain,
    geometry::{Geometry, Transducer, Vector3},
    Drive, PI,
};
use autd3_traits::Gain;
use nalgebra::ComplexField;
use rand::seq::SliceRandom;

/// Reference
/// * Shun Suzuki, Masahiro Fujiwara, Yasutoshi Makino, and Hiroyuki Shinoda, “Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search,” in IEEE Transactions on Haptics, doi: 10.1109/TOH.2021.3076489
#[derive(Gain)]
pub struct Greedy {
    foci: Vec<Vector3>,
    amps: Vec<float>,
    pub phase_div: usize,
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

    fn transfer_foci<T: Transducer>(
        trans: &T,
        phase: Complex,
        sound_speed: float,
        attenuation: float,
        foci: &[Vector3],
        res: &mut [Complex],
    ) {
        for i in 0..foci.len() {
            res[i] = propagate::<Sphere>(
                trans.position(),
                &trans.z_direction(),
                attenuation,
                trans.wavenumber(sound_speed),
                &foci[i],
            ) * phase;
        }
    }
}

impl<T: Transducer> Gain<T> for Greedy {
    fn calc(&mut self, geometry: &Geometry<T>) -> Result<Vec<Drive>, AUTDInternalError> {
        let phase_candidates = (0..self.phase_div)
            .map(|i| Complex::new(0., 2.0 * PI * i as float / self.phase_div as float).exp())
            .collect::<Vec<_>>();

        let m = self.foci.len();

        let mut tmp = vec![vec![Complex::new(0., 0.); m]; phase_candidates.len()];

        let mut cache = vec![Complex::new(0., 0.); m];

        let sound_speed = geometry.sound_speed;
        let attenuation = geometry.attenuation;

        let amp = self.constraint.convert(1.0, 1.0);
        let mut res = vec![Drive { amp, phase: 0.0 }; geometry.num_transducers()];
        let mut tr_idx: Vec<_> = (0..geometry.num_transducers()).collect();
        let mut rng = rand::thread_rng();
        tr_idx.shuffle(&mut rng);

        for i in tr_idx {
            let trans = &geometry[i];

            let mut min_idx = 0;
            let mut min_v = float::INFINITY;
            for (idx, &phase) in phase_candidates.iter().enumerate() {
                Self::transfer_foci(
                    trans,
                    phase,
                    sound_speed,
                    attenuation,
                    &self.foci,
                    &mut tmp[idx],
                );
                let mut v = 0.0;
                for (j, c) in cache.iter().enumerate() {
                    v += (self.amps[j] - (tmp[idx][j] + c).abs()).abs();
                }

                if v < min_v {
                    min_v = v;
                    min_idx = idx;
                }
            }

            for (j, c) in cache.iter_mut().enumerate() {
                *c += tmp[min_idx][j];
            }

            res[i].phase = phase_candidates[min_idx].argument() + PI;
        }

        Ok(res)
    }
}

impl Default for Greedy {
    fn default() -> Self {
        Self::new()
    }
}
