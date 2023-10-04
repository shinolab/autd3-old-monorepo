/*
 * File: sound_sources.rs
 * Project: src
 * Created Date: 22/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::f32::consts::PI;

use bytemuck::{Pod, Zeroable};

use crate::{Quaternion, Vector3, Vector4};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Drive {
    pub amp: f32,
    pub phase: f32,
    pub enable: f32,
    pub wave_num: f32,
}

impl Drive {
    pub fn new(amp: f32, phase: f32, enable: f32, frequency: f32, sound_speed: f32) -> Self {
        Self {
            amp,
            phase,
            enable,
            wave_num: Self::to_wave_number(frequency, sound_speed),
        }
    }

    pub fn set_wave_number(&mut self, frequency: f32, sound_speed: f32) {
        self.wave_num = Self::to_wave_number(frequency, sound_speed);
    }

    fn to_wave_number(frequency: f32, sound_speed: f32) -> f32 {
        2.0 * PI * frequency / sound_speed
    }
}

pub struct SoundSources {
    pos: Vec<Vector4>,
    rot: Vec<Quaternion>,
    drive: Vec<Drive>,
    visibilities: Vec<f32>,
}

impl SoundSources {
    pub const fn new() -> Self {
        Self {
            pos: vec![],
            rot: vec![],
            drive: vec![],
            visibilities: vec![],
        }
    }

    pub fn add(&mut self, pos: Vector3, rot: Quaternion, drive: Drive, visibility: f32) {
        self.pos.push(pos.extend(0.));
        self.rot.push(rot);
        self.drive.push(drive);
        self.visibilities.push(visibility);
    }

    pub fn clear(&mut self) {
        self.pos.clear();
        self.rot.clear();
        self.drive.clear();
        self.visibilities.clear();
    }

    pub fn len(&self) -> usize {
        self.pos.len()
    }

    pub fn positions(&self) -> impl ExactSizeIterator<Item = &Vector4> {
        self.pos.iter()
    }

    pub fn rotations(&self) -> impl ExactSizeIterator<Item = &Quaternion> {
        self.rot.iter()
    }

    pub fn drives(&self) -> impl ExactSizeIterator<Item = &Drive> {
        self.drive.iter()
    }

    pub fn drives_mut(&mut self) -> impl ExactSizeIterator<Item = &mut Drive> {
        self.drive.iter_mut()
    }

    pub fn visibilities(&self) -> impl ExactSizeIterator<Item = &f32> {
        self.visibilities.iter()
    }

    pub fn visibilities_mut(&mut self) -> impl ExactSizeIterator<Item = &mut f32> {
        self.visibilities.iter_mut()
    }

    pub fn update_geometry(&mut self, i: usize, pos: Vector3, rot: Quaternion) {
        self.pos[i] = pos.extend(0.);
        self.rot[i] = rot;
    }
}

impl Default for SoundSources {
    fn default() -> Self {
        Self::new()
    }
}
