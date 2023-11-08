/*
 * File: lib.rs
 * Project: src
 * Created Date: 12/10/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

pub mod null;
pub mod plotters;
pub mod python;

use std::ffi::c_char;

use autd3_link_visualizer::{
    NullBackend, NullPlotConfig, PlotConfig, PlotRange, PlottersBackend, PyPlotConfig,
    PythonBackend, Visualizer,
};
use autd3capi_def::{
    common::{
        driver::acoustics::directivity::{Sphere, T4010A1},
        *,
    },
    *,
};

#[repr(u8)]
pub enum Backend {
    Plotters = 0,
    Python = 1,
    Null = 2,
}

#[repr(u8)]
pub enum Directivity {
    Sphere = 0,
    T4010A1 = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ConfigPtr(pub ConstPtr);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PlotRangePtr(pub ConstPtr);

macro_rules! match_visualizer {
    ($b:expr, $d:expr, $v: expr, $call:tt,  $( $args:expr ),*) => {
        match $b {
            Backend::Plotters => match $d {
                Directivity::Sphere => cast!($v, Box<Visualizer<Sphere, PlottersBackend>>).$call( $($args),*),
                Directivity::T4010A1 => cast!($v, Box<Visualizer<T4010A1, PlottersBackend>>).$call( $($args),*),
            },
            Backend::Python =>  match $d {
                Directivity::Sphere => cast!($v, Box<Visualizer<Sphere, PythonBackend>>).$call($($args),*),
                Directivity::T4010A1 => cast!($v, Box<Visualizer<T4010A1, PythonBackend>>).$call( $($args),*),
            },
            Backend::Null =>  match $d {
                Directivity::Sphere => cast!($v, Box<Visualizer<Sphere, NullBackend>>).$call($($args),*),
                Directivity::T4010A1 => cast!($v, Box<Visualizer<T4010A1, NullBackend>>).$call( $($args),*),
            },
        }
    };
}

macro_rules! match_visualizer_plot {
    ($b:expr, $d:expr, $v: expr, $call:tt, $config:expr, $( $args:expr ),*) => {
        match $b {
            Backend::Plotters => match $d {
                Directivity::Sphere => cast!($v, Box<Visualizer<Sphere, PlottersBackend>>).$call(*Box::from_raw($config.0 as *mut PlotConfig), $($args),*),
                Directivity::T4010A1 => cast!($v, Box<Visualizer<T4010A1, PlottersBackend>>).$call(*Box::from_raw($config.0 as *mut PlotConfig), $($args),*),
            },
            Backend::Python =>  match $d {
                Directivity::Sphere => cast!($v, Box<Visualizer<Sphere, PythonBackend>>).$call(*Box::from_raw($config.0 as *mut PyPlotConfig), $($args),*),
                Directivity::T4010A1 => cast!($v, Box<Visualizer<T4010A1, PythonBackend>>).$call(*Box::from_raw($config.0 as *mut PyPlotConfig), $($args),*),
            },
            Backend::Null =>  match $d {
                Directivity::Sphere => cast!($v, Box<Visualizer<Sphere, NullBackend>>).$call(*Box::from_raw($config.0 as *mut NullPlotConfig), $($args),*),
                Directivity::T4010A1 => cast!($v, Box<Visualizer<T4010A1, NullBackend>>).$call(*Box::from_raw($config.0 as *mut NullPlotConfig), $($args),*),
            },
        }
    };
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotRange(
    x_min: float,
    x_max: float,
    y_min: float,
    y_max: float,
    z_min: float,
    z_max: float,
    resolution: float,
) -> PlotRangePtr {
    PlotRangePtr(Box::into_raw(Box::new(PlotRange {
        x_range: x_min..x_max,
        y_range: y_min..y_max,
        z_range: z_min..z_max,
        resolution,
    })) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerPhasesOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    idx: u32,
    buf: *mut float,
) -> u32 {
    let idx = idx as usize;
    let m = match_visualizer!(backend, directivity, visualizer, phases_of, idx);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf, m.len());
    }
    m.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerDutiesOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    idx: u32,
    buf: *mut float,
) -> u32 {
    let idx = idx as usize;
    let m = match_visualizer!(backend, directivity, visualizer, duties_of, idx);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf, m.len());
    }
    m.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerModulationRaw(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    buf: *mut float,
) -> u32 {
    let m = match_visualizer!(backend, directivity, visualizer, modulation_raw,);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf, m.len());
    }
    m.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerModulation(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    buf: *mut float,
) -> u32 {
    let m = match_visualizer!(backend, directivity, visualizer, modulation,);
    if !buf.is_null() {
        std::ptr::copy_nonoverlapping(m.as_ptr(), buf, m.len());
    }
    m.len() as _
}

#[no_mangle]
pub unsafe extern "C" fn AUTDLinkVisualizerCalcFieldOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    points: *const float,
    points_len: u32,
    geometry: GeometryPtr,
    idx: u32,
    buf: *mut float,
    err: *mut c_char,
) -> i32 {
    let idx = idx as usize;
    let len = points_len as usize;
    let points = std::slice::from_raw_parts(points as *const Vector3, len);
    let m = try_or_return!(
        match_visualizer!(
            backend,
            directivity,
            visualizer,
            calc_field_of,
            points.iter(),
            cast!(geometry, Geometry),
            idx
        ),
        err,
        AUTD3_ERR
    );
    std::ptr::copy_nonoverlapping(m.as_ptr(), buf as *mut _, m.len());
    AUTD3_TRUE
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotFieldOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
    range: PlotRangePtr,
    geometry: GeometryPtr,
    idx: u32,
    err: *mut c_char,
) -> i32 {
    let idx = idx as usize;
    try_or_return!(
        match_visualizer_plot!(
            backend,
            directivity,
            visualizer,
            plot_field_of,
            config,
            *Box::from_raw(range.0 as *mut PlotRange),
            cast!(geometry, Geometry),
            idx
        ),
        err,
        AUTD3_ERR
    );
    AUTD3_TRUE
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotPhaseOf(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
    geometry: GeometryPtr,
    idx: u32,
    err: *mut c_char,
) -> i32 {
    let idx = idx as usize;
    try_or_return!(
        match_visualizer_plot!(
            backend,
            directivity,
            visualizer,
            plot_phase_of,
            config,
            cast!(geometry, Geometry),
            idx
        ),
        err,
        AUTD3_ERR
    );
    AUTD3_TRUE
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotModulationRaw(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
    err: *mut c_char,
) -> i32 {
    try_or_return!(
        match_visualizer_plot!(
            backend,
            directivity,
            visualizer,
            plot_modulation_raw,
            config,
        ),
        err,
        AUTD3_ERR
    );
    AUTD3_TRUE
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDLinkVisualizerPlotModulation(
    visualizer: LinkPtr,
    backend: Backend,
    directivity: Directivity,
    config: ConfigPtr,
    err: *mut c_char,
) -> i32 {
    try_or_return!(
        match_visualizer_plot!(backend, directivity, visualizer, plot_modulation, config,),
        err,
        AUTD3_ERR
    );
    AUTD3_TRUE
}
