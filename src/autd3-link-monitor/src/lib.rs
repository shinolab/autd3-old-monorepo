/*
 * File: lib.rs
 * Project: src
 * Created Date: 14/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 14/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

mod error;

use std::{ffi::OsStr, path::Path, time::Duration};

use autd3_core::{
    acoustics::{propagate, Complex},
    error::AUTDInternalError,
    float,
    geometry::{Geometry, Transducer, Vector3},
    link::Link,
    RxDatagram, TxDatagram, PI,
};
use autd3_firmware_emulator::CPUEmulator;

use error::MonitorError;

use pyo3::prelude::*;

pub struct Monitor {
    is_open: bool,
    timeout: Duration,
    cpus: Vec<CPUEmulator>,
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
        }
    }
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            is_open: false,
            timeout: Duration::ZERO,
            cpus: Vec::new(),
        }
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor {
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
                let (_, phase) = cpu.fpga().drives(idx);
                let cycle = cpu.fpga().cycles();
                phase
                    .iter()
                    .zip(cycle.iter())
                    .map(|(&p, &c)| 2. * PI * p as float / c as float)
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn duties_of(&self, idx: usize) -> Vec<float> {
        self.cpus
            .iter()
            .flat_map(|cpu| {
                let (duty, _) = cpu.fpga().drives(idx);
                let cycle = cpu.fpga().cycles();
                duty.iter()
                    .zip(cycle.iter())
                    .map(|(&d, &c)| (PI * d as float / c as float).sin())
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

    fn plot_1d(
        path: &OsStr,
        observe_points: Vec<float>,
        acoustic_pressures: Vec<float>,
        resolution: float,
        x_label: &str,
        config: PlotConfig,
    ) -> Result<(), MonitorError> {
        if cfg!(target_os = "windows") {
            Self::initialize_python()?;
        }

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

def plot(observe, acoustic_pressures, resolution, x_label, path, config):
    plt.rcParams["font.size"] = config.fontsize
    fig = plt.figure(figsize=config.figsize, dpi=config.dpi)
    ax = fig.add_subplot(111)
    plot_acoustic_field_1d(ax, acoustic_pressures, observe, resolution, config)

    ax.set_xlabel(x_label)
    ax.set_ylabel("Amplitude [-]")

    plt.tight_layout()
    plt.savefig(path, dpi=fig.dpi, bbox_inches='tight')            
                "#,
                "",
                "",
            )?
            .getattr("plot")?;
            fun.call1((
                observe_points,
                acoustic_pressures,
                resolution,
                x_label,
                path,
                config,
            ))?;
            Ok(())
        })?;

        Ok(())
    }

    fn plot_2d(
        path: &OsStr,
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<float>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: PlotConfig,
    ) -> Result<(), MonitorError> {
        if cfg!(target_os = "windows") {
            Self::initialize_python()?;
        }

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

def plot(observe_x, observe_y, acoustic_pressures, resolution, x_label, y_label, path, config):
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
    plt.savefig(path, dpi=fig.dpi, bbox_inches='tight')            
                "#,
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
                path,
                config,
            ))?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn calc_field<'a, T: Transducer, I: Iterator<Item = Vector3>>(
        &self,
        observe_points: I,
        geometry: &Geometry<T>,
    ) -> Vec<float> {
        let sound_speed = geometry.sound_speed;
        observe_points
            .map(|target| {
                self.cpus
                    .iter()
                    .enumerate()
                    .fold(Complex::new(0., 0.), |acc, (i, cpu)| {
                        let (duty, phase) = cpu.fpga().drives(0);
                        acc + geometry
                            .transducers_of(i)
                            .zip(duty.iter())
                            .zip(phase.iter())
                            .fold(Complex::new(0., 0.), |acc, ((t, &d), &p)| {
                                let amp = (PI * d as f64 / t.cycle() as f64).sin();
                                let phase = 2. * PI * p as f64 / t.cycle() as f64;
                                acc + propagate::<autd3_core::acoustics::Sphere>(
                                    &t.position(),
                                    &t.z_direction(),
                                    0.0,
                                    t.wavenumber(sound_speed),
                                    &target,
                                ) * Complex::from_polar(amp, phase)
                            })
                    })
                    .norm()
            })
            .collect()
    }

    pub fn save_field<P: AsRef<Path>, T: Transducer>(
        &self,
        path: P,
        x_range: std::ops::Range<float>,
        y_range: std::ops::Range<float>,
        z_range: std::ops::Range<float>,
        resolution: float,
        config: PlotConfig,
        geometry: &Geometry<T>,
    ) -> Result<(), MonitorError> {
        let nx = ((x_range.end - x_range.start) / resolution).floor() as usize + 1;
        let ny = ((y_range.end - y_range.start) / resolution).floor() as usize + 1;
        let nz = ((z_range.end - z_range.start) / resolution).floor() as usize + 1;
        let path = path.as_ref().as_os_str();

        match (nx, ny, nz) {
            (nx, 1, 1) => {
                let observe = (0..nx)
                    .map(|i| x_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let acoustic_pressures = self.calc_field(
                    observe
                        .iter()
                        .map(|&x| Vector3::new(x, y_range.start, z_range.start)),
                    geometry,
                );
                Self::plot_1d(
                    path,
                    observe,
                    acoustic_pressures,
                    resolution,
                    "x [mm]",
                    config,
                )?;
            }
            (1, ny, 1) => {
                let observe = (0..ny)
                    .map(|i| y_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let acoustic_pressures = self.calc_field(
                    observe
                        .iter()
                        .map(|&y| Vector3::new(x_range.start, y, z_range.start)),
                    geometry,
                );
                Self::plot_1d(
                    path,
                    observe,
                    acoustic_pressures,
                    resolution,
                    "y [mm]",
                    config,
                )?;
            }
            (1, 1, nz) => {
                let observe = (0..nz)
                    .map(|i| z_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let acoustic_pressures = self.calc_field(
                    observe
                        .iter()
                        .map(|&z| Vector3::new(x_range.start, y_range.start, z)),
                    geometry,
                );
                Self::plot_1d(
                    path,
                    observe,
                    acoustic_pressures,
                    resolution,
                    "z [mm]",
                    config,
                )?;
            }
            (nx, ny, 1) => {
                let observe_x = (0..nx)
                    .map(|i| x_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let observe_y = (0..ny)
                    .map(|i| y_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let acoustic_pressures = self.calc_field(
                    itertools::iproduct!(&observe_y, &observe_x)
                        .map(|(&y, &x)| Vector3::new(x, y, z_range.start)),
                    geometry,
                );
                Self::plot_2d(
                    path,
                    observe_x,
                    observe_y,
                    acoustic_pressures,
                    resolution,
                    "x [mm]",
                    "y [mm]",
                    config,
                )?;
            }
            (nx, 1, nz) => {
                let observe_x = (0..nx)
                    .map(|i| x_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let observe_z = (0..nz)
                    .map(|i| z_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let acoustic_pressures = self.calc_field(
                    itertools::iproduct!(&observe_x, &observe_z)
                        .map(|(&x, &z)| Vector3::new(x, y_range.start, z)),
                    geometry,
                );
                Self::plot_2d(
                    path,
                    observe_z,
                    observe_x,
                    acoustic_pressures,
                    resolution,
                    "z [mm]",
                    "x [mm]",
                    config,
                )?;
            }
            (1, ny, nz) => {
                let observe_z = (0..nz)
                    .map(|i| z_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let observe_y = (0..ny)
                    .map(|i| y_range.start + resolution * i as float)
                    .collect::<Vec<_>>();
                let acoustic_pressures = self.calc_field(
                    itertools::iproduct!(&observe_z, &observe_y)
                        .map(|(&z, &y)| Vector3::new(x_range.start, y, z)),
                    geometry,
                );
                Self::plot_2d(
                    path,
                    observe_y,
                    observe_z,
                    acoustic_pressures,
                    resolution,
                    "y [mm]",
                    "z [mm]",
                    config,
                )?;
            }
            _ => return Err(MonitorError::InvalidPlotRange),
        }

        Ok(())
    }

    pub fn save_phase<P: AsRef<Path>, T: Transducer>(
        &self,
        path: P,
        config: PlotConfig,
        geometry: &Geometry<T>,
    ) -> Result<(), MonitorError> {
        if cfg!(target_os = "windows") {
            Self::initialize_python()?;
        }

        let path = path.as_ref().as_os_str();

        let trans_x = geometry
            .transducers()
            .map(|t| t.position().x)
            .collect::<Vec<_>>();
        let trans_y = geometry
            .transducers()
            .map(|t| t.position().y)
            .collect::<Vec<_>>();
        let trans_phase = self.phases();

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

def plot(trans_x, trans_y, trans_phase, path, config, trans_size):
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
    plt.savefig(path, dpi=fig.dpi, bbox_inches='tight')            
                "#,
                "",
                "",
            )?
            .getattr("plot")?;
            fun.call1((
                trans_x,
                trans_y,
                trans_phase,
                path,
                config,
                autd3_core::autd3_device::TRANS_SPACING,
            ))?;
            Ok(())
        })?;

        Ok(())
    }
}

impl<T: Transducer> Link<T> for Monitor {
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

        for cpu in &mut self.cpus {
            cpu.send(tx);
        }

        Ok(true)
    }

    fn receive(&mut self, rx: &mut RxDatagram) -> Result<bool, AUTDInternalError> {
        if !self.is_open {
            return Ok(false);
        }

        for cpu in &mut self.cpus {
            rx.messages_mut()[cpu.id()].ack = cpu.ack();
            rx.messages_mut()[cpu.id()].msg_id = cpu.msg_id();
        }

        Ok(true)
    }

    fn is_open(&self) -> bool {
        self.is_open
    }

    fn timeout(&self) -> Duration {
        self.timeout
    }
}
