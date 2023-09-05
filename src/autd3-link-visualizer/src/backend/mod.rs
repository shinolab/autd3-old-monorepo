/*
 * File: mod.rs
 * Project: backend
 * Created Date: 16/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod null;
#[cfg(feature = "plotters")]
mod plotters;
#[cfg(feature = "python")]
mod python;

use crate::{error::VisualizerError, Config};

use autd3::{
    driver::{acoustics::Complex, defined::float, geometry::Transducer},
    geometry::Geometry,
};

/// Plotting backend
pub trait Backend: Send {
    type PlotConfig: Config;

    fn new() -> Self;

    fn initialize(&mut self) -> Result<(), VisualizerError>;

    fn plot_1d(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<Complex>,
        resolution: float,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), VisualizerError>;

    #[allow(clippy::too_many_arguments)]
    fn plot_2d(
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<Complex>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), VisualizerError>;

    fn plot_modulation(
        modulation: Vec<float>,
        config: Self::PlotConfig,
    ) -> Result<(), VisualizerError>;

    fn plot_phase<T: Transducer>(
        config: Self::PlotConfig,
        geometry: &Geometry<T>,
        phases: Vec<float>,
    ) -> Result<(), VisualizerError>;
}

#[cfg(feature = "plotters")]
pub use self::plotters::*;
pub use null::*;
#[cfg(feature = "python")]
pub use python::*;
