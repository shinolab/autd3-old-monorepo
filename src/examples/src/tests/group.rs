/*
 * File: group.rs
 * Project: tests
 * Created Date: 27/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 19/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn group<T: Transducer + 'static, L: Link<T>>(
    autd: &mut Controller<T, L>,
) -> anyhow::Result<bool> {
    autd.send(SilencerConfig::default())?;

    let m = Sine::new(150);

    if autd.geometry().num_devices() > 1 {
        let g1 =
            Focus::new(autd.geometry().center_of(0) + Vector3::new(0., 0., 150.0 * MILLIMETER));
        let g2 = Bessel::new(autd.geometry().center_of(1), Vector3::z(), 18. / 180. * PI);
        let g = Group::by_device(|dev| match dev {
            0 => Some("focus"),
            1.. => Some("bessel"),
            _ => None,
        })
        .set("focus", g1)
        .set("bessel", g2);
        autd.send((m, g))?;
    } else {
        let cx = autd.geometry().center().x;
        let g1 =
            Focus::new(autd.geometry().center_of(0) + Vector3::new(0., 0., 150.0 * MILLIMETER));
        let g2 = Null::new();
        let g = Group::by_transducer(move |tr: &T| {
            if tr.position().x < cx {
                Some("focus")
            } else {
                Some("null")
            }
        })
        .set("focus", g1)
        .set("null", g2);
        autd.send((m, g))?;
    };

    Ok(true)
}
