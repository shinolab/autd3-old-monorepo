/*
 * File: flag.rs
 * Project: tests
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    io,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use autd3::prelude::*;

pub fn flag<T: Transducer, L: Link<T>>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.reads_fpga_info(true);

    println!("press any key to force fan...");
    let mut _s = String::new();
    io::stdin().read_line(&mut _s).unwrap();

    autd.force_fan(true);
    autd.send(UpdateFlags::default())?;

    let fin = Arc::new(AtomicBool::new(false));
    std::thread::scope(|s| -> anyhow::Result<bool, AUTDError> {
        s.spawn(|| {
            println!("press any key to stop checking FPGA status...");
            let mut _s = String::new();
            io::stdin().read_line(&mut _s).unwrap();

            fin.store(true, Ordering::Release);
        });
        s.spawn(|| -> anyhow::Result<bool, AUTDError> {
            let prompts = ['-', '/', '|', '\\'];
            let mut idx = 0;
            while !fin.load(Ordering::Relaxed) {
                let states = autd.fpga_info()?;
                println!("{} FPGA Status...", prompts[idx / 1000 % prompts.len()]);
                idx += 1;
                states.iter().enumerate().for_each(|(i, state)| {
                    println!("\x1b[0K[{}]: thermo = {}", i, state.is_thermal_assert());
                });
                print!("\x1b[{}A", states.len() + 1);
            }
            Ok(true)
        })
        .join()
        .unwrap()
    })?;

    autd.reads_fpga_info(false);
    autd.force_fan(false);
    autd.send(UpdateFlags::default())?;

    Ok(true)
}
