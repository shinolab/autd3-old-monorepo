/*
 * File: seq.rs
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
macro_rules! focus_stm {
    ($autd:ident) => {{
        use autd3::prelude::*;

        $autd.send(SilencerConfig::none())?;

        let center = $autd.geometry().center() + Vector3::new(0., 0., 150.0);

        let mut stm = FocusSTM::new();
        let point_num = 200;
        let radius = 30.0;
        for i in 0..point_num {
            let theta = 2.0 * std::f64::consts::PI * i as f64 / point_num as f64;
            let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
            stm.add(center + p);
        }
        stm.set_freq(1.0);

        let m = Static::new();

        $autd.send((m, stm))?;
    }};
}

#[macro_export]
macro_rules! gain_stm {
    ($autd:ident) => {{
        use autd3::prelude::*;

        $autd.send(SilencerConfig::none())?;

        let center = $autd.geometry().center() + Vector3::new(0., 0., 150.0);

        let mut stm = GainSTM::new();
        let point_num = 200;
        for i in 0..point_num {
            let radius = 30.0;
            let theta = 2.0 * std::f64::consts::PI * i as f64 / point_num as f64;
            let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);

            let g = Focus::new(center + p);
            stm.add(g);
        }
        stm.set_freq(1.0);

        let m = Static::new();

        $autd.send((m, stm))?;
    }};
}
