/*
 * File: grouped.rs
 * Project: tests
 * Created Date: 13/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3::prelude::*;

pub fn grouped<T: Transducer + 'static, L: Link<T>>(
    autd: &mut Controller<T, L>,
) -> anyhow::Result<bool, AUTDError> {
    autd.send(SilencerConfig::default())?;

    let g1 = Focus::new(autd.geometry().center_of(0) + Vector3::new(0., 0., 150.0 * MILLIMETER));
    let g2 = Bessel::new(autd.geometry().center_of(1), Vector3::z(), 18. / 180. * PI);

    let g = Grouped::new().add(0, g1).add(1, g2);

    let m = Sine::new(150);

    autd.send((m, g))
}
