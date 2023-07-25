/*
 * File: audio_file.rs
 * Project: tests
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 26/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn audio_file<T: Transducer, L: Link<T>>(
    autd: &mut Controller<T, L>,
) -> anyhow::Result<bool, AUTDError> {
    autd.send(SilencerConfig::default())?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let g = Focus::new(center);
    const WAV_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/resources/sin150.wav");
    let m = autd3_modulation_audio_file::Wav::new(WAV_FILE)
        .map_err(|e| AUTDError::Internal(e.into()))?;

    // const WAV_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/resources/sin150.dat");
    // let m = autd3_modulation_audio_file::RawPCM::new(WAV_FILE, 4000)
    //     .map_err(|e| AUTDError::Internal(e.into()))?;

    autd.send((m, g))
}
