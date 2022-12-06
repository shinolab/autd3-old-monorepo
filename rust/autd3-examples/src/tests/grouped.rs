/*
 * File: grouped.rs
 * Project: tests
 * Created Date: 13/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/12/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! grouped {
    ($autd:ident) => {{
        let mut silencer_config = SilencerConfig::default();
        $autd.send(&mut silencer_config).flush()?;

        let second_dev_tr_idx = $autd.geometry().device_map()[0];
        let first_center = (0..second_dev_tr_idx)
            .map(|i| $autd.geometry()[i].position())
            .sum::<Vector3>()
            / second_dev_tr_idx as f64;
        let second_center = (second_dev_tr_idx..$autd.geometry().num_transducers())
            .map(|i| $autd.geometry()[i].position())
            .sum::<Vector3>()
            / ($autd.geometry().num_transducers() - second_dev_tr_idx) as f64;

        let g1 = Focus::new(first_center + Vector3::new(0., 0., 150.0));
        let g2 = Bessel::new(
            second_center,
            Vector3::z(),
            18. / 180. * std::f64::consts::PI,
        );

        let mut g = Grouped::new();
        g.add(0, g1, $autd.geometry())?;
        g.add(1, g2, $autd.geometry())?;

        let mut m = Sine::new(150);

        $autd.send(&mut m).send(&mut g)?;
    }};
}
