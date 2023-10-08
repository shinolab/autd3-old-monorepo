/*
 * File: transtest.rs
 * Project: tests
 * Created Date: 30/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn transtest<T: Transducer, L: Link>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.send(Silencer::default())?;

    let m = Static::new();
    let g = TransducerTest::new().set(0, 0, 0., 1.).set(0, 248, 0., 1.);

    autd.send((m, g))?;

    Ok(true)
}
