/*
 * File: test_runner.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use crate::tests::*;

use autd3::prelude::*;
use colored::*;
use std::io::{self, Write};

#[allow(clippy::type_complexity)]
pub fn run<T: Transducer, L: Link<T>>(mut autd: Controller<T, L>) -> anyhow::Result<()> {
    println!("*********************************** Firmware information ****************************************");
    autd.firmware_infos()?.iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });
    println!("*************************************************************************************************");

    autd.send(Clear::new())?;
    autd.send(Synchronize::new())?;

    let mut examples: Vec<(
        &'static str,
        &dyn Fn(&mut Controller<T, L>) -> Result<bool, AUTDError>,
    )> = vec![
        ("Single Focal Point Test", &focus),
        ("BesselBeam Test", &bessel),
        ("Wav modulation Test", &audio_file),
        ("FocusSTM Test", &focus_stm),
        ("GainSTM Test", &gain_stm),
        ("Multiple foci Test", &holo),
    ];
    if autd.geometry().num_devices() == 2 {
        examples.push(("Grouped Gain Test", &grouped));
    }

    loop {
        for (i, (name, _)) in examples.iter().enumerate() {
            println!("[{}]: {}", i, name);
        }
        println!("[Others]: Finish");
        print!("{}", "Choose number: ".green().bold());
        io::stdout().flush()?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let res = match s.trim().parse::<usize>() {
            Ok(i) if i < examples.len() => (examples[i].1)(&mut autd),
            _ => break,
        }?;
        if !res {
            eprintln!("Failed to send data");
        }

        println!("press any key to finish...");
        let mut _s = String::new();
        io::stdin().read_line(&mut _s)?;

        let res = autd.send(Stop::new())?;
        println!("stop: {}", res);
    }

    let res = autd.close()?;
    println!("finish: {}", res);

    Ok(())
}
