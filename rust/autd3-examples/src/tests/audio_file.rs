/*
 * File: audio_file.rs
 * Project: tests
 * Created Date: 10/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! audio_file {
    ($autd:ident) => {{
        $autd.send(SilencerConfig::default())?;

        let center = $autd.geometry().center() + Vector3::new(0., 0., 150.0);

        let g = Focus::new(center);
        const WAV_FILE: &'static str =
            concat!(env!("CARGO_MANIFEST_DIR"), "/src/resources/sin150.wav");
        dbg!(WAV_FILE);
        let m = autd3_modulation_audio_file::Wav::new(WAV_FILE)?;

        $autd.send((m, g))?;
    }};
}
