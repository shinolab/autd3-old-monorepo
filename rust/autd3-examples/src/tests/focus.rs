/*
 * File: focus.rs
 * Project: tests
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/04/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! focus {
    ($autd:ident) => {{
        let mut silencer_config = SilencerConfig::default();
        $autd.send(&mut silencer_config).flush()?;

        let center = $autd.geometry().center() + Vector3::new(0., 0., 150.0);

        let mut g = Focus::new(center);
        let mut m = Sine::new(150);

        $autd.send(&mut m).send(&mut g)?;
    }};
}
