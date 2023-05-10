/*
 * File: lib.rs
 * Project: src
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{
    error::AUTDInternalError,
    float,
    modulation::{Modulation, ModulationProperty},
};
use autd3_traits::Modulation;
use hound::SampleFormat;

use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioFileError {
    #[error("{0}")]
    Io(std::io::Error),
    #[error("{0}")]
    Wav(hound::Error),
}

impl From<std::io::Error> for AudioFileError {
    fn from(e: std::io::Error) -> Self {
        AudioFileError::Io(e)
    }
}

impl From<hound::Error> for AudioFileError {
    fn from(e: hound::Error) -> Self {
        AudioFileError::Wav(e)
    }
}

#[derive(Modulation, Clone)]
pub struct Wav {
    channels: u16,
    sample_rate: u32,
    raw_buffer: Vec<f32>,
    freq_div: u32,
}

impl Wav {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, AudioFileError> {
        let mut reader = hound::WavReader::open(path)?;
        let channels = reader.spec().channels;
        let sample_format = reader.spec().sample_format;
        let sample_rate = reader.spec().sample_rate;
        let bits_per_sample = reader.spec().bits_per_sample;
        let raw_buffer = match (sample_format, bits_per_sample) {
            (SampleFormat::Int, 8) => reader
                .samples::<i32>()
                .map(|i| (i.unwrap() - std::i8::MIN as i32) as f32 / 255.)
                .collect(),
            (SampleFormat::Int, 16) => reader
                .samples::<i32>()
                .map(|i| (i.unwrap() - std::i16::MIN as i32) as f32 / 65535.)
                .collect(),
            (SampleFormat::Int, 24) => reader
                .samples::<i32>()
                .map(|i| (i.unwrap() - 8388608i32) as f32 / 16777215.)
                .collect(),
            (SampleFormat::Int, 32) => reader
                .samples::<i32>()
                .map(|i| (i.unwrap() as i64 - std::i32::MIN as i64) as f32 / 4294967295.)
                .collect(),
            (SampleFormat::Float, 32) => reader.samples::<f32>().map(|i| i.unwrap()).collect(),
            _ => return Err(AudioFileError::Wav(hound::Error::Unsupported)),
        };

        Ok(Self {
            channels,
            sample_rate,
            raw_buffer,
            freq_div: 40960,
        })
    }
}

impl Modulation for Wav {
    #[allow(clippy::unnecessary_cast)]
    fn calc(self) -> Result<Vec<float>, AUTDInternalError> {
        let sample_rate = self.sampling_freq() as u32;
        let samples = wav_io::resample::linear(
            self.raw_buffer,
            self.channels,
            self.sample_rate,
            sample_rate,
        );
        Ok(samples.iter().map(|&d| d as float).collect())
    }
}
