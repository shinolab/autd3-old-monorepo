/*
 * File: test_runner.rs
 * Project: tests
 * Created Date: 27/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use colored::*;
use std::io::{self, Write};

use autd3::prelude::*;
// pub use autd3_gain_holo::*;

// #[cfg(feature = "cuda")]
// pub use autd3_backend_cuda::CUDABackend as Backend;
// #[cfg(not(feature = "cuda"))]
// pub use NalgebraBackend as Backend;

use super::{
    audio_file::*, bessel::*, custom::*, flag::*, focus::*, group::*, 
    // holo::*, 
    plane::*, stm::*,
    transtest::*,
};

pub async fn run<L: Link>(mut autd: Controller<L>) -> anyhow::Result<()> {
    type Test<L> = (
        &'static str,
        fn(
            &'_ mut Controller<L>,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<bool>> + '_>>,
    );

    println!("======== AUTD3 firmware information ========");
    autd.firmware_infos().await?.iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });
    println!("============================================");

    let mut examples: Vec<Test<_>> = vec![
        ("Single focus test", |autd| Box::pin(focus(autd))),
        ("Bessel beam test", |autd| Box::pin(bessel(autd))),
        ("Plane wave test", |autd| Box::pin(plane(autd))),
        ("Wav modulation test", |autd| Box::pin(audio_file(autd))),
        ("FocusSTM test", |autd| Box::pin(focus_stm(autd))),
        ("GainSTM test", |autd| Box::pin(gain_stm(autd))),
        // ("Multiple foci test", |autd| Box::pin(holo(autd))),
        ("Custom Gain & Modulation test", |autd| {
            Box::pin(custom(autd))
        }),
        ("Flag test", |autd| Box::pin(flag(autd))),
        ("TransducerTest test", |autd| Box::pin(transtest(autd))),
    ];
    if autd.geometry.num_devices() >= 2 {
        examples.push(("Group test", |autd| Box::pin(group(autd))));
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
        match s.trim().parse::<usize>() {
            Ok(i) if i < examples.len() => {
                if !(examples[i].1)(&mut autd).await? {
                    eprintln!("Failed to send data");
                }
            }
            _ => break,
        }

        println!("press any key to finish...");
        let mut _s = String::new();
        io::stdin().read_line(&mut _s)?;

        if !autd.send(Stop::new()).await? {
            eprintln!("Failed to stop");
        }
    }

    if !autd.close().await? {
        println!("Failed to close");
    }

    Ok(())
}
