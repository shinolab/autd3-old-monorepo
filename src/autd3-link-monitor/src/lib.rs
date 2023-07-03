/*
 * File: lib.rs
 * Project: src
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod error;

#[cfg(feature = "gpu")]
mod gpu;

use std::{
    cell::{Cell, RefCell},
    ffi::OsString,
    io::Write,
    marker::PhantomData,
    time::Duration,
};

use autd3_core::{
    acoustics::{propagate, Complex, Directivity, Sphere},
    error::AUTDInternalError,
    float,
    geometry::{Geometry, Transducer, Vector3},
    link::Link,
    CPUControlFlags, RxDatagram, TxDatagram, PI,
};
use autd3_firmware_emulator::CPUEmulator;

use error::MonitorError;

use pyo3::prelude::*;

pub struct Monitor<D: Directivity> {
    is_open: bool,
    timeout: Duration,
    cpus: Vec<CPUEmulator>,
    _d: PhantomData<D>,
    animate: Cell<bool>,
    animate_is_stm: Cell<bool>,
    animate_drives: RefCell<Vec<Vec<(u16, u16)>>>,
    #[cfg(feature = "gpu")]
    gpu_compute: Option<gpu::FieldCompute>,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PlotConfig {
    #[pyo3(get)]
    pub figsize: (i32, i32),
    #[pyo3(get)]
    pub dpi: i32,
    #[pyo3(get)]
    pub cbar_position: String,
    #[pyo3(get)]
    pub cbar_size: String,
    #[pyo3(get)]
    pub cbar_pad: String,
    #[pyo3(get)]
    pub fontsize: i32,
    #[pyo3(get)]
    pub ticks_step: float,
    #[pyo3(get)]
    pub cmap: String,
    #[pyo3(get)]
    pub show: bool,
    #[pyo3(get)]
    pub fname: OsString,
    #[pyo3(get)]
    pub interval: i32,
    #[pyo3(get)]
    pub print_progress: bool,
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

    pub fn is_1d(&self) -> bool {
        matches!(
            (self.nx(), self.ny(), self.nz()),
            (_, 1, 1) | (1, _, 1) | (1, 1, _)
        )
    }

    pub fn is_2d(&self) -> bool {
        if self.is_1d() {
            return false;
        }
        matches!(
            (self.nx(), self.ny(), self.nz()),
            (1, _, _) | (_, 1, _) | (_, _, 1)
        )
    }

    pub fn observe(n: usize, start: float, resolution: float) -> Vec<float> {
        (0..n).map(|i| start + resolution * i as float).collect()
    }

    pub fn observe_x(&self) -> Vec<float> {
        Self::observe(self.nx(), self.x_range.start, self.resolution)
    }

    pub fn observe_y(&self) -> Vec<float> {
        Self::observe(self.ny(), self.y_range.start, self.resolution)
    }

    pub fn observe_z(&self) -> Vec<float> {
        Self::observe(self.nz(), self.z_range.start, self.resolution)
    }

    pub fn observe_points(&self) -> Vec<Vector3> {
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

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            figsize: (6, 4),
            dpi: 72,
            cbar_position: "right".to_string(),
            cbar_size: "5%".to_string(),
            cbar_pad: "3%".to_string(),
            fontsize: 12,
            ticks_step: 10.,
            cmap: "jet".to_string(),
            show: false,
            fname: OsString::new(),
            interval: 100,
            print_progress: false,
        }
    }
}

impl Monitor<Sphere> {
    pub fn new() -> Self {
        Self {
            is_open: false,
            timeout: Duration::ZERO,
            cpus: Vec::new(),
            _d: PhantomData,
            animate: Cell::new(false),
            animate_is_stm: Cell::new(false),
            animate_drives: RefCell::new(Vec::new()),
            #[cfg(feature = "gpu")]
            gpu_compute: None,
        }
    }
}

impl<D: Directivity> Monitor<D> {
    pub fn with_directivity<U: Directivity>(self) -> Monitor<U> {
        unsafe { std::mem::transmute(self) }
    }
}

#[cfg(feature = "gpu")]
impl<D: Directivity> Monitor<D> {
    pub fn with_gpu(self, gpu_idx: i32) -> Monitor<D> {
        Self {
            gpu_compute: Some(gpu::FieldCompute::new(gpu_idx)),
            ..self
        }
    }
}

impl Default for Monitor<Sphere> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Directivity> Monitor<D> {
    #[cfg(target_os = "windows")]
    fn initialize_python() -> PyResult<()> {
        let python_exe = which::which("python").unwrap();
        let python_home = python_exe.parent().unwrap();

        let mut python_home = python_home
            .to_str()
            .unwrap()
            .encode_utf16()
            .collect::<Vec<u16>>();
        python_home.push(0);
        unsafe {
            pyo3::ffi::Py_SetPythonHome(python_home.as_ptr());
        }

        pyo3::prepare_freethreaded_python();

        Ok(())
    }

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

    pub fn phases(&self) -> Vec<float> {
        self.phases_of(0)
    }

    pub fn duties(&self) -> Vec<float> {
        self.duties_of(0)
    }

    pub fn modulation_raw(&self) -> Vec<float> {
        self.cpus[0]
            .fpga()
            .modulation()
            .iter()
            .map(|&x| x as float / 255.)
            .collect()
    }

    pub fn modulation(&self) -> Vec<float> {
        self.modulation_raw()
            .iter()
            .map(|&x| (0.5 * PI * x).sin())
            .collect()
    }

    fn plot_1d_impl(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<Complex>,
        resolution: float,
        x_label: &str,
        config: PlotConfig,
    ) -> Result<(), MonitorError> {
        #[cfg(target_os = "windows")]
        {
            Self::initialize_python()?;
        }

        let acoustic_pressures = acoustic_pressures
            .iter()
            .map(|&x| x.norm())
            .collect::<Vec<_>>();

        Python::with_gil(|py| -> PyResult<()> {
            let fun = PyModule::from_code(
                py,
                r#"
import matplotlib.pyplot as plt
import numpy as np

def plot_acoustic_field_1d(axes, acoustic_pressures, observe, resolution, config):
    plot = axes.plot(acoustic_pressures)
    x_label_num = int(np.floor((observe[-1] - observe[0]) / config.ticks_step)) + 1
    x_labels = ['{:.2f}'.format(observe[0] + config.ticks_step * i) for i in range(x_label_num)]
    x_ticks = [config.ticks_step / resolution * i for i in range(x_label_num)]
    axes.set_xticks(np.array(x_ticks), minor=False)
    axes.set_xticklabels(x_labels, minor=False)
    return plot

def plot(observe, acoustic_pressures, resolution, x_label, config):
    plt.rcParams["font.size"] = config.fontsize
    fig = plt.figure(figsize=config.figsize, dpi=config.dpi)
    ax = fig.add_subplot(111)
    plot_acoustic_field_1d(ax, acoustic_pressures, observe, resolution, config)
    ax.set_xlabel(x_label)
    ax.set_ylabel("Amplitude [-]")
    plt.tight_layout()
    if config.fname != "":
        plt.savefig(config.fname, dpi=fig.dpi, bbox_inches='tight')
    if config.show:
        plt.show()
    plt.close()"#,
                "",
                "",
            )?
            .getattr("plot")?;
            fun.call1((
                observe_points,
                acoustic_pressures,
                resolution,
                x_label,
                config,
            ))?;
            Ok(())
        })?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn plot_2d_impl(
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<Complex>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: PlotConfig,
    ) -> Result<(), MonitorError> {
        #[cfg(target_os = "windows")]
        {
            Self::initialize_python()?;
        }

        let acoustic_pressures = acoustic_pressures
            .iter()
            .map(|&x| x.norm())
            .collect::<Vec<_>>();

        Python::with_gil(|py| -> PyResult<()> {
            let fun = PyModule::from_code(
                    py,
r#"
import matplotlib.pyplot as plt
import numpy as np
import mpl_toolkits.axes_grid1

def plot_acoustic_field_2d(axes, acoustic_pressures, observe_x, observe_y, resolution, config):
    heatmap = axes.pcolor(acoustic_pressures, cmap=config.cmap)
    x_label_num = int(np.floor((observe_x[-1] - observe_x[0]) / config.ticks_step)) + 1
    y_label_num = int(np.floor((observe_y[-1] - observe_y[0]) / config.ticks_step)) + 1
    x_labels = ['{:.2f}'.format(observe_x[0] + config.ticks_step * i) for i in range(x_label_num)]
    y_labels = ['{:.2f}'.format(observe_y[0] + config.ticks_step * i) for i in range(y_label_num)]
    x_ticks = [config.ticks_step / resolution * i for i in range(x_label_num)]
    y_ticks = [config.ticks_step / resolution * i for i in range(y_label_num)]
    axes.set_xticks(np.array(x_ticks) + 0.5, minor=False)
    axes.set_yticks(np.array(y_ticks) + 0.5, minor=False)
    axes.set_xticklabels(x_labels, minor=False)
    axes.set_yticklabels(y_labels, minor=False)
    return heatmap

def add_colorbar(fig, axes, mappable, config):
    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes(config.cbar_position, config.cbar_size, pad=config.cbar_pad)
    cbar = fig.colorbar(mappable, cax=cax)
    cbar.ax.set_ylabel("Amplitude [-]")

def plot(observe_x, observe_y, acoustic_pressures, resolution, x_label, y_label, config):
    plt.rcParams["font.size"] = config.fontsize
    fig = plt.figure(figsize=config.figsize, dpi=config.dpi)
    ax = fig.add_subplot(111, aspect="equal")
    nx = len(observe_x)
    ny = len(observe_y)
    acoustic_pressures = np.array(acoustic_pressures).reshape((ny, nx))
    heatmap = plot_acoustic_field_2d(ax, acoustic_pressures, observe_x, observe_y, resolution, config)
    add_colorbar(fig, ax, heatmap, config)
    ax.set_xlabel(x_label)
    ax.set_ylabel(y_label)
    plt.tight_layout()
    if config.fname != "":
        plt.savefig(config.fname, dpi=fig.dpi, bbox_inches='tight')
    if config.show:
        plt.show()
    plt.close()"#, "", "")?
                .getattr("plot")?;
            fun.call1((
                observe_x,
                observe_y,
                acoustic_pressures,
                resolution,
                x_label,
                y_label,
                config,
            ))?;
            Ok(())
        })?;

        Ok(())
    }

    fn plot_modulation_impl(
        modulation: Vec<float>,
        config: PlotConfig,
    ) -> Result<(), MonitorError> {
        #[cfg(target_os = "windows")]
        {
            Self::initialize_python()?;
        }

        Python::with_gil(|py| -> PyResult<()> {
            let fun = PyModule::from_code(
                py,
                r#"
import matplotlib.pyplot as plt
import numpy as np

def plot(modulation, config):
    plt.rcParams["font.size"] = config.fontsize
    fig = plt.figure(figsize=config.figsize, dpi=config.dpi)
    ax = fig.add_subplot(111)
    ax.plot(modulation)
    ax.set_xlim(0, len(modulation))
    ax.set_ylim(0, 1)
    ax.set_xlabel("Index")
    ax.set_ylabel("Modulation")
    plt.tight_layout()
    if config.fname != "":
        plt.savefig(config.fname, dpi=fig.dpi, bbox_inches='tight')
    if config.show:
        plt.show()
    plt.close()"#,
                "",
                "",
            )?
            .getattr("plot")?;
            fun.call1((modulation, config))?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn calc_field<T: Transducer, I: IntoIterator<Item = Vector3>>(
        &self,
        observe_points: I,
        geometry: &Geometry<T>,
    ) -> Vec<Complex> {
        self.calc_field_of::<T, I>(observe_points, geometry, 0)
    }

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
                            .drives(idx)
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
        let sound_speed = geometry.sound_speed;
        observe_points
            .into_iter()
            .map(|target| {
                self.cpus
                    .iter()
                    .enumerate()
                    .fold(Complex::new(0., 0.), |acc, (i, cpu)| {
                        let drives = cpu.fpga().duties_and_phases(idx);
                        acc + geometry.transducers_of(i).zip(drives.iter()).fold(
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

    pub fn plot_field<T: Transducer>(
        &self,
        range: PlotRange,
        config: PlotConfig,
        geometry: &Geometry<T>,
    ) -> Result<(), MonitorError> {
        self.plot_field_of(range, config, geometry, 0)
    }

    pub fn plot_field_of<T: Transducer>(
        &self,
        range: PlotRange,
        config: PlotConfig,
        geometry: &Geometry<T>,
        idx: usize,
    ) -> Result<(), MonitorError> {
        let observe_points = range.observe_points();
        let acoustic_pressures = self.calc_field_of(observe_points, geometry, idx);
        if range.is_1d() {
            let (observe, label) = match (range.nx(), range.ny(), range.nz()) {
                (_, 1, 1) => (range.observe_x(), "x [mm]"),
                (1, _, 1) => (range.observe_y(), "y [mm]"),
                (1, 1, _) => (range.observe_z(), "z [mm]"),
                _ => unreachable!(),
            };
            Self::plot_1d_impl(observe, acoustic_pressures, range.resolution, label, config)
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
            Self::plot_2d_impl(
                observe_x,
                observe_y,
                acoustic_pressures,
                range.resolution,
                x_label,
                y_label,
                config,
            )
        } else {
            Err(MonitorError::InvalidPlotRange)
        }
    }

    pub fn plot_phase<T: Transducer>(
        &self,
        config: PlotConfig,
        geometry: &Geometry<T>,
    ) -> Result<(), MonitorError> {
        self.plot_phase_of(config, geometry, 0)
    }

    pub fn plot_phase_of<T: Transducer>(
        &self,
        config: PlotConfig,
        geometry: &Geometry<T>,
        idx: usize,
    ) -> Result<(), MonitorError> {
        #[cfg(target_os = "windows")]
        {
            Self::initialize_python()?;
        }

        let trans_x = geometry
            .transducers()
            .map(|t| t.position().x)
            .collect::<Vec<_>>();
        let trans_y = geometry
            .transducers()
            .map(|t| t.position().y)
            .collect::<Vec<_>>();
        let trans_phase = self.phases_of(idx);

        Python::with_gil(|py| -> PyResult<()> {
            let fun = PyModule::from_code(
                    py,
r#"
import matplotlib.pyplot as plt
import numpy as np
import mpl_toolkits.axes_grid1

def adjust_marker_size(fig, axes, scat, radius):
    fig.canvas.draw()
    r_pix = axes.transData.transform((radius, radius)) - axes.transData.transform((0, 0))
    sizes = (2 * r_pix * 72 / fig.dpi)**2
    scat.set_sizes(sizes)

def add_colorbar(fig, axes, mappable, config):
    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes(config.cbar_position, config.cbar_size, pad=config.cbar_pad)
    cbar = fig.colorbar(mappable, cax=cax)
    cbar.ax.set_ylim((0, 2 * np.pi))
    cbar.ax.set_yticks([0, np.pi, 2 * np.pi])
    cbar.ax.set_yticklabels(['0', '$\\pi$', '$2\\pi$'])

def plot_phase_2d(fig, axes, trans_x, trans_y, trans_phase, trans_size, config, cmap='jet', marker='o'):
    scat = axes.scatter(trans_x, trans_y, c=trans_phase, cmap=cmap, s=0,
                        marker=marker, vmin=0, vmax=2 * np.pi,
                        clip_on=False)
    add_colorbar(fig, axes, scat, config)
    adjust_marker_size(fig, axes, scat, trans_size / 2)
    return scat

def plot(trans_x, trans_y, trans_phase, config, trans_size):
    plt.rcParams["font.size"] = config.fontsize
    fig = plt.figure(figsize=config.figsize, dpi=config.dpi)
    ax = fig.add_subplot(111, aspect='equal')
    trans_x = np.array(trans_x)
    trans_y = np.array(trans_y)
    x_min = np.min(trans_x) - trans_size / 2
    x_max = np.max(trans_x) + trans_size / 2
    y_min = np.min(trans_y) - trans_size / 2
    y_max = np.max(trans_y) + trans_size / 2
    ax.set_xlim((x_min, x_max))
    ax.set_ylim((y_min, y_max))
    scat = plot_phase_2d(fig, ax, trans_x, trans_y, trans_phase, trans_size, config)
    plt.tight_layout()
    if config.fname != '':
        plt.savefig(config.fname, dpi=fig.dpi, bbox_inches='tight')
    if config.show:
        plt.show()
    plt.close()"#, "", "")?
                .getattr("plot")?;
            fun.call1((
                trans_x,
                trans_y,
                trans_phase,
                config,
                autd3_core::autd3_device::TRANS_SPACING,
            ))?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn plot_modulation(&self, config: PlotConfig) -> Result<(), MonitorError> {
        Self::plot_modulation_impl(self.modulation(), config)?;
        Ok(())
    }

    pub fn plot_modulation_raw(&self, config: PlotConfig) -> Result<(), MonitorError> {
        Self::plot_modulation_impl(self.modulation_raw(), config)?;
        Ok(())
    }

    pub fn begin_animation(&self) {
        self.animate.set(true);
        self.animate_drives.borrow_mut().clear();
    }

    fn calc_field_from_drive<T: Transducer>(
        &self,
        range: PlotRange,
        drives: &[(u16, u16)],
        geometry: &Geometry<T>,
    ) -> Vec<Complex> {
        let observe_points = range.observe_points();

        #[cfg(feature = "gpu")]
        {
            if let Some(gpu) = &self.gpu_compute {
                let sound_speed = geometry.sound_speed;
                let source_drive = drives
                    .iter()
                    .zip(geometry.iter())
                    .map(|(d, t)| {
                        let c = t.cycle();
                        let w = t.wavenumber(sound_speed);
                        let amp = (std::f32::consts::PI * d.0 as f32 / c as f32).sin();
                        let phase = 2. * std::f32::consts::PI * d.1 as f32 / c as f32;
                        [amp, phase, 0., w as f32]
                    })
                    .collect::<Vec<_>>();
                return gpu.calc_field_of::<T, D>(
                    observe_points.into_iter().collect(),
                    geometry,
                    source_drive,
                );
            }
        }
        let sound_speed = geometry.sound_speed;
        observe_points
            .into_iter()
            .map(|target| {
                drives
                    .iter()
                    .zip(geometry.iter())
                    .fold(Complex::new(0., 0.), |acc, (d, t)| {
                        let amp = (PI * d.0 as float / t.cycle() as float).sin();
                        let phase = 2. * PI * d.1 as float / t.cycle() as float;
                        acc + propagate::<D>(
                            t.position(),
                            &t.z_direction(),
                            0.0,
                            t.wavenumber(sound_speed),
                            &target,
                        ) * Complex::from_polar(amp, phase)
                    })
            })
            .collect()
    }

    fn animate_1d_impl(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<Vec<Complex>>,
        resolution: float,
        x_label: &str,
        config: PlotConfig,
    ) -> Result<(), MonitorError> {
        #[cfg(target_os = "windows")]
        {
            Self::initialize_python()?;
        }

        let acoustic_pressures = acoustic_pressures
            .iter()
            .flat_map(|x| x.iter().map(|&x| x.norm()))
            .collect::<Vec<_>>();

        Python::with_gil(|py| -> PyResult<()> {
            let fun = PyModule::from_code(
                py,
                r#"
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.animation import FuncAnimation
import sys

def plot_acoustic_field_1d(axes, acoustic_pressures, observe, resolution, config):
    plot = axes.plot(acoustic_pressures)
    x_label_num = int(np.floor((observe[-1] - observe[0]) / config.ticks_step)) + 1
    x_labels = ['{:.2f}'.format(observe[0] + config.ticks_step * i) for i in range(x_label_num)]
    x_ticks = [config.ticks_step / resolution * i for i in range(x_label_num)]
    axes.set_xticks(np.array(x_ticks), minor=False)
    axes.set_xticklabels(x_labels, minor=False)
    return plot

def plot(observe, acoustic_pressures, resolution, x_label, config):
    plt.rcParams["font.size"] = config.fontsize
    fig = plt.figure(figsize=config.figsize, dpi=config.dpi)
    ax = fig.add_subplot(111)

    nx = len(observe)
    size = len(acoustic_pressures) // nx
    acoustic_pressures = np.array(acoustic_pressures)
    max_p = np.max(acoustic_pressures)
    acoustic_pressures = acoustic_pressures.reshape((size, nx))
    
    def plot_frame(frame):
        ax.cla()
        plot_acoustic_field_1d(ax, acoustic_pressures[frame], observe, resolution, config)
        ax.set_xlabel(x_label)
        ax.set_ylabel("Amplitude [-]")
        ax.set_ylim([0, max_p*1.1])
        plt.tight_layout()
        if config.print_progress:
            percent = 100 * (frame+1) / size
            sys.stdout.write('\r')
            sys.stdout.write(f"Plotted: [{'='*int(percent/(100/30)):30}] {frame+1}/{size} ({int(percent):>3}%)")
            sys.stdout.flush()
    
    ani = FuncAnimation(fig, plot_frame, frames=size, interval=config.interval)    
    if config.fname != "":
        ani.save(config.fname, dpi=fig.dpi)

    if config.show:
        plt.show()
    plt.close()"#,
                "",
                "",
            )?
            .getattr("plot")?;
            fun.call1((
                observe_points,
                acoustic_pressures,
                resolution,
                x_label,
                config,
            ))?;
            Ok(())
        })?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn animate_2d_impl(
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<Vec<Complex>>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: PlotConfig,
    ) -> Result<(), MonitorError> {
        #[cfg(target_os = "windows")]
        {
            Self::initialize_python()?;
        }

        let acoustic_pressures = acoustic_pressures
            .iter()
            .flat_map(|x| x.iter().map(|&x| x.norm()))
            .collect::<Vec<_>>();

        Python::with_gil(|py| -> PyResult<()> {
            let fun = PyModule::from_code(
                py,
                r#"
from matplotlib.animation import FuncAnimation
import matplotlib.pyplot as plt
import numpy as np
import mpl_toolkits.axes_grid1
import sys
from matplotlib.colors import Normalize

def config_heatmap(axes, observe_x, observe_y, resolution, config):
    x_label_num = int(np.floor((observe_x[-1] - observe_x[0]) / config.ticks_step)) + 1
    y_label_num = int(np.floor((observe_y[-1] - observe_y[0]) / config.ticks_step)) + 1
    x_labels = ['{:.2f}'.format(observe_x[0] + config.ticks_step * i) for i in range(x_label_num)]
    y_labels = ['{:.2f}'.format(observe_y[0] + config.ticks_step * i) for i in range(y_label_num)]
    x_ticks = [config.ticks_step / resolution * i for i in range(x_label_num)]
    y_ticks = [config.ticks_step / resolution * i for i in range(y_label_num)]
    axes.set_xticks(np.array(x_ticks) + 0.5, minor=False)
    axes.set_yticks(np.array(y_ticks) + 0.5, minor=False)
    axes.set_xticklabels(x_labels, minor=False)
    axes.set_yticklabels(y_labels, minor=False)

def add_colorbar(fig, axes, mappable, max_p, config):
    divider = mpl_toolkits.axes_grid1.make_axes_locatable(axes)
    cax = divider.append_axes(config.cbar_position, config.cbar_size, pad=config.cbar_pad)
    cbar = fig.colorbar(mappable, cax=cax)
    cbar.ax.set_ylabel("Amplitude [-]")
    cbar.ax.set_ylim(0, max_p)

def plot(observe_x, observe_y, acoustic_pressures, resolution, x_label, y_label, config):
    plt.rcParams["font.size"] = config.fontsize
    fig = plt.figure(figsize=config.figsize, dpi=config.dpi)
    ax = fig.add_subplot(111, aspect="equal")
    nx = len(observe_x)
    ny = len(observe_y)
    size = len(acoustic_pressures) // (nx * ny)
    acoustic_pressures = np.array(acoustic_pressures).reshape((size, ny, nx))

    max_p = np.max(acoustic_pressures)
    heatmap = ax.pcolor(acoustic_pressures[0], cmap=config.cmap, norm=Normalize(vmin=0, vmax=max_p))
    config_heatmap(ax, observe_x, observe_y, resolution, config)
    add_colorbar(fig, ax, heatmap, max_p, config)
    ax.set_xlabel(x_label)
    ax.set_ylabel(y_label)
    plt.tight_layout()

    def plot_frame(frame):
        heatmap.set_array(acoustic_pressures[frame].flatten())
        if config.print_progress:
            percent = 100 * (frame+1) / size
            sys.stdout.write('\r')
            sys.stdout.write(f"Plotted: [{'='*int(percent/(100/30)):30}] {frame+1}/{size} ({int(percent):>3}%)")
            sys.stdout.flush()
        
    ani = FuncAnimation(fig, plot_frame, frames=size, interval=config.interval)    
    if config.fname != "":
        ani.save(config.fname, dpi=fig.dpi)
    if config.show:
        plt.show()
    plt.close()"#,
                "",
                "",
            )?
            .getattr("plot")?;
            fun.call1((
                observe_x,
                observe_y,
                acoustic_pressures,
                resolution,
                x_label,
                y_label,
                config,
            ))?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn end_animation<T: Transducer>(
        &self,
        range: PlotRange,
        config: PlotConfig,
        geometry: &Geometry<T>,
    ) -> Result<(), MonitorError> {
        self.animate.set(false);

        let size = self.animate_drives.borrow().len() as float;
        let acoustic_pressures = self
            .animate_drives
            .borrow()
            .iter()
            .enumerate()
            .map(|(i, d)| {
                if config.print_progress {
                    let percent = 100.0 * (i + 1) as float / size;
                    print!("\r");
                    print!(
                        "Calculated: [{:30}] {}/{} ({}%)",
                        "=".repeat((percent / (100.0 / 30.0)) as usize),
                        i + 1,
                        size as usize,
                        percent as usize
                    );
                    std::io::stdout().flush().unwrap();
                }
                self.calc_field_from_drive(range.clone(), d, geometry)
            })
            .collect::<Vec<_>>();
        if config.print_progress {
            println!();
        }
        let res = if range.is_1d() {
            let (observe, label) = match (range.nx(), range.ny(), range.nz()) {
                (_, 1, 1) => (range.observe_x(), "x [mm]"),
                (1, _, 1) => (range.observe_y(), "y [mm]"),
                (1, 1, _) => (range.observe_z(), "z [mm]"),
                _ => unreachable!(),
            };
            Self::animate_1d_impl(observe, acoustic_pressures, range.resolution, label, config)
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
            Self::animate_2d_impl(
                observe_x,
                observe_y,
                acoustic_pressures,
                range.resolution,
                x_label,
                y_label,
                config,
            )
        } else {
            Err(MonitorError::InvalidPlotRange)
        };

        self.animate_drives.borrow_mut().clear();

        res
    }
}

impl<T: Transducer, D: Directivity> Link<T> for Monitor<D> {
    fn open(&mut self, geometry: &Geometry<T>) -> Result<(), AUTDInternalError> {
        if self.is_open {
            return Ok(());
        }

        self.cpus = geometry
            .device_map()
            .iter()
            .enumerate()
            .map(|(i, &dev)| {
                let mut cpu = CPUEmulator::new(i, dev);
                cpu.init();
                cpu
            })
            .collect();

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

        if self.animate.get() {
            if tx.header().cpu_flag.contains(CPUControlFlags::STM_BEGIN) {
                self.animate_is_stm.set(true);
            }
            if self.animate_is_stm.get() {
                if tx.header().cpu_flag.contains(CPUControlFlags::STM_END) {
                    self.animate_is_stm.set(false);
                    self.animate_drives.borrow_mut().extend(
                        (0..self.cpus[0].fpga().stm_cycle()).map(|idx| {
                            self.cpus
                                .iter()
                                .flat_map(|cpu| cpu.fpga().duties_and_phases(idx))
                                .collect()
                        }),
                    );
                }
            } else {
                self.animate_drives.borrow_mut().push(
                    self.cpus
                        .iter()
                        .flat_map(|cpu| cpu.fpga().duties_and_phases(0))
                        .collect(),
                );
            }
        }

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Ok(false);
        }

        self.cpus.iter_mut().for_each(|cpu| {
            rx[cpu.id()].ack = cpu.ack();
            rx[cpu.id()].msg_id = cpu.msg_id();
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
