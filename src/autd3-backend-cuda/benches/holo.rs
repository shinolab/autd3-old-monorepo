/*
 * File: holo.rs
 * Project: benches
 * Created Date: 31/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 01/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod helper;

use autd3_backend_cuda::CUDABackend;
use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};

use autd3_core::{
    float,
    gain::Gain,
    geometry::{LegacyTransducer, Vector3},
    PI,
};
use autd3_gain_holo::*;

use crate::helper::generate_geometry;

const NUM_SAMPLES: usize = 10;

fn gen_foci(n: usize) -> impl Iterator<Item = (Vector3, float)> {
    (0..n).map(move |i| {
        (
            Vector3::new(
                black_box(90. + 10. * (2.0 * PI * i as float / n as float).cos()),
                black_box(70. + 10. * (2.0 * PI * i as float / n as float).sin()),
                black_box(150.),
            ),
            1.0,
        )
    })
}

fn foci<const N: usize>(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("holo-cuda-calc-over-num-devices/num-foci-{}", N));
    group
        .sample_size(NUM_SAMPLES)
        .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let backend = CUDABackend::new().unwrap();
    for size in (1..).take(3) {
        group.bench_with_input(
            BenchmarkId::new("Naive", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    Naive::new(backend.clone())
                        .add_foci_from_iter(gen_foci(N))
                        .calc(geometry)
                        .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("GS", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    GS::new(backend.clone())
                        .add_foci_from_iter(gen_foci(N))
                        .calc(geometry)
                        .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("GSPAT", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    GSPAT::new(backend.clone())
                        .add_foci_from_iter(gen_foci(N))
                        .calc(geometry)
                        .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("EVP", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    if let Err(e) = EVP::new(backend.clone())
                        .add_foci_from_iter(gen_foci(N))
                        .calc(geometry)
                    {
                        eprintln!("{}", e);
                    }
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("SDP", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    if let Err(e) = SDP::new(backend.clone())
                        .add_foci_from_iter(gen_foci(N))
                        .calc(geometry)
                    {
                        eprintln!("{}", e);
                    }
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("LM", size * size),
            &generate_geometry::<LegacyTransducer>(size),
            |b, geometry| {
                b.iter(|| {
                    if let Err(e) = LM::new(backend.clone())
                        .add_foci_from_iter(gen_foci(N))
                        .calc(geometry)
                    {
                        eprintln!("{}", e);
                    }
                })
            },
        );
    }
    group.finish();
}

fn devices<const N: usize>(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!(
        "holo-cuda-calc-over-num-foci/num-devices-{}",
        N * N
    ));
    group
        .sample_size(NUM_SAMPLES)
        .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let backend = CUDABackend::new().unwrap();
    for size in [2].into_iter().chain((2..6).map(|i| i * i)) {
        group.bench_with_input(
            BenchmarkId::new("Naive", size),
            &generate_geometry::<LegacyTransducer>(N),
            |b, geometry| {
                b.iter(|| {
                    Naive::new(backend.clone())
                        .add_foci_from_iter(gen_foci(size))
                        .calc(geometry)
                        .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("GS", size),
            &generate_geometry::<LegacyTransducer>(N),
            |b, geometry| {
                b.iter(|| {
                    GS::new(backend.clone())
                        .add_foci_from_iter(gen_foci(size))
                        .calc(geometry)
                        .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("GSPAT", size),
            &generate_geometry::<LegacyTransducer>(N),
            |b, geometry| {
                b.iter(|| {
                    GSPAT::new(backend.clone())
                        .add_foci_from_iter(gen_foci(size))
                        .calc(geometry)
                        .unwrap();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("EVP", size),
            &generate_geometry::<LegacyTransducer>(N),
            |b, geometry| {
                b.iter(|| {
                    if let Err(e) = EVP::new(backend.clone())
                        .add_foci_from_iter(gen_foci(size))
                        .calc(geometry)
                    {
                        eprintln!("{}", e);
                    }
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("SDP", size),
            &generate_geometry::<LegacyTransducer>(N),
            |b, geometry| {
                b.iter(|| {
                    if let Err(e) = SDP::new(backend.clone())
                        .add_foci_from_iter(gen_foci(size))
                        .calc(geometry)
                    {
                        eprintln!("{}", e);
                    }
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("LM", size),
            &generate_geometry::<LegacyTransducer>(N),
            |b, geometry| {
                b.iter(|| {
                    if let Err(e) = LM::new(backend.clone())
                        .add_foci_from_iter(gen_foci(size))
                        .calc(geometry)
                    {
                        eprintln!("{}", e);
                    }
                })
            },
        );
    }
    group.finish();
}

criterion_group!(benches, foci::<4>, devices::<2>);
criterion_main!(benches);
