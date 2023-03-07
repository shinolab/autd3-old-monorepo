/*
 * File: grouped.rs
 * Project: tests
 * Created Date: 13/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! grouped {
    ($autd:ident) => {{
        let mut silencer_config = SilencerConfig::default();
        $autd
            .timeout(std::time::Duration::from_millis(20))
            .send(&mut silencer_config)
            .flush()?;

        let g1 = Focus::new($autd.geometry().center_of(0) + Vector3::new(0., 0., 150.0));
        let g2 = Bessel::new(
            $autd.geometry().center_of(1),
            Vector3::z(),
            18. / 180. * std::f64::consts::PI,
        );

        let mut g = Grouped::new();
        g.add(0, g1)?;
        g.add(1, g2)?;

        let mut m = Sine::new(150);

        $autd
            .timeout(std::time::Duration::from_millis(20))
            .send(&mut m)
            .send(&mut g)?;
    }};
}
