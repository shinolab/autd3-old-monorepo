/*
 * File: test_runner.rs
 * Project: src
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! run {
    ($autd:ident) => {{
        use autd3::prelude::*;
        use colored::*;
        use std::io::{self, Write};

        let mut autd = $autd;

        println!("*********************************** Firmware information ****************************************");
        autd.firmware_infos()?.iter().for_each(|firm_info| {
            println!("{}", firm_info);
        });
        println!("*************************************************************************************************");

        autd.send(Clear::new())?;
        autd.send(Synchronize::new())?;

        loop {
            println!("[0]: Single Focal Point Test");
            println!("[1]: BesselBeam Test");
            // println!("[2]: Multiple foci Test");
            println!("[3]: FocusSTM Test");
            println!("[4]: GainSTM Test");
            if autd.geometry().num_devices() == 2 {
                println!("[5]: Grouped Gain Test");
            }
            println!("[9]: Transducer Test");
            println!("[Others]: Finish");
            print!("{}", "Choose number: ".green().bold());
            io::stdout().flush()?;

            let mut s = String::new();
            io::stdin().read_line(&mut s)?;
            match s.trim().parse::<usize>() {
                Ok(0) => focus!(autd),
                Ok(1) => bessel!(autd),
                // Ok(2) => holo!(autd),
                Ok(3) => focus_stm!(autd),
                Ok(4) => gain_stm!(autd),
                Ok(5) => grouped!(autd),
                Ok(9) => trans_test!(autd),
                _ => break,
            };

            println!("press any key to finish...");
            let mut _s = String::new();
            io::stdin().read_line(&mut _s)?;

            let res = autd.send(Stop::new())?;
            println!("stop: {}", res);
        }

        let res = autd.close()?;
        println!("finish: {}", res);
    }};
}
