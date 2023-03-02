/*
 * File: holo.rs
 * Project: tests
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/03/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! holo {
    ($autd:ident) => {{
        use autd3_gain_holo::*;

        let mut silencer_config = SilencerConfig::default();
        $autd.send(&mut silencer_config).flush()?;

        let center = $autd.geometry().center() + Vector3::new(0., 0., 150.0);

        let p = Vector3::new(30., 0., 0.);
        let foci = vec![center + p, center - p];
        let amps = vec![1.0, 1.0];
        let mut m = Sine::new(150);

        println!("[0]: SDP");
        println!("[1]: EVP");
        println!("[2]: Naive");
        println!("[3]: GS");
        println!("[4]: GS-PAT");
        println!("[5]: LM");
        println!("[6]: Greedy");
        println!("[Others]: GS-PAT");
        print!("{}", "Choose number: ".green().bold());
        io::stdout().flush()?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let c = autd3_gain_holo::Normalize {};
        match s.trim().parse::<usize>() {
            Ok(0) => {
                let mut g = SDP::<NalgebraBackend, _>::new(foci, amps, c);
                $autd.send(&mut m).send(&mut g)?;
            }
            Ok(1) => {
                let mut g = EVP::<NalgebraBackend, _>::new(foci, amps, c);
                $autd.send(&mut m).send(&mut g)?;
            }
            Ok(2) => {
                let mut g = Naive::<NalgebraBackend, _>::new(foci, amps, c);
                $autd.send(&mut m).send(&mut g)?;
            }
            Ok(3) => {
                let mut g = GS::<NalgebraBackend, _>::new(foci, amps, c);
                $autd.send(&mut m).send(&mut g)?;
            }
            Ok(4) => {
                let mut g = GSPAT::<NalgebraBackend, _>::new(foci, amps, c);
                $autd.send(&mut m).send(&mut g)?;
            }
            Ok(5) => {
                let mut g = LM::<NalgebraBackend, _>::new(foci, amps, c);
                $autd.send(&mut m).send(&mut g)?;
            }
            Ok(6) => {
                let mut g = Greedy::<_>::new(foci, amps, c);
                $autd.send(&mut m).send(&mut g)?;
            }
            _ => {
                let mut g = GSPAT::<NalgebraBackend, _>::new(foci, amps, c);
                $autd.send(&mut m).send(&mut g)?;
            }
        };
    }};
}
