/*
 * File: focus.rs
 * Project: tests
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn focus<T: Transducer, L: Link<T>>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.send(SilencerConfig::default())?;

    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    let g = Focus::new(center);
    let m = Sine::new(150);

    autd.send((m, g))?;

    Ok(true)
}
