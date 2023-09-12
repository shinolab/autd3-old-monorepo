/*
 * File: holo.rs
 * Project: benches
 * Created Date: 12/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 21/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#[cfg(feature = "test-utilities")]
criterion::criterion_group!(
    benches,
    autd3_gain_holo::test_utilities::bench_utils::foci::<
        autd3_backend_arrayfire::ArrayFireBackend,
        4,
    >,
    autd3_gain_holo::test_utilities::bench_utils::devices::<
        autd3_backend_arrayfire::ArrayFireBackend,
        2,
    >
);
#[cfg(feature = "test-utilities")]
criterion::criterion_main!(benches);

#[cfg(not(feature = "test-utilities"))]
fn main() {}
