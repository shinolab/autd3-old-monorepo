/*
 * File: seq.rs
 * Project: tests
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub async fn focus_stm<T: Transducer, L: Link>(
    autd: &mut Controller<T, L>,
) -> anyhow::Result<bool> {
    autd.send(Silencer::disable()).await?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let point_num = 200;
    let radius = 30.0 * MILLIMETER;
    let stm = FocusSTM::new(1.0).add_foci_from_iter((0..point_num).map(|i| {
        let theta = 2.0 * PI * i as float / point_num as float;
        let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
        center + p
    }));

    let m = Static::new();

    autd.send((m, stm)).await?;

    Ok(true)
}

pub async fn gain_stm<T: Transducer, L: Link>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.send(Silencer::disable()).await?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let point_num = 50;
    let radius = 30.0 * MILLIMETER;

    let stm = GainSTM::new(1.0).add_gains_from_iter((0..point_num).map(|i| {
        let theta = 2.0 * PI * i as float / point_num as float;
        let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
        Focus::new(center + p)
    }));

    let m = Static::new();

    autd.send((m, stm)).await?;

    Ok(true)
}
