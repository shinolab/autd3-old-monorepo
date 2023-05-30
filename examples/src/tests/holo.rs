/*
 * File: holo.rs
 * Project: tests
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;
use autd3_gain_holo::*;

use colored::*;
use std::io::{self, Write};

pub fn holo<T: Transducer, L: Link<T>>(
    autd: &mut Controller<T, L>,
) -> anyhow::Result<bool, AUTDError> {
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
    io::stdout().flush().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let backend = NalgebraBackend::new();

    match s.trim().parse::<usize>() {
        Ok(0) => {
            let mut g = SDP::new(backend);
            g.add_focus(center + p, 1.);
            g.add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(1) => {
            let mut g = EVP::new(backend);
            g.add_focus(center + p, 1.);
            g.add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(2) => {
            let mut g = GS::new(backend);
            g.add_focus(center + p, 1.);
            g.add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(3) => {
            let mut g = GSPAT::new(backend);
            g.add_focus(center + p, 1.);
            g.add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(4) => {
            let mut g = LSS::new(backend);
            g.add_focus(center + p, 1.);
            g.add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(5) => {
            let mut g = LM::new(backend);
            g.add_focus(center + p, 1.);
            g.add_focus(center - p, 1.);
            autd.send((m, g))
        }
        Ok(6) => {
            let mut g = Greedy::new();
            g.add_focus(center + p, 1.);
            g.add_focus(center - p, 1.);
            autd.send((m, g))
        }
        _ => {
            let mut g = GSPAT::new(backend);
            g.add_focus(center + p, 1.);
            g.add_focus(center - p, 1.);
            autd.send((m, g))
        }
    }
}
