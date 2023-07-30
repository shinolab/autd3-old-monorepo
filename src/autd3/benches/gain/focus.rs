/*
 * File: focus.rs
 * Project: gain
 * Created Date: 30/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod helper;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use autd3::{core::gain::Gain, prelude::*};

use crate::helper::generate_geometry;

fn focus(c: &mut Criterion) {
    let mut group = c.benchmark_group("gain-calc-over-num-devices/focus");

    for size in (1..).take(5) {
        group.bench_with_input(
            BenchmarkId::new("Legacy", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    Focus::new(Vector3::new(
                        black_box(90.),
                        black_box(70.),
                        black_box(150.),
                    ))
                    .calc(geometry)
                    .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Advanced", size * size),
            &generate_geometry::<AdvancedTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    Focus::new(Vector3::new(
                        black_box(90.),
                        black_box(70.),
                        black_box(150.),
                    ))
                    .calc(geometry)
                    .unwrap();
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, focus);
criterion_main!(benches);
