/*
 * File: holo.rs
 * Project: benches
 * Created Date: 12/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_backend_arrayfire::ArrayFireBackend;
use criterion::{criterion_group, criterion_main};

use autd3_gain_holo::test_utilities::bench_utils::*;

criterion_group!(
    benches,
    foci::<ArrayFireBackend, 4>,
    devices::<ArrayFireBackend, 2>
);
criterion_main!(benches);
