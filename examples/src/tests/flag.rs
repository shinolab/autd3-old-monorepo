/*
 * File: flag.rs
 * Project: tests
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/06/2023
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

pub fn flag<T: Transducer, L: Link<T>>(autd: &mut Controller<T, L>) -> Result<bool, AUTDError> {
    autd.reads_fpga_info(true);

    println!("press any key to force fan...");
    let mut _s = String::new();
    io::stdin().read_line(&mut _s).unwrap();

    autd.force_fan(true);
    autd.send(UpdateFlags::default())?;

    let fin = Arc::new(AtomicBool::new(false));
    std::thread::scope(|s| {
        s.spawn(|| {
            let prompts = ['-', '/', '|', '\\'];
            let mut idx = 0;
            while !fin.load(Ordering::Relaxed) {
                let states = autd.fpga_info().unwrap();
                println!("{} FPGA Status...", prompts[idx / 1000 % prompts.len()]);
                idx += 1;
                states.iter().enumerate().for_each(|(i, state)| {
                    println!("\x1b[0K[{}]: thermo = {}", i, state.is_thermal_assert());
                });
                print!("\x1b[{}A", states.len() + 1);
            }
        });
        s.spawn(|| {
            println!("press any key to stop checking FPGA status...");
            let mut _s = String::new();
            io::stdin().read_line(&mut _s).unwrap();

            fin.store(true, Ordering::Release);
        });
    });

    autd.reads_fpga_info(false);
    autd.force_fan(false);
    autd.send(UpdateFlags::default())
}
