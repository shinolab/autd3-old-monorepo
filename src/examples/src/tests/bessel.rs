/*
 * File: bessel.rs
 * Project: tests
 * Created Date: 28/05/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn bessel<T: Transducer, L: Link>(autd: &mut Controller<T, L>) -> anyhow::Result<bool>
where
    autd3_driver::operation::GainOp<T, Bessel>: autd3_driver::operation::Operation<T>,
{
    autd.send(Silencer::default())?;

    let center = autd.geometry().center();
    let dir = Vector3::z();

    let g = Bessel::new(center, dir, 18. / 180. * PI);
    let m = Sine::new(150);

    autd.send((m, g))?;

    Ok(true)
}
