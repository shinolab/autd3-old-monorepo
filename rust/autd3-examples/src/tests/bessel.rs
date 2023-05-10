/*
 * File: bessel.rs
 * Project: tests
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! bessel {
    ($autd:ident) => {{
        $autd.send(SilencerConfig::default())?;

        let center = $autd.geometry().center();
        let dir = Vector3::z();

        let g = Bessel::new(center, dir, 18. / 180. * std::f64::consts::PI);
        let m = Sine::new(150);

        $autd.send((m, g))?;
    }};
}
