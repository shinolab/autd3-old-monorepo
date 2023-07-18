/*
 * File: test_runner.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/07/2023
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
pub fn run<T: Transducer + 'static, L: Link<T>>(mut autd: Controller<T, L>) -> anyhow::Result<()> {
    println!("*********************************** Firmware information ****************************************");
    autd.firmware_infos()?.iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });
    println!("*************************************************************************************************");

    let mut examples: Vec<(
        &'static str,
        &dyn Fn(&mut Controller<T, L>) -> Result<bool, AUTDError>,
    )> = vec![
        ("Single focus test", &focus),
        ("Bessel beam test", &bessel),
        ("Plane wave test", &plane),
        ("Wav modulation test", &audio_file),
        ("FocusSTM test", &focus_stm),
        ("GainSTM test", &gain_stm),
        ("SoftwareSTM test", &software_stm),
        ("Multiple foci test", &holo),
        ("Custom Gain & Modulation test", &custom),
        ("Flag test", &flag),
        ("TransducerTest test", &transtest),
    ];
    if autd.geometry().num_devices() == 2 {
        examples.push(("Grouped Gain Test", &grouped));
    }

    loop {
        examples.iter().enumerate().for_each(|(i, (name, _))| {
            println!("[{}]: {}", i, name);
        });
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
