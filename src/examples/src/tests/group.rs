/*
 * File: group.rs
 * Project: tests
 * Created Date: 15/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn group<T: Transducer + 'static, L: Link>(autd: &mut Controller<T, L>) -> anyhow::Result<bool>
where
    autd3_driver::operation::GainOp<T, Null>: autd3_driver::operation::Operation<T>,
    autd3_driver::operation::GainOp<T, Focus>: autd3_driver::operation::Operation<T>,
{
    let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);

    autd.group(|dev| match dev.idx() {
        0 => Some("null"),
        1 => Some("focus"),
        _ => None,
    })
    .set("null", (Static::new(), Null::new()))?
    .set("focus", (Sine::new(150), Focus::new(center)))?
    .send()?;

    Ok(true)
}
