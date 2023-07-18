/*
 * File: null.rs
 * Project: backend
 * Created Date: 17/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{Backend, Config};

/// Backend with no plotting
pub struct NullBackend {}

pub struct NullPlotConfig {}

impl Config for NullPlotConfig {
    fn print_progress(&self) -> bool {
        false
    }
}

impl Backend for NullBackend {
    type PlotConfig = NullPlotConfig;

    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self) -> Result<(), crate::error::MonitorError> {
        Ok(())
    }

    fn plot_1d(
        _observe_points: Vec<f64>,
        _acoustic_pressures: Vec<autd3_core::acoustics::Complex>,
        _resolution: f64,
        _x_label: &str,
        _config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        Err(crate::error::MonitorError::NotSupported)
    }

    fn plot_2d(
        _observe_x: Vec<f64>,
        _observe_y: Vec<f64>,
        _acoustic_pressures: Vec<autd3_core::acoustics::Complex>,
        _resolution: f64,
        _x_label: &str,
        _y_label: &str,
        _config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        Err(crate::error::MonitorError::NotSupported)
    }

    fn plot_modulation(
        _modulation: Vec<f64>,
        _config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        Err(crate::error::MonitorError::NotSupported)
    }

    fn plot_phase<T: autd3_core::geometry::Transducer>(
        _config: Self::PlotConfig,
        _geometry: &autd3_core::geometry::Geometry<T>,
        _phases: Vec<f64>,
    ) -> Result<(), crate::error::MonitorError> {
        Err(crate::error::MonitorError::NotSupported)
    }

    fn animate_1d(
        _observe_points: Vec<f64>,
        _acoustic_pressures: Vec<Vec<autd3_core::acoustics::Complex>>,
        _resolution: f64,
        _x_label: &str,
        _config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        Err(crate::error::MonitorError::NotSupported)
    }

    fn animate_2d(
        _observe_x: Vec<f64>,
        _observe_y: Vec<f64>,
        _acoustic_pressures: Vec<Vec<autd3_core::acoustics::Complex>>,
        _resolution: f64,
        _x_label: &str,
        _y_label: &str,
        _config: Self::PlotConfig,
    ) -> Result<(), crate::error::MonitorError> {
        Err(crate::error::MonitorError::NotSupported)
    }
}
