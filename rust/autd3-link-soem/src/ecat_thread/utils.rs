/*
 * File: utils.rs
 * Project: ecat_thread
 * Created Date: 03/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

pub fn ec_sync(reftime: i64, cycletime: i64, integral: &mut i64) -> i64 {
    let mut delta = (reftime - 50000) % cycletime;
    if delta > (cycletime / 2) {
        delta -= cycletime;
    }
    if delta > 0 {
        *integral += 1;
    }
    if delta < 0 {
        *integral -= 1;
    }
    -(delta / 100) - (*integral / 20)
}
