/*
 * File: trans_test.rs
 * Project: tests
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 09/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! trans_test {
    ($autd:ident) => {{
        $autd.send(SilencerConfig::default())?;

        let mut g = TransducerTest::new();
        g.set(0, 0., 1.0);
        g.set(17, 0., 1.0);
        g.set(NUM_TRANS_IN_UNIT + 0, 0., 1.0);
        g.set(NUM_TRANS_IN_UNIT + 17, 0., 1.0);

        let m = Static::new();

        $autd.send((m, g))?;
    }};
}
