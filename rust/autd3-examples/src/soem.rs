/*
 * File: soem.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/03/2023
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
    let mut geometry = GeometryBuilder::new().build();
    geometry.add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))?;

    let config = Config::default();
    let link = SOEM::new(config, |msg| {
        eprintln!("unrecoverable error occurred: {msg}");
        std::process::exit(-1);
    });

    let autd = Controller::open(geometry, link).expect("Failed to open");

    run!(autd);

    Ok(())
}
