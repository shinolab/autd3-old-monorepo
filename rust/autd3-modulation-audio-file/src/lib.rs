/*
 * File: lib.rs
 * Project: src
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_core::{error::AUTDInternalError, float, modulation::Modulation};
use autd3_traits::Modulation;

use hound::SampleFormat;
use std::path::{Path, PathBuf};

#[derive(Modulation, Clone)]
pub struct Wav {
    buffer: Vec<float>,
    freq_div: u32,
}

impl Wav {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, hound::Error> {
        let mut reader = hound::WavReader::open(path)?;
        let sample_format = reader.spec().sample_format;
        let sample_rate = reader.spec().sample_rate;
        let bits_per_sample = reader.spec().bits_per_sample;
        let buffer = match (sample_format, bits_per_sample) {
            (SampleFormat::Int, 8) => reader
                .samples::<i32>()
                .map(|i| (i.unwrap() as i32 - std::i8::MIN as i32) as float / 255.)
                .collect(),
            (SampleFormat::Int, 16) => reader
                .samples::<i32>()
                .map(|i| (i.unwrap() as i32 - std::i16::MIN as i32) as float / 65535.)
                .collect(),
            (SampleFormat::Int, 24) => reader
                .samples::<i32>()
                .map(|i| (i.unwrap() as i32 - 8388608i32) as float / 16777215.)
                .collect(),
            (SampleFormat::Int, 32) => reader
                .samples::<i32>()
                .map(|i| (i.unwrap() as i64 - std::i32::MIN as i64) as float / 4294967295.)
                .collect(),
            (SampleFormat::Float, 32) => reader
                .samples::<f32>()
                .map(|i| i.unwrap() as float)
                .collect(),
            _ => return Err(hound::Error::Unsupported),
        };
        Ok(Self {
            buffer,
            freq_div: 40960,
        })
    }
}

impl Modulation for Wav {
    fn calc(self) -> Result<Vec<float>, AUTDInternalError> {
        Ok(self.buffer)
    }
}
