/*
 * File: holo.rs
 * Project: tests
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;
use autd3_gain_holo::*;

use colored::*;
use std::io::{self, Write};

pub async fn holo<L: Link>(autd: &mut Controller<L>) -> anyhow::Result<bool> {
    autd.send(Silencer::default()).await?;

    let m = Sine::new(150);

    let center = autd.geometry.center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
    let p = Vector3::new(30. * MILLIMETER, 0., 0.);

    println!("[0]: SDP");
    println!("[1]: GS");
    println!("[2]: GSPAT");
    println!("[3]: LSS");
    println!("[4]: LM");
    println!("[5]: Greedy");
    println!("[Others]: GS-PAT");
    print!("{}", "Choose number: ".green().bold());
    io::stdout().flush()?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    let backend = NalgebraBackend::new()?;

    let target_amp = 5e3 * autd.geometry.num_devices() as float * Pascal;
    match s.trim().parse::<usize>() {
        Ok(0) => {
            let g = SDP::new(backend)
                .add_focus(center + p, target_amp)
                .add_focus(center - p, target_amp);
            autd.send((m, g)).await?
        }
        Ok(1) => {
            let g = GS::new(backend)
                .add_focus(center + p, target_amp)
                .add_focus(center - p, target_amp);
            autd.send((m, g)).await?
        }
        Ok(2) => {
            let g = GSPAT::new(backend)
                .add_focus(center + p, target_amp)
                .add_focus(center - p, target_amp);
            autd.send((m, g)).await?
        }
        Ok(3) => {
            let g = LSS::new(backend)
                .add_focus(center + p, target_amp)
                .add_focus(center - p, target_amp);
            autd.send((m, g)).await?
        }
        Ok(4) => {
            let g = LM::new(backend)
                .add_focus(center + p, target_amp)
                .add_focus(center - p, target_amp);
            autd.send((m, g)).await?
        }
        Ok(5) => {
            let g = Greedy::new()
                .add_focus(center + p, target_amp)
                .add_focus(center - p, target_amp);
            autd.send((m, g)).await?
        }
        _ => {
            let g = GSPAT::new(backend)
                .add_focus(center + p, target_amp)
                .add_focus(center - p, target_amp);
            autd.send((m, g)).await?
        }
    };

    Ok(true)
}
