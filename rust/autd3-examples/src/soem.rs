/*
 * File: soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/08/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

mod test_runner;
mod tests;

use anyhow::Result;

use autd3::prelude::*;
use autd3_link_soem::{Config, SOEM};

fn main() -> Result<()> {
    let mut geometry = GeometryBuilder::new().legacy_mode().build();
    geometry.add_device(Vector3::zeros(), Vector3::zeros());
    // let mut geometry = GeometryBuilder::new().build();
    // geometry.add_device(Vector3::zeros(), Vector3::zeros());
    // geometry
    //     .transducers_mut()
    //     .for_each(|t| t.set_frequency(40e3).unwrap());

    let config = Config {
        high_precision_timer: true,
        ..Config::default()
    };
    let link = SOEM::new(config, |msg| {
        eprintln!("unrecoverable error occurred: {}", msg);
        std::process::exit(-1);
    });

    let mut autd = Controller::open(geometry, link).expect("Failed to open");

    autd.check_trials = 50;

    run!(autd);

    Ok(())
}
