/*
 * File: transtest.rs
 * Project: tests
 * Created Date: 30/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn transtest<T: Transducer, L: Link<T>>(
    autd: &mut Controller<T, L>,
) -> Result<bool, AUTDError> {
    autd.send(SilencerConfig::default())?;

    let mut g = TransducerTest::new();
    g.set(0, 0., 1.);
    g.set(248, 0., 1.);

    let m = Static::new();

    autd.send((m, g))
}
