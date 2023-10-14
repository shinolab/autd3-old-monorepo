/*
 * File: seq.rs
 * Project: tests
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use autd3::prelude::*;

pub fn focus_stm<T: Transducer, L: Link>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.send(Silencer::disable())?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let point_num = 200;
    let radius = 30.0 * MILLIMETER;
    let stm = FocusSTM::new(1.0).add_foci_from_iter((0..point_num).map(|i| {
        let theta = 2.0 * PI * i as float / point_num as float;
        let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
        center + p
    }));

    let m = Static::new();

    autd.send((m, stm))?;

    Ok(true)
}

pub fn gain_stm<T: Transducer, L: Link>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.send(Silencer::disable())?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let point_num = 50;
    let radius = 30.0 * MILLIMETER;

    let stm = GainSTM::new(1.0).add_gains_from_iter((0..point_num).map(|i| {
        let theta = 2.0 * PI * i as float / point_num as float;
        let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
        Focus::new(center + p)
    }));

    let m = Static::new();

    autd.send((m, stm))?;

    Ok(true)
}

pub fn software_stm<T: Transducer, L: Link>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.send(Silencer::disable())?;

    let m = Static::new();

    autd.send(m)?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let freq = 1.;
    let point_num = 100;
    let radius = 30.0 * MILLIMETER;
    let fin = Arc::new(AtomicBool::new(false));
    std::thread::scope(|s| -> anyhow::Result<(), AUTDError> {
        println!("press enter to stop software stm...");
        let fin2 = fin.clone();
        s.spawn(move || {
            let mut _s = String::new();
            io::stdin().read_line(&mut _s).unwrap();

            fin2.store(true, Ordering::Relaxed);
        });
        s.spawn(move || {
            autd.software_stm(move |autd, i, _elapsed| {
                if fin.load(Ordering::Relaxed) {
                    return false; // return false to stop STM
                }

                let theta = 2.0 * PI * (i % point_num) as float / point_num as float;
                let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
                match autd.send(Focus::new(center + p)) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}", e);
                        false
                    }
                }
            })
            .with_timer_strategy(TimerStrategy::NativeTimer)
            .start(std::time::Duration::from_secs_f64(
                1. / freq / point_num as f64,
            ))
        })
        .join()
        .unwrap()
    })?;

    Ok(true)
}
