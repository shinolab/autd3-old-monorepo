/*
 * File: null.rs
 * Project: src
 * Created Date: 13/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_link_visualizer::{NullBackend, NullPlotConfig, Visualizer};
use autd3capi_def::{
    common::{
        driver::acoustics::directivity::{Sphere, T4010A1},
        *,
    },
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerSphereNull(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<Sphere>()
        .with_backend::<NullBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    LinkBuilderPtr::new(builder)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerT4010A1Null(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<T4010A1>()
        .with_backend::<NullBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    LinkBuilderPtr::new(builder)
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NullPlotConfigPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerNullPlotConfigDefault() -> NullPlotConfigPtr {
    NullPlotConfigPtr(Box::into_raw(Box::new(NullPlotConfig {})) as _)
}
