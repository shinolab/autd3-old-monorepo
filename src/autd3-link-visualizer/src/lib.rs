/*
 * File: lib.rs
 * Project: src
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod error;

#[cfg(feature = "gpu")]
mod gpu;

mod backend;

pub use backend::*;

#[cfg(feature = "plotters")]
pub mod colormap;

use std::{marker::PhantomData, time::Duration};

use autd3::driver::{
    acoustics::{propagate, Complex, Directivity, Sphere},
    cpu::{RxDatagram, TxDatagram},
    defined::{float, PI},
    error::AUTDInternalError,
    geometry::{Device, Geometry, Transducer, Vector3},
    link::Link,
};
use autd3_firmware_emulator::CPUEmulator;

#[cfg(feature = "plotters")]
pub use scarlet::colormap::ListedColorMap;

use error::VisualizerError;

/// Link to monitoring the status of AUTD and acoustic field
pub struct Visualizer<D: Directivity, B: Backend> {
    backend: B,
    is_open: bool,
    timeout: Duration,
    cpus: Vec<CPUEmulator>,
    _d: PhantomData<D>,
    #[cfg(feature = "gpu")]
    gpu_compute: Option<gpu::FieldCompute>,
}

pub trait Config {
    fn print_progress(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct PlotRange {
    pub x_range: std::ops::Range<float>,
    pub y_range: std::ops::Range<float>,
    pub z_range: std::ops::Range<float>,
    pub resolution: float,
}

impl PlotRange {
    fn n(range: &std::ops::Range<float>, resolution: float) -> usize {
        ((range.end - range.start) / resolution).floor() as usize + 1
    }

    fn nx(&self) -> usize {
        Self::n(&self.x_range, self.resolution)
    }

    fn ny(&self) -> usize {
        Self::n(&self.y_range, self.resolution)
    }

    fn nz(&self) -> usize {
        Self::n(&self.z_range, self.resolution)
    }

    fn is_1d(&self) -> bool {
        matches!(
            (self.nx(), self.ny(), self.nz()),
            (_, 1, 1) | (1, _, 1) | (1, 1, _)
        )
    }

    fn is_2d(&self) -> bool {
        if self.is_1d() {
            return false;
        }
        matches!(
            (self.nx(), self.ny(), self.nz()),
            (1, _, _) | (_, 1, _) | (_, _, 1)
        )
    }

    fn observe(n: usize, start: float, resolution: float) -> Vec<float> {
        (0..n).map(|i| start + resolution * i as float).collect()
    }

    fn observe_x(&self) -> Vec<float> {
        Self::observe(self.nx(), self.x_range.start, self.resolution)
    }

    fn observe_y(&self) -> Vec<float> {
        Self::observe(self.ny(), self.y_range.start, self.resolution)
    }

    fn observe_z(&self) -> Vec<float> {
        Self::observe(self.nz(), self.z_range.start, self.resolution)
    }

    fn observe_points(&self) -> Vec<Vector3> {
        match (self.nx(), self.ny(), self.nz()) {
            (_, 1, 1) => self
                .observe_x()
                .iter()
                .map(|&x| Vector3::new(x, self.y_range.start, self.z_range.start))
                .collect(),
            (1, _, 1) => self
                .observe_y()
                .iter()
                .map(|&y| Vector3::new(self.x_range.start, y, self.z_range.start))
                .collect(),
            (1, 1, _) => self
                .observe_z()
                .iter()
                .map(|&z| Vector3::new(self.x_range.start, self.y_range.start, z))
                .collect(),
            (_, _, 1) => itertools::iproduct!(self.observe_y(), self.observe_x())
                .map(|(y, x)| Vector3::new(x, y, self.z_range.start))
                .collect(),
            (_, 1, _) => itertools::iproduct!(self.observe_x(), self.observe_z())
                .map(|(x, z)| Vector3::new(x, self.y_range.start, z))
                .collect(),
            (1, _, _) => itertools::iproduct!(self.observe_z(), self.observe_y())
                .map(|(z, y)| Vector3::new(self.x_range.start, y, z))
                .collect(),
            (_, _, _) => itertools::iproduct!(self.observe_z(), self.observe_y(), self.observe_x())
                .map(|(z, y, x)| Vector3::new(x, y, z))
                .collect(),
        }
    }
}

impl<B: Backend> Visualizer<Sphere, B> {
    pub fn new() -> Self {
        Self {
            backend: B::new(),
            is_open: false,
            timeout: Duration::ZERO,
            cpus: Vec::new(),
            _d: PhantomData,
            #[cfg(feature = "gpu")]
            gpu_compute: None,
        }
    }
}

#[cfg(feature = "plotters")]
impl Visualizer<Sphere, PlottersBackend> {
    /// Constructor with Plotters backend
    pub fn plotters() -> Self {
        Self::new()
    }
}

#[cfg(feature = "python")]
impl Visualizer<Sphere, PythonBackend> {
    /// Constructor with Python backend
    pub fn python() -> Self {
        Self::new()
    }
}

impl Visualizer<Sphere, NullBackend> {
    /// Constructor with Null backend
    pub fn null() -> Self {
        Self::new()
    }
}

impl<D: Directivity, B: Backend> Visualizer<D, B> {
    /// Set directivity
    pub fn with_directivity<U: Directivity>(self) -> Visualizer<U, B> {
        Visualizer {
            backend: self.backend,
            is_open: self.is_open,
            timeout: self.timeout,
            cpus: self.cpus,
            _d: PhantomData,
            #[cfg(feature = "gpu")]
            gpu_compute: self.gpu_compute,
        }
    }
}

#[cfg(feature = "gpu")]
impl<D: Directivity, B: Backend> Visualizer<D, B> {
    /// Enable GPU acceleration
    pub fn with_gpu(self, gpu_idx: i32) -> Visualizer<D, B> {
        Self {
            gpu_compute: Some(gpu::FieldCompute::new(gpu_idx)),
            ..self
        }
    }
}

#[cfg(all(feature = "plotters", not(feature = "python")))]
impl Default for Visualizer<Sphere, PlottersBackend> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(not(feature = "plotters"), feature = "python"))]
impl Default for Visualizer<Sphere, PythonBackend> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(not(feature = "plotters"), not(feature = "python")))]
impl Default for Visualizer<Sphere, NullBackend> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Directivity, B: Backend> Visualizer<D, B> {
    /// Get phases of transducers
    ///
    /// # Arguments
    ///
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn phases_of(&self, idx: usize) -> Vec<float> {
        self.cpus
            .iter()
            .flat_map(|cpu| {
                cpu.fpga()
                    .duties_and_phases(idx)
                    .iter()
                    .zip(cpu.fpga().cycles().iter())
                    .map(|(&d, &c)| 2. * PI * d.1 as float / c as float)
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Get duty ratios of transducers
    ///
    /// # Arguments
    ///
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn duties_of(&self, idx: usize) -> Vec<float> {
        self.cpus
            .iter()
            .flat_map(|cpu| {
                cpu.fpga()
                    .duties_and_phases(idx)
                    .iter()
                    .zip(cpu.fpga().cycles().iter())
                    .map(|(&d, &c)| (PI * d.0 as float / c as float).sin())
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Get phases of transducers
    pub fn phases(&self) -> Vec<float> {
        self.phases_of(0)
    }

    /// Get duty ratios of transducers
    pub fn duties(&self) -> Vec<float> {
        self.duties_of(0)
    }

    /// Get raw modulation data
    pub fn modulation_raw(&self) -> Vec<float> {
        self.cpus[0]
            .fpga()
            .modulation()
            .iter()
            .map(|&x| x as float / 255.)
            .collect()
    }

    /// Get modulation data
    pub fn modulation(&self) -> Vec<float> {
        self.modulation_raw()
            .iter()
            .map(|&x| (0.5 * PI * x).sin())
            .collect()
    }

    /// Calculate acoustic field at specified points
    ///
    /// # Arguments
    ///
    /// * `observe_points` - Observe points iterator
    /// * `geometry` - Geometry
    ///
    pub fn calc_field<T: Transducer, I: IntoIterator<Item = Vector3>>(
        &self,
        observe_points: I,
        geometry: &Geometry<T>,
    ) -> Vec<Complex> {
        self.calc_field_of::<T, I>(observe_points, geometry, 0)
    }

    /// Calculate acoustic field at specified points
    ///
    /// # Arguments
    ///
    /// * `observe_points` - Observe points iterator
    /// * `geometry` - Geometry
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn calc_field_of<T: Transducer, I: IntoIterator<Item = Vector3>>(
        &self,
        observe_points: I,
        geometry: &Geometry<T>,
        idx: usize,
    ) -> Vec<Complex> {
        #[cfg(feature = "gpu")]
        {
            if let Some(gpu) = &self.gpu_compute {
                let sound_speed = geometry.sound_speed;
                let source_drive = self
                    .cpus
                    .iter()
                    .enumerate()
                    .flat_map(|(i, cpu)| {
                        cpu.fpga()
                            .duties_and_phases(idx)
                            .iter()
                            .zip(geometry.transducers_of(i).map(|t| t.cycle()))
                            .zip(
                                geometry
                                    .transducers_of(i)
                                    .map(|t| t.wavenumber(sound_speed)),
                            )
                            .map(|((d, c), w)| {
                                let amp = (std::f32::consts::PI * d.0 as f32 / c as f32).sin();
                                let phase = 2. * std::f32::consts::PI * d.1 as f32 / c as f32;
                                [amp, phase, 0., w as f32]
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                return gpu.calc_field_of::<T, D>(
                    observe_points.into_iter().collect(),
                    geometry,
                    source_drive,
                );
            }
        }
        observe_points
            .into_iter()
            .map(|target| {
                self.cpus
                    .iter()
                    .enumerate()
                    .fold(Complex::new(0., 0.), |acc, (i, cpu)| {
                        let sound_speed = geometry[i].sound_speed;
                        let drives = cpu.fpga().duties_and_phases(idx);
                        acc + geometry[i].iter().zip(drives.iter()).fold(
                            Complex::new(0., 0.),
                            |acc, (t, d)| {
                                let amp = (PI * d.0 as float / t.cycle() as float).sin();
                                let phase = 2. * PI * d.1 as float / t.cycle() as float;
                                acc + propagate::<D>(
                                    t.position(),
                                    &t.z_direction(),
                                    0.0,
                                    t.wavenumber(sound_speed),
                                    &target,
                                ) * Complex::from_polar(amp, phase)
                            },
                        )
                    })
            })
            .collect()
    }

    /// Plot acoustic field
    ///
    /// # Arguments
    ///
    /// * `range` - Plot range
    /// * `config` - Plot configuration
    /// * `geometry` - Geometry
    ///
    pub fn plot_field<T: Transducer>(
        &self,
        range: PlotRange,
        config: B::PlotConfig,
        geometry: &Geometry<T>,
    ) -> Result<(), VisualizerError> {
        self.plot_field_of(range, config, geometry, 0)
    }

    /// Plot acoustic field
    ///
    /// # Arguments
    ///
    /// * `range` - Plot range
    /// * `config` - Plot configuration
    /// * `geometry` - Geometry
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn plot_field_of<T: Transducer>(
        &self,
        range: PlotRange,
        config: B::PlotConfig,
        geometry: &Geometry<T>,
        idx: usize,
    ) -> Result<(), VisualizerError> {
        let observe_points = range.observe_points();
        let acoustic_pressures = self.calc_field_of(observe_points, geometry, idx);
        if range.is_1d() {
            let (observe, label) = match (range.nx(), range.ny(), range.nz()) {
                (_, 1, 1) => (range.observe_x(), "x [mm]"),
                (1, _, 1) => (range.observe_y(), "y [mm]"),
                (1, 1, _) => (range.observe_z(), "z [mm]"),
                _ => unreachable!(),
            };
            B::plot_1d(observe, acoustic_pressures, range.resolution, label, config)
        } else if range.is_2d() {
            let (observe_x, x_label) = match (range.nx(), range.ny(), range.nz()) {
                (_, _, 1) => (range.observe_x(), "x [mm]"),
                (1, _, _) => (range.observe_y(), "y [mm]"),
                (_, 1, _) => (range.observe_z(), "z [mm]"),
                _ => unreachable!(),
            };
            let (observe_y, y_label) = match (range.nx(), range.ny(), range.nz()) {
                (_, _, 1) => (range.observe_y(), "y [mm]"),
                (1, _, _) => (range.observe_z(), "z [mm]"),
                (_, 1, _) => (range.observe_x(), "x [mm]"),
                _ => unreachable!(),
            };
            B::plot_2d(
                observe_x,
                observe_y,
                acoustic_pressures,
                range.resolution,
                x_label,
                y_label,
                config,
            )
        } else {
            Err(VisualizerError::InvalidPlotRange)
        }
    }

    /// Plot transducers with phases
    ///
    /// # Arguments
    ///
    /// * `config` - Plot configuration
    /// * `geometry` - Geometry
    ///
    pub fn plot_phase<T: Transducer>(
        &self,
        config: B::PlotConfig,
        geometry: &Geometry<T>,
    ) -> Result<(), VisualizerError> {
        self.plot_phase_of(config, geometry, 0)
    }

    /// Plot transducers with phases
    ///
    /// # Arguments
    ///
    /// * `config` - Plot configuration
    /// * `geometry` - Geometry
    /// * `idx` - Index of STM. If you use Gain, this value should be 0.
    ///
    pub fn plot_phase_of<T: Transducer>(
        &self,
        config: B::PlotConfig,
        geometry: &Geometry<T>,
        idx: usize,
    ) -> Result<(), VisualizerError> {
        let phases = self.phases_of(idx);
        B::plot_phase(config, geometry, phases)
    }

    /// Plot modulation data
    pub fn plot_modulation(&self, config: B::PlotConfig) -> Result<(), VisualizerError> {
        B::plot_modulation(self.modulation(), config)?;
        Ok(())
    }

    /// Plot raw modulation data
    pub fn plot_modulation_raw(&self, config: B::PlotConfig) -> Result<(), VisualizerError> {
        B::plot_modulation(self.modulation_raw(), config)?;
        Ok(())
    }
}

impl<T: Transducer, D: Directivity, B: Backend> Link<T> for Visualizer<D, B> {
    fn open(&mut self, devices: &[Device<T>]) -> Result<(), AUTDInternalError> {
        if self.is_open {
            return Ok(());
        }

        self.cpus = devices
            .iter()
            .enumerate()
            .map(|(i, dev)| {
                let mut cpu = CPUEmulator::new(i, dev.num_transducers());
                cpu.init();
                cpu
            })
            .collect();

        self.backend.initialize()?;

        self.is_open = true;

        Ok(())
    }

    fn close(&mut self) -> Result<(), AUTDInternalError> {
        if !self.is_open {
            return Ok(());
        }

        self.is_open = false;
        Ok(())
    }

    fn send(&mut self, tx: &TxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Ok(false);
        }

        self.cpus.iter_mut().for_each(|cpu| {
            cpu.send(tx);
        });

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Ok(false);
        }

        self.cpus.iter_mut().for_each(|cpu| {
            rx[cpu.idx()].ack = cpu.ack();
            rx[cpu.idx()].data = cpu.rx_data();
        });

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
