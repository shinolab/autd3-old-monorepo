/*
 * File: seq.rs
 * Project: tests
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn focus_stm<T: Transducer, L: Link<T>>(
    autd: &mut Controller<T, L>,
) -> anyhow::Result<bool, AUTDError> {
    use autd3::prelude::*;

    autd.send(SilencerConfig::none())?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let mut stm = FocusSTM::new();
    let point_num = 200;
    let radius = 30.0 * MILLIMETER;
    for i in 0..point_num {
        let theta = 2.0 * PI * i as float / point_num as float;
        let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
        stm.add(center + p);
    }
    stm.set_freq(1.0);

    let m = Static::new();

    autd.send((m, stm))
}

pub fn gain_stm<T: Transducer, L: Link<T>>(
    autd: &mut Controller<T, L>,
) -> anyhow::Result<bool, AUTDError> {
    use autd3::prelude::*;

    autd.send(SilencerConfig::none())?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let mut stm = GainSTM::new();
    let point_num = 50;
    let radius = 30.0 * MILLIMETER;
    for i in 0..point_num {
        let theta = 2.0 * PI * i as float / point_num as float;
        let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);

        let g = Focus::new(center + p);
        stm.add(g);
    }
    stm.set_freq(1.0);

    let m = Static::new();

    autd.send((m, stm))
}
