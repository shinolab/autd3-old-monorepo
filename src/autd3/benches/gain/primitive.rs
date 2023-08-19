/*
 * File: primitive.rs
 * Project: gain
 * Created Date: 31/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod helper;

use autd3_core::gain::GainFilter;
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
                    .calc(geometry, GainFilter::All)
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
                    .calc(geometry, GainFilter::All)
                    .unwrap();
                })
            },
        );
    }
    group.finish();
}

fn focus_cached(c: &mut Criterion) {
    let mut group = c.benchmark_group("gain-calc-over-num-devices/focus-cached");

    for size in (1..).take(5) {
        let geometry = generate_geometry::<LegacyTransducer>(size);
        let g = Focus::new(Vector3::new(
            black_box(90.),
            black_box(70.),
            black_box(150.),
        ))
        .with_cache();
        group.bench_with_input(
            BenchmarkId::new("Legacy", size * size),
            &geometry,
            |b, geometry| {
                b.iter(|| {
                    g.calc(geometry, GainFilter::All).unwrap();
                })
            },
        );
        let geometry = generate_geometry::<AdvancedTransducer>(size);
        let g = Focus::new(Vector3::new(
            black_box(90.),
            black_box(70.),
            black_box(150.),
        ))
        .with_cache();
        group.bench_with_input(
            BenchmarkId::new("Advanced", size * size),
            &geometry,
            |b, geometry| {
                b.iter(|| {
                    g.calc(geometry, GainFilter::All).unwrap();
                })
            },
        );
    }
    group.finish();
}

fn bessel(c: &mut Criterion) {
    let mut group = c.benchmark_group("gain-calc-over-num-devices/bessel");

    for size in (1..).take(5) {
        group.bench_with_input(
            BenchmarkId::new("Legacy", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    Bessel::new(
                        Vector3::new(black_box(90.), black_box(70.), black_box(0.)),
                        Vector3::new(black_box(0.), black_box(0.), black_box(1.)),
                        black_box(0.1),
                    )
                    .calc(geometry, GainFilter::All)
                    .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Advanced", size * size),
            &generate_geometry::<AdvancedTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    Bessel::new(
                        Vector3::new(black_box(90.), black_box(70.), black_box(0.)),
                        Vector3::new(black_box(0.), black_box(0.), black_box(1.)),
                        black_box(0.1),
                    )
                    .calc(geometry, GainFilter::All)
                    .unwrap();
                })
            },
        );
    }
    group.finish();
}

fn plane(c: &mut Criterion) {
    let mut group = c.benchmark_group("gain-calc-over-num-devices/plane");

    for size in (1..).take(5) {
        group.bench_with_input(
            BenchmarkId::new("Legacy", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    Plane::new(Vector3::new(black_box(0.), black_box(0.), black_box(1.)))
                        .calc(geometry, GainFilter::All)
                        .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Advanced", size * size),
            &generate_geometry::<AdvancedTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    Plane::new(Vector3::new(black_box(0.), black_box(0.), black_box(1.)))
                        .calc(geometry, GainFilter::All)
                        .unwrap();
                })
            },
        );
    }
    group.finish();
}

fn group_dev(c: &mut Criterion) {
    let mut group = c.benchmark_group("group");

    group.bench_with_input(
        "group-dev",
        &generate_geometry::<LegacyTransducer>(3),
        |b, geometry| {
            b.iter(|| {
                (0..geometry.num_devices())
                    .fold(Group::by_device(|dev| Some(dev)), |acc, i| {
                        acc.set(
                            i,
                            Focus::new(Vector3::new(
                                black_box(90.),
                                black_box(70.),
                                black_box(150.),
                            )),
                        )
                    })
                    .calc(geometry, GainFilter::All)
                    .unwrap();
            })
        },
    );
    group.finish();
}

fn group_trans(c: &mut Criterion) {
    let mut group = c.benchmark_group("group");

    group.bench_with_input(
        "group-trans",
        &generate_geometry::<LegacyTransducer>(3),
        |b, geometry| {
            b.iter(|| {
                (0..geometry.num_devices())
                    .fold(
                        Group::<_, LegacyTransducer>::by_transducer(|tr| {
                            Some(tr.idx() / autd3_core::autd3_device::AUTD3::NUM_TRANS_IN_UNIT)
                        }),
                        |acc, i| {
                            acc.set(
                                i,
                                Focus::new(Vector3::new(
                                    black_box(90.),
                                    black_box(70.),
                                    black_box(150.),
                                )),
                            )
                        },
                    )
                    .calc(geometry, GainFilter::All)
                    .unwrap();
            })
        },
    );
    group.finish();
}

criterion_group!(
    benches,
    focus,
    focus_cached,
    bessel,
    plane,
    group_dev,
    group_trans
);
criterion_main!(benches);
