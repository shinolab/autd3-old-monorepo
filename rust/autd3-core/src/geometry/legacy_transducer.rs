/*
 * File: legacy_transducer.rs
 * Project: geometry
 * Created Date: 04/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/10/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

use std::f64::consts::PI;

use autd3_driver::Drive;

use super::{Transducer, Vector3};

pub struct LegacyTransducer {
    id: usize,
    pos: Vector3,
    x_direction: Vector3,
    y_direction: Vector3,
    z_direction: Vector3,
    mod_delay: u16,
}

impl Transducer for LegacyTransducer {
    fn new(
        id: usize,
        pos: Vector3,
        x_direction: Vector3,
        y_direction: Vector3,
        z_direction: Vector3,
    ) -> Self {
        Self {
            id,
            pos,
            x_direction,
            y_direction,
            z_direction,
            mod_delay: 0,
        }
    }

    fn align_phase_at(&self, dist: f64, sound_speed: f64) -> f64 {
        let wavelength = sound_speed * 1e3 / self.frequency();
        dist / wavelength
    }

    fn position(&self) -> &Vector3 {
        &self.pos
    }

    fn id(&self) -> usize {
        self.id
    }

    fn x_direction(&self) -> &Vector3 {
        &self.x_direction
    }

    fn y_direction(&self) -> &Vector3 {
        &self.y_direction
    }

    fn z_direction(&self) -> &Vector3 {
        &self.z_direction
    }

    fn cycle(&self) -> u16 {
        4096
    }

    fn frequency(&self) -> f64 {
        40e3
    }

    fn mod_delay(&self) -> u16 {
        self.mod_delay
    }

    fn set_mod_delay(&mut self, delay: u16) {
        self.mod_delay = delay;
    }

    fn wavelength(&self, sound_speed: f64) -> f64 {
        sound_speed * 1e3 / 40e3
    }

    fn wavenumber(&self, sound_speed: f64) -> f64 {
        2.0 * PI * 40e3 / (sound_speed * 1e3)
    }

    fn pack_head(tx: &mut autd3_driver::TxDatagram) {
        autd3_driver::normal_legacy_head(tx);
    }

    fn pack_body(
        phase_sent: &mut bool,
        duty_sent: &mut bool,
        drives: &[Drive],
        tx: &mut autd3_driver::TxDatagram,
    ) -> anyhow::Result<()> {
        autd3_driver::normal_legacy_body(drives, tx)?;
        *phase_sent = true;
        *duty_sent = true;
        Ok(())
    }

    fn gain_stm_max() -> usize {
        autd3_driver::GAIN_STM_LEGACY_BUF_SIZE_MAX
    }
}
