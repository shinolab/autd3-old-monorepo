/*
 * File: backend.rs
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

use crate::{error::MonitorError, Config};

use autd3_core::{
    acoustics::Complex,
    float,
    geometry::{Geometry, Transducer},
};

pub trait Backend: Send {
    type PlotConfig: Config;

    fn new() -> Self;

    fn initialize(&mut self) -> Result<(), MonitorError>;

    fn plot_1d(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<Complex>,
        resolution: float,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError>;

    #[allow(clippy::too_many_arguments)]
    fn plot_2d(
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<Complex>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError>;

    fn plot_modulation(
        modulation: Vec<float>,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError>;

    fn plot_phase<T: Transducer>(
        config: Self::PlotConfig,
        geometry: &Geometry<T>,
        phases: Vec<float>,
    ) -> Result<(), MonitorError>;

    fn animate_1d(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<Vec<Complex>>,
        resolution: float,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError>;

    fn animate_2d(
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<Vec<Complex>>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError>;
}
