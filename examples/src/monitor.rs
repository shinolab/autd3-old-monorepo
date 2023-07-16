/*
 * File: soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use std::path::Path;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_monitor::*;

fn main() -> Result<()> {
    let mut autd = Controller::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .open_with(Monitor::default())?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let g = Focus::new(center);
    let m = Square::new(150);

    autd.send((m, g))?;

    autd.link().plot_phase(
        PlotConfig {
            fname: Path::new("phase.png").into(),
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;
    autd.link().plot_field(
        PlotRange {
            x_range: center.x - 50.0..center.x + 50.0,
            y_range: center.y..center.y,
            z_range: center.z..center.z,
            resolution: 1.,
        },
        PlotConfig {
            fname: Path::new("x.png").into(),
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;
    autd.link().plot_field(
        PlotRange {
            x_range: center.x - 20.0..center.x + 20.0,
            y_range: center.y - 30.0..center.y + 30.0,
            z_range: center.z..center.z,
            resolution: 1.,
        },
        PlotConfig {
            fname: Path::new("xy.png").into(),
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;
    autd.link().plot_field(
        PlotRange {
            x_range: center.x..center.x,
            y_range: center.y - 30.0..center.y + 30.0,
            z_range: 0.0..center.z + 50.0,
            resolution: 2.,
        },
        PlotConfig {
            fname: Path::new("yz.png").into(),
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;
    autd.link().plot_field(
        PlotRange {
            x_range: center.x - 30.0..center.x + 30.0,
            y_range: center.y..center.y,
            z_range: 0.0..center.z + 50.0,
            resolution: 2.,
        },
        PlotConfig {
            fname: Path::new("zx.png").into(),
            ticks_step: 20.,
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;

    autd.link().plot_modulation(PlotConfig {
        fname: Path::new("mod.png").into(),
        ..PlotConfig::default()
    })?;

    // Calculate acoustic pressure without plotting
    let p = autd.link().calc_field([center], autd.geometry());
    println!(
        "Acoustic pressure at ({}, {}, {}) = {}",
        center.x, center.y, center.z, p[0]
    );

    // // Plot animation
    // autd.link().begin_animation();
    // let point_num = 200;
    // let radius = 30.0 * MILLIMETER;
    // let stm = FocusSTM::new(1.0).add_foci_from_iter((0..point_num).map(|i| {
    //     let theta = 2.0 * PI * i as float / point_num as float;
    //     let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
    //     center + p
    // }));
    // autd.send(stm)?;
    // autd.link().end_animation(
    //     PlotRange {
    //         x_range: center.x - 40.0..center.x + 40.0,
    //         y_range: center.y - 40.0..center.y + 40.0,
    //         z_range: center.z..center.z,
    //         resolution: 1.,
    //     },
    //     PlotConfig {
    //         fname: Path::new("stm.mp4").into(),
    //         figsize: (8, 6),
    //         dpi: 72,
    //         print_progress: true,
    //         ..PlotConfig::default()
    //     },
    //     autd.geometry(),
    // )?;

    autd.close()?;

    Ok(())
}
