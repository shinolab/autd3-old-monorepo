/*
 * File: seq.rs
 * Project: tests
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/06/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! point_stm {
    ($autd:ident) => {{
        use autd3::prelude::*;

        let mut silencer_config = SilencerConfig::none();
        $autd.send(&mut silencer_config).flush()?;

        let center = $autd.geometry().center() + Vector3::new(0., 0., 150.0);

        let mut stm = PointSTM::new();
        let point_num = 200;
        let radius = 30.0;
        for i in 0..point_num {
            let theta = 2.0 * std::f64::consts::PI * i as f64 / point_num as f64;
            let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
            stm.add(center + p, 0)?;
        }
        stm.set_freq(1.0);

        let mut m = Static::new(0xFF);

        $autd.send(&mut m).send(&mut stm)?;
    }};
}

#[macro_export]
macro_rules! gain_stm {
    ($autd:ident) => {{
        use autd3::prelude::*;

        let mut silencer_config = SilencerConfig::none();
        $autd.send(&mut silencer_config).flush()?;

        let center = $autd.geometry().center() + Vector3::new(0., 0., 150.0);

        let mut stm = GainSTM::new();
        let point_num = 200;
        for i in 0..point_num {
            let radius = 30.0;
            let theta = 2.0 * std::f64::consts::PI * i as f64 / point_num as f64;
            let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);

            let g = Focus::new(center + p);
            stm.add(g, $autd.geometry())?;
        }
        stm.set_freq(1.0);

        let mut m = Static::new(0xFF);

        $autd.send(&mut m).send(&mut stm)?;
    }};
}
