/*
 * File: holo.rs
 * Project: tests
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;
use autd3_gain_holo::*;

use colored::*;
use std::io::{self, Write};

pub fn holo<T: Transducer, L: Link<T>>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.send(SilencerConfig::default())?;

    let m = Sine::new(150);

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
    let p = Vector3::new(30. * MILLIMETER, 0., 0.);

    println!("[0]: SDP");
    println!("[1]: EVP");
    println!("[2]: GS");
    println!("[3]: GSPAT");
    println!("[4]: LSS");
    println!("[5]: LM");
    println!("[6]: Greedy");
    println!("[Others]: GS-PAT");
    print!("{}", "Choose number: ".green().bold());
    io::stdout().flush()?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;

    #[cfg(feature = "cuda")]
    let backend = autd3_backend_cuda::CUDABackend::new()?;
    #[cfg(not(feature = "cuda"))]
    let backend = NalgebraBackend::new()?;

    match s.trim().parse::<usize>() {
        Ok(0) => {
            let g = SDP::new(backend)
                .add_focus(center + p, 1.)
                .add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(1) => {
            let g = EVP::new(backend)
                .add_focus(center + p, 1.)
                .add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(2) => {
            let g = GS::new(backend)
                .add_focus(center + p, 1.)
                .add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(3) => {
            let g = GSPAT::new(backend)
                .add_focus(center + p, 1.)
                .add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(4) => {
            let g = LSS::new(backend)
                .add_focus(center + p, 1.)
                .add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(5) => {
            let g = LM::new(backend)
                .add_focus(center + p, 1.)
                .add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(6) => {
            let g = Greedy::new()
                .add_focus(center + p, 1.)
                .add_focus(center - p, 1.);
            autd.send((m, g))
        }
        _ => {
            let g = GSPAT::new(backend)
                .add_focus(center + p, 1.)
                .add_focus(center - p, 1.);
            autd.send((m, g))
        }
    }?;

    Ok(true)
}
