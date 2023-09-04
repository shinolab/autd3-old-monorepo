/*
 * File: advanced_phase_transducer.rs
 * Project: geometry
 * Created Date: 31/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    defined::float,
    error::AUTDInternalError,
    fpga::{FPGA_CLK_FREQ, MAX_CYCLE},
};

use super::{Matrix4, Transducer, UnitQuaternion, Vector3, Vector4};

pub struct AdvancedPhaseTransducer {
    local_idx: usize,
    pos: Vector3,
    rot: UnitQuaternion,
    cycle: u16,
    mod_delay: u16,
    amp_filter: float,
    phase_filter: float,
}

impl Transducer for AdvancedPhaseTransducer {
    fn new(local_idx: usize, pos: Vector3, rot: UnitQuaternion) -> Self {
        Self {
            local_idx,
            pos,
            rot,
            cycle: 4096,
            mod_delay: 0,
            amp_filter: 0.,
            phase_filter: 0.,
        }
    }

    fn affine(&mut self, t: Vector3, r: UnitQuaternion) {
        let rot_mat: Matrix4 = From::from(r);
        let trans_mat = rot_mat.append_translation(&t);
        let homo = Vector4::new(self.pos[0], self.pos[1], self.pos[2], 1.0);
        let new_pos = trans_mat * homo;
        self.pos = Vector3::new(new_pos[0], new_pos[1], new_pos[2]);
        self.rot = r * self.rot;
    }

    fn position(&self) -> &Vector3 {
        &self.pos
    }

    fn rotation(&self) -> &UnitQuaternion {
        &self.rot
    }

    fn local_idx(&self) -> usize {
        self.local_idx
    }

    fn amp_filter(&self) -> float {
        self.amp_filter
    }

    fn set_amp_filter(&mut self, value: float) {
        self.amp_filter = value;
    }

    fn phase_filter(&self) -> float {
        self.phase_filter
    }

    fn set_phase_filter(&mut self, value: float) {
        self.phase_filter = value;
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn set_mod_delay(&mut self, delay: u16) {
        self.mod_delay = delay;
    }

    fn frequency(&self) -> float {
        FPGA_CLK_FREQ as float / self.cycle as float
    }

    fn cycle(&self) -> u16 {
        self.cycle
    }
}

impl AdvancedPhaseTransducer {
    /// Set ultrasound cycle
    /// The frequency will be [FPGA_CLK_FREQ] / `cycle`.
    ///
    /// # Arguments
    ///
    /// * `cycle` - Cycle of ultrasound (from 2 to [MAX_CYCLE])
    ///
    pub fn set_cycle(&mut self, cycle: u16) -> Result<(), AUTDInternalError> {
        if !(2..=MAX_CYCLE).contains(&cycle) {
            return Err(AUTDInternalError::CycleOutOfRange(cycle));
        }
        self.cycle = cycle;
        Ok(())
    }

    /// Set ultrasound frequency
    ///
    /// # Arguments
    ///
    /// * `freq` - frequency of ultrasound. The frequency closest to `freq` from the possible frequencies is set.
    ///
    pub fn set_frequency(&mut self, freq: float) -> Result<(), AUTDInternalError> {
        let cycle = (FPGA_CLK_FREQ as float / freq).round() as u16;
        self.set_cycle(cycle)
    }
}

#[cfg(test)]
mod tests {
    use crate::defined::PI;

    use assert_approx_eq::assert_approx_eq;

    use super::*;

    macro_rules! assert_vec3_approx_eq {
        ($a:expr, $b:expr) => {
            assert_approx_eq!($a.x, $b.x, 1e-3);
            assert_approx_eq!($a.y, $b.y, 1e-3);
            assert_approx_eq!($a.z, $b.z, 1e-3);
        };
    }

    #[test]
    fn affine() {
        let mut tr = AdvancedPhaseTransducer::new(0, Vector3::zeros(), UnitQuaternion::identity());

        let t = Vector3::new(40., 50., 60.);
        let rot = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.)
            * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), PI / 2.);
        tr.affine(t, rot);

        let expect_x = Vector3::new(0., 1., 0.);
        let expect_y = Vector3::new(-1., 0., 0.);
        let expect_z = Vector3::new(0., 0., 1.);
        assert_vec3_approx_eq!(expect_x, tr.x_direction());
        assert_vec3_approx_eq!(expect_y, tr.y_direction());
        assert_vec3_approx_eq!(expect_z, tr.z_direction());

        let expect_pos = Vector3::zeros() + t;
        assert_vec3_approx_eq!(expect_pos, tr.position());
    }

    #[test]
    fn cycle() {
        let mut tr = AdvancedPhaseTransducer::new(0, Vector3::zeros(), UnitQuaternion::identity());

        let cycle = 2000;
        let res = tr.set_cycle(cycle);
        assert!(res.is_ok());
        assert_eq!(cycle, tr.cycle());

        let cycle = MAX_CYCLE;
        let res = tr.set_cycle(cycle);
        assert!(res.is_ok());
        assert_eq!(cycle, tr.cycle());

        let cycle = MAX_CYCLE + 1;
        let res = tr.set_cycle(cycle);
        assert!(res.is_err());
    }

    #[test]
    fn freq() {
        let mut tr = AdvancedPhaseTransducer::new(0, Vector3::zeros(), UnitQuaternion::identity());

        let freq = 70e3;
        let res = tr.set_frequency(freq);
        assert!(res.is_ok());
        assert_approx_eq!(freq, tr.frequency(), 100.);

        let freq = 20e3;
        let res = tr.set_frequency(freq);
        assert!(res.is_err());
    }
}
