/*
 * File: plane.rs
 * Project: tests
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn plane<T: Transducer, L: Link>(autd: &mut Controller<T, L>) -> anyhow::Result<bool> {
    autd.send(Silencer::default())?;

    let dir = Vector3::z();

    let m = Sine::new(150);
    let g = Plane::new(dir);

    autd.send((m, g))?;

    Ok(true)
}
