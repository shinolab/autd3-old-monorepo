/*
 * File: soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_monitor::{Monitor, PlotConfig};

fn main() -> Result<()> {
    let mut autd = Controller::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .open_with(Monitor::new())?;

    autd.send(Clear::new())?;
    autd.send(Synchronize::new())?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let g = Focus::new(center);
    let m = Sine::new(150);

    autd.send((m, g))?;

    autd.link().save_phase(
        "phase.png",
        PlotConfig {
            figsize: (6, 4),
            dpi: 72,
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;
    autd.link().save_field(
        "x.png",
        center.x - 50.0..center.x + 50.0,
        center.y..center.y,
        center.z..center.z,
        1.,
        PlotConfig {
            figsize: (6, 4),
            dpi: 72,
            fontsize: 8,
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;
    let now = std::time::Instant::now();
    autd.link().save_field(
        "xy.png",
        center.x - 20.0..center.x + 20.0,
        center.y - 30.0..center.y + 30.0,
        center.z..center.z,
        1.,
        PlotConfig {
            figsize: (6, 6),
            dpi: 72,
            fontsize: 8,
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;
    println!("elapsed: {} ms", now.elapsed().as_millis());
    autd.link().save_field(
        "yz.png",
        center.x..center.x,
        center.y - 30.0..center.y + 30.0,
        0.0..center.z + 50.0,
        2.,
        PlotConfig {
            figsize: (6, 6),
            dpi: 72,
            fontsize: 8,
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;
    autd.link().save_field(
        "zx.png",
        center.x - 30.0..center.x + 30.0,
        center.y..center.y,
        0.0..center.z + 50.0,
        2.,
        PlotConfig {
            figsize: (6, 6),
            dpi: 72,
            fontsize: 8,
            ticks_step: 20.,
            ..PlotConfig::default()
        },
        autd.geometry(),
    )?;

    autd.link().save_modulation(
        "mod.png",
        PlotConfig {
            figsize: (6, 4),
            dpi: 72,
            ..PlotConfig::default()
        },
    )?;

    autd.close()?;

    Ok(())
}
