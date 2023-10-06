/*
 * File: test_runner.rs
 * Project: tests
 * Created Date: 27/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use colored::*;
use std::io::{self, Write};

use autd3::prelude::*;
pub use autd3_gain_holo::*;

#[cfg(feature = "cuda")]
pub use autd3_backend_cuda::CUDABackend as Backend;
#[cfg(not(feature = "cuda"))]
pub use NalgebraBackend as Backend;

use super::{
    audio_file::*, bessel::*, custom::*, flag::*, focus::*, group::*, holo::*, plane::*, stm::*,
    transtest::*,
};

pub fn run<T: Transducer + 'static, L: Link>(mut autd: Controller<T, L>) -> anyhow::Result<()>
where
    autd3_driver::operation::GainOp<T, Null>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, Bessel>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, Focus>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, Plane>: autd3_driver::operation::Operation<T>,
    GainSTM<T, Focus>: autd3_driver::datagram::Datagram<T, O2 = autd3_driver::operation::NullOp>,
    <GainSTM<T, Focus> as autd3_driver::datagram::Datagram<T>>::O1:
        autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, SDP<Backend>>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, EVP<Backend>>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, GS<Backend>>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, GSPAT<Backend>>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, Naive<Backend>>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, LM<Backend>>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, Greedy>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, TransducerTest>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, MyUniform>: autd3_driver::operation::Operation<T>,
{
    type Test<'a, T, L> = (
        &'static str,
        &'a dyn Fn(&mut Controller<T, L>) -> anyhow::Result<bool>,
    );

    println!("======== AUTD3 firmware information ========");
    autd.firmware_infos()?.iter().for_each(|firm_info| {
        println!("{}", firm_info);
    });
    println!("============================================");

    let mut examples: Vec<Test<_, _>> = vec![
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
    if autd.geometry().num_devices() >= 2 {
        examples.push(("Group test", &group));
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
                if !(examples[i].1)(&mut autd)? {
                    eprintln!("Failed to send data");
                }
            }
            _ => break,
        }

        println!("press any key to finish...");
        let mut _s = String::new();
        io::stdin().read_line(&mut _s)?;

        if !autd.send(Stop::new())? {
            eprintln!("Failed to stop");
        }
    }

    if !autd.close()? {
        println!("Failed to close");
    }

    Ok(())
}
