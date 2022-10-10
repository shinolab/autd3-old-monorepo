/*
 * File: trans_test.rs
 * Project: tests
 * Created Date: 09/05/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/05/2022
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022 Shun Suzuki. All rights reserved.
 *
 */

#[macro_export]
macro_rules! trans_test {
    ($autd:ident) => {{
        let mut silencer_config = SilencerConfig::default();
        $autd.send(&mut silencer_config).flush()?;

        let mut g = TransducerTest::new();
        g.set(0, 0., 1.0);
        g.set(17, 0., 1.0);
        g.set(NUM_TRANS_IN_UNIT * 2 + 0, 0., 1.0);
        g.set(NUM_TRANS_IN_UNIT * 2 + 17, 0., 1.0);

        let mut m = Static::new(0xFF);

        $autd.send(&mut m).send(&mut g)?;
    }};
}
