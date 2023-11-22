/*
 * File: rawpcm.rs
 * Project: src
 * Created Date: 15/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_derive::Modulation;
use autd3_driver::{common::EmitIntensity, derive::prelude::*};

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::error::AudioFileError;

/// Modulation constructed from a raw PCM data
///
/// The raw PCM data must be 8bit unsigned integer.
///
/// The raw PCM data is resampled to the sampling frequency of Modulation.
#[derive(Modulation, Clone)]
pub struct RawPCM {
    sample_rate: u32,
    raw_buffer: Vec<f32>,
    config: SamplingConfiguration,
}

impl RawPCM {
    /// Constructor
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the raw PCM file
    /// * `sample_rate` - Sampling frequency of the raw PCM file
    ///
    pub fn new<P: AsRef<Path>>(path: P, sample_rate: u32) -> Result<Self, AudioFileError> {
        let f = File::open(path)?;
        let mut reader = BufReader::new(f);
        let mut raw_buffer = Vec::new();
        reader.read_to_end(&mut raw_buffer)?;

        let raw_buffer = raw_buffer.iter().map(|&v| v as f32 / 255.).collect();

        Ok(Self {
            sample_rate,
            raw_buffer,
            config: SamplingConfiguration::new_with_frequency(4e3).unwrap(),
        })
    }
}

impl Modulation for RawPCM {
    #[allow(clippy::unnecessary_cast)]
    fn calc(&self) -> Result<Vec<EmitIntensity>, AUTDInternalError> {
        let sample_rate = self.sampling_config().frequency() as u32;
        let samples =
            wav_io::resample::linear(self.raw_buffer.clone(), 1, self.sample_rate, sample_rate);
        Ok(samples
            .iter()
            .map(|&d| EmitIntensity::new((d * 255.).round() as u8))
            .collect())
    }
}
