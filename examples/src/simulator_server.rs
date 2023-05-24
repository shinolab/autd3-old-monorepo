/*
 * File: simulator_server.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_simulator::{Simulator, ViewerSettings};

fn main() -> ! {
    let mut settings = ViewerSettings::default();
    settings.port = 8080;

    Simulator::new().settings(settings).run()
}
