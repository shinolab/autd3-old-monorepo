/*
 * File: holo.rs
 * Project: tests
 * Created Date: 29/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! holo {
    ($autd:ident) => {{
        use autd3_gain_holo::*;

        $autd.send(SilencerConfig::default())?;

        let m = Sine::new(150);

        let center = $autd.geometry().center() + Vector3::new(0., 0., 150.0);
        let p = Vector3::new(30., 0., 0.);

        println!("[0]: SDP");
        println!("[1]: EVP");
        println!("[2]: GS");
        println!("[3]: GSPAT");
        println!("[4]: LSS");
        println!("[5]: LM");
        println!("[6]: Greedy");
        println!("[Others]: GS-PAT");
        print!("{}", "Choose number: ".green().bold());
        io::stdout().flush()?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        match s.trim().parse::<usize>() {
            Ok(0) => {
                let mut g = SDP::<NalgebraBackend>::new();
                g.add_focus(center + p, 1.);
                g.add_focus(center - p, 1.);
                $autd.send((m, g))?;
            }
            Ok(1) => {
                let mut g = EVP::<NalgebraBackend>::new();
                g.add_focus(center + p, 1.);
                g.add_focus(center - p, 1.);
                $autd.send((m, g))?;
            }
            Ok(2) => {
                let mut g = GS::<NalgebraBackend>::new();
                g.add_focus(center + p, 1.);
                g.add_focus(center - p, 1.);
                $autd.send((m, g))?;
            }
            Ok(3) => {
                let mut g = GSPAT::<NalgebraBackend>::new();
                g.add_focus(center + p, 1.);
                g.add_focus(center - p, 1.);
                $autd.send((m, g))?;
            }
            Ok(4) => {
                let mut g = LSS::<NalgebraBackend>::new();
                g.add_focus(center + p, 1.);
                g.add_focus(center - p, 1.);
                $autd.send((m, g))?;
            }
            Ok(5) => {
                let mut g = LM::<NalgebraBackend>::new();
                g.add_focus(center + p, 1.);
                g.add_focus(center - p, 1.);
                $autd.send((m, g))?;
            }
            Ok(6) => {
                let mut g = Greedy::new();
                g.add_focus(center + p, 1.);
                g.add_focus(center - p, 1.);
                $autd.send((m, g))?;
            }
            _ => {
                let mut g = GSPAT::<NalgebraBackend>::new();
                g.add_focus(center + p, 1.);
                g.add_focus(center - p, 1.);
                $autd.send((m, g))?;
            }
        };
    }};
}
