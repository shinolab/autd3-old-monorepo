/*
 * File: visualizer.rs
 * Project: src
 * Created Date: 29/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::path::Path;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_visualizer::*;

fn main() -> Result<()> {
    let mut autd = Controller::builder()
        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
        .open_with(Visualizer::default())?;

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

    autd.close()?;

    Ok(())
}
