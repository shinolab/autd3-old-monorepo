/*
 * File: plotters.rs
 * Project: backend
 * Created Date: 16/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 16/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{Backend, PlotConfig};

pub struct PlottersBackend {}

impl Backend for PlottersBackend {
    type PlotConfig = PlotConfig;

    fn new() -> Self {
        todo!()
    }

    fn initialize(&mut self) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn plot_1d(
        observe_points: Vec<f64>,
        acoustic_pressures: Vec<autd3_core::acoustics::Complex>,
        resolution: f64,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn plot_2d(
        observe_x: Vec<f64>,
        observe_y: Vec<f64>,
        acoustic_pressures: Vec<autd3_core::acoustics::Complex>,
        resolution: f64,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn plot_modulation(
        modulation: Vec<f64>,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn plot_phase<T: autd3_core::geometry::Transducer>(
        config: Self::PlotConfig,
        geometry: &autd3_core::geometry::Geometry<T>,
        phases: Vec<f64>,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn animate_1d(
        observe_points: Vec<f64>,
        acoustic_pressures: Vec<Vec<autd3_core::acoustics::Complex>>,
        resolution: f64,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }

    fn animate_2d(
        observe_x: Vec<f64>,
        observe_y: Vec<f64>,
        acoustic_pressures: Vec<Vec<autd3_core::acoustics::Complex>>,
        resolution: f64,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        todo!()
    }
}
