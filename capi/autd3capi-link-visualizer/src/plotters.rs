/*
 * File: plotters.rs
 * Project: src
 * Created Date: 12/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 13/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::ffi::{c_char, CStr};

use autd3_link_visualizer::{ListedColorMap, PlotConfig, PlottersBackend, Visualizer};
use autd3capi_def::{
    common::{
        driver::acoustics::directivity::{Sphere, T4010A1},
        *,
    },
    *,
};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerSpherePlotters(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<Sphere>()
        .with_backend::<PlottersBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    LinkBuilderPtr::new(builder)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerT4010A1Plotters(
    use_gpu: bool,
    gpu_idx: i32,
) -> LinkBuilderPtr {
    let mut builder = Visualizer::builder()
        .with_directivity::<T4010A1>()
        .with_backend::<PlottersBackend>();
    if use_gpu {
        builder = builder.with_gpu(gpu_idx);
    }
    LinkBuilderPtr::new(builder)
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PlotConfigPtr(pub ConstPtr);

#[no_mangle]
#[must_use]
#[allow(clippy::box_default)]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigDefault() -> PlotConfigPtr {
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig::default())) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigWithFigSize(
    config: PlotConfigPtr,
    width: u32,
    height: u32,
) -> PlotConfigPtr {
    let config = *Box::from_raw(config.0 as *mut PlotConfig);
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig {
        figsize: (width, height),
        ..config
    })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigWithCBarSize(
    config: PlotConfigPtr,
    cbar_size: float,
) -> PlotConfigPtr {
    let config = *Box::from_raw(config.0 as *mut PlotConfig);
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig {
        cbar_size,
        ..config
    })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigWithFontSize(
    config: PlotConfigPtr,
    font_size: u32,
) -> PlotConfigPtr {
    let config = *Box::from_raw(config.0 as *mut PlotConfig);
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig {
        font_size,
        ..config
    })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigWithLabelAreaSize(
    config: PlotConfigPtr,
    label_area_size: u32,
) -> PlotConfigPtr {
    let config = *Box::from_raw(config.0 as *mut PlotConfig);
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig {
        label_area_size,
        ..config
    })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigWithMargin(
    config: PlotConfigPtr,
    margin: u32,
) -> PlotConfigPtr {
    let config = *Box::from_raw(config.0 as *mut PlotConfig);
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig { margin, ..config })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigWithTicksStep(
    config: PlotConfigPtr,
    ticks_step: float,
) -> PlotConfigPtr {
    let config = *Box::from_raw(config.0 as *mut PlotConfig);
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig {
        ticks_step,
        ..config
    })) as _)
}

#[repr(u8)]
pub enum CMap {
    Jet = 0,
    Viridis = 1,
    Magma = 2,
    Inferno = 3,
    Plasma = 4,
    Cividis = 5,
    Turbo = 6,
    Circle = 7,
    Bluered = 8,
    Breeze = 9,
    Mist = 10,
    Earth = 11,
    Hell = 12,
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigWithCMap(
    config: PlotConfigPtr,
    cmap: CMap,
) -> PlotConfigPtr {
    let config = *Box::from_raw(config.0 as *mut PlotConfig);
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig {
        cmap: match cmap {
            CMap::Jet => autd3_link_visualizer::colormap::jet(),
            CMap::Viridis => ListedColorMap::viridis(),
            CMap::Magma => ListedColorMap::magma(),
            CMap::Inferno => ListedColorMap::inferno(),
            CMap::Plasma => ListedColorMap::plasma(),
            CMap::Cividis => ListedColorMap::cividis(),
            CMap::Turbo => ListedColorMap::turbo(),
            CMap::Circle => ListedColorMap::circle(),
            CMap::Bluered => ListedColorMap::bluered(),
            CMap::Breeze => ListedColorMap::breeze(),
            CMap::Mist => ListedColorMap::mist(),
            CMap::Earth => ListedColorMap::earth(),
            CMap::Hell => ListedColorMap::hell(),
        },
        ..config
    })) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotConfigWithFName(
    config: PlotConfigPtr,
    fname: *const c_char,
    err: *mut c_char,
) -> PlotConfigPtr {
    let config = *Box::from_raw(config.0 as *mut PlotConfig);
    let fname = try_or_return!(CStr::from_ptr(fname).to_str(), err, PlotConfigPtr(NULL));
    PlotConfigPtr(Box::into_raw(Box::new(PlotConfig {
        fname: fname.into(),
        ..config
    })) as _)
}
