/*
 * File: holo.rs
 * Project: benches
 * Created Date: 31/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use criterion::{criterion_group, criterion_main};

use autd3_gain_holo::{test_utilities::bench_utils::*, NalgebraBackend};

criterion_group!(
    benches,
    foci::<NalgebraBackend, 4>,
    devices::<NalgebraBackend, 2>
);
criterion_main!(benches);
