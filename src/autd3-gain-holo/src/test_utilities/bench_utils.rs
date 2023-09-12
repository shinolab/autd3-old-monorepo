/*
 * File: mod_utils.rs
 * Project: test_utilities
 * Created Date: 08/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use criterion::{black_box, AxisScale, BenchmarkId, Criterion, PlotConfiguration};

use autd3::autd3_device::AUTD3;
use autd3_driver::{
    datagram::{Gain, GainFilter},
    defined::{float, PI},
    geometry::{Geometry, IntoDevice, LegacyTransducer, Transducer, Vector3},
};

use crate::*;

const NUM_SAMPLES: usize = 10;
const ENABLE_GREEDY_BENCH: bool = true;
const ENABLE_NAIVE_BENCH: bool = true;
const ENABLE_GS_BENCH: bool = true;
const ENABLE_GSPAT_BENCH: bool = true;
const ENABLE_EVP_BENCH: bool = true;
const ENABLE_SDP_BENCH: bool = true;
const ENABLE_LM_BENCH: bool = true;

pub fn generate_geometry<T: Transducer>(size: usize) -> Geometry<T> {
    Geometry::new(
        (0..size)
            .flat_map(|i| {
                (0..size).map(move |j| {
                    AUTD3::new(
                        Vector3::new(
                            i as float * AUTD3::DEVICE_WIDTH,
                            j as float * AUTD3::DEVICE_HEIGHT,
                            0.,
                        ),
                        Vector3::zeros(),
                    )
                    .into_device(j + i * size)
                })
            })
            .collect(),
    )
}

pub fn gen_foci(n: usize) -> impl Iterator<Item = (Vector3, float)> {
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

pub fn foci<B: LinAlgBackend + 'static, const N: usize>(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("holo-calc-over-num-devices/num-foci-{}", N));
    group
        .sample_size(NUM_SAMPLES)
        .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let backend = B::new().unwrap();
    for size in (1..).take(3) {
        if ENABLE_NAIVE_BENCH {
            group.bench_with_input(
                BenchmarkId::new("Naive", size * size),
                &generate_geometry::<LegacyTransducer>(size),
                |b, geometry| {
                    b.iter(|| {
                        Naive::new(backend.clone())
                            .add_foci_from_iter(gen_foci(N))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_GS_BENCH {
            group.bench_with_input(
                BenchmarkId::new("GS", size * size),
                &generate_geometry::<LegacyTransducer>(size),
                |b, geometry| {
                    b.iter(|| {
                        GS::new(backend.clone())
                            .add_foci_from_iter(gen_foci(N))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_GSPAT_BENCH {
            group.bench_with_input(
                BenchmarkId::new("GSPAT", size * size),
                &generate_geometry::<LegacyTransducer>(size),
                |b, geometry| {
                    b.iter(|| {
                        GSPAT::new(backend.clone())
                            .add_foci_from_iter(gen_foci(N))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_EVP_BENCH {
            group.bench_with_input(
                BenchmarkId::new("EVP", size * size),
                &generate_geometry::<LegacyTransducer>(size),
                |b, geometry| {
                    b.iter(|| {
                        EVP::new(backend.clone())
                            .add_foci_from_iter(gen_foci(N))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_SDP_BENCH {
            group.bench_with_input(
                BenchmarkId::new("SDP", size * size),
                &generate_geometry::<LegacyTransducer>(size),
                |b, geometry| {
                    b.iter(|| {
                        SDP::new(backend.clone())
                            .add_foci_from_iter(gen_foci(N))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_LM_BENCH {
            group.bench_with_input(
                BenchmarkId::new("LM", size * size),
                &generate_geometry::<LegacyTransducer>(size),
                |b, geometry| {
                    b.iter(|| {
                        LM::new(backend.clone())
                            .add_foci_from_iter(gen_foci(N))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_GREEDY_BENCH {
            group.bench_with_input(
                BenchmarkId::new("Greedy", size * size),
                &generate_geometry::<LegacyTransducer>(size),
                |b, geometry| {
                    b.iter(|| {
                        Greedy::new()
                            .add_foci_from_iter(gen_foci(N))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
    }
    group.finish();
}

pub fn devices<B: LinAlgBackend + 'static, const N: usize>(c: &mut Criterion) {
    let mut group = c.benchmark_group(format!("holo-calc-over-num-foci/num-devices-{}", N * N));
    group
        .sample_size(NUM_SAMPLES)
        .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let backend = B::new().unwrap();
    for size in [2].into_iter().chain((2..6).map(|i| i * i)) {
        if ENABLE_NAIVE_BENCH {
            group.bench_with_input(
                BenchmarkId::new("Naive", size),
                &generate_geometry::<LegacyTransducer>(N),
                |b, geometry| {
                    b.iter(|| {
                        Naive::new(backend.clone())
                            .add_foci_from_iter(gen_foci(size))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_GS_BENCH {
            group.bench_with_input(
                BenchmarkId::new("GS", size),
                &generate_geometry::<LegacyTransducer>(N),
                |b, geometry| {
                    b.iter(|| {
                        GS::new(backend.clone())
                            .add_foci_from_iter(gen_foci(size))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_GSPAT_BENCH {
            group.bench_with_input(
                BenchmarkId::new("GSPAT", size),
                &generate_geometry::<LegacyTransducer>(N),
                |b, geometry| {
                    b.iter(|| {
                        GSPAT::new(backend.clone())
                            .add_foci_from_iter(gen_foci(size))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_EVP_BENCH {
            group.bench_with_input(
                BenchmarkId::new("EVP", size),
                &generate_geometry::<LegacyTransducer>(N),
                |b, geometry| {
                    b.iter(|| {
                        EVP::new(backend.clone())
                            .add_foci_from_iter(gen_foci(size))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_SDP_BENCH {
            group.bench_with_input(
                BenchmarkId::new("SDP", size),
                &generate_geometry::<LegacyTransducer>(N),
                |b, geometry| {
                    b.iter(|| {
                        SDP::new(backend.clone())
                            .add_foci_from_iter(gen_foci(size))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_LM_BENCH {
            group.bench_with_input(
                BenchmarkId::new("LM", size),
                &generate_geometry::<LegacyTransducer>(N),
                |b, geometry| {
                    b.iter(|| {
                        LM::new(backend.clone())
                            .add_foci_from_iter(gen_foci(size))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
        if ENABLE_GREEDY_BENCH {
            group.bench_with_input(
                BenchmarkId::new("Greedy", size),
                &generate_geometry::<LegacyTransducer>(N),
                |b, geometry| {
                    b.iter(|| {
                        Greedy::new()
                            .add_foci_from_iter(gen_foci(size))
                            .calc(geometry, GainFilter::All)
                            .unwrap();
                    })
                },
            );
        }
    }
    group.finish();
}
