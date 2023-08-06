/*
 * File: python.rs
 * Project: src
 * Created Date: 16/07/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 07/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::ffi::OsString;

use pyo3::prelude::*;

use crate::{error::MonitorError, Backend, Config};

use autd3_core::{
    acoustics::Complex,
    float,
    geometry::{Geometry, Transducer},
};

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyPlotConfig {
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

impl Default for PyPlotConfig {
    fn default() -> Self {
        Self {
            figsize: (8, 6),
            dpi: 72,
            cbar_position: "right".to_string(),
            cbar_size: "5%".to_string(),
            cbar_pad: "3%".to_string(),
            fontsize: 12,
            ticks_step: 10.,
            cmap: "jet".to_string(),
            show: false,
            fname: "fig.png".into(),
            interval: 100,
            print_progress: false,
        }
    }
}

impl Config for PyPlotConfig {
    fn print_progress(&self) -> bool {
        self.print_progress
    }
}

/// Backend using Python and matplotlib
pub struct PythonBackend {}

impl PythonBackend {
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

    #[cfg(not(target_os = "windows"))]
    fn initialize_python() -> PyResult<()> {
        Ok(())
    }
}

impl Backend for PythonBackend {
    type PlotConfig = PyPlotConfig;

    fn new() -> Self {
        Self {}
    }

    fn initialize(&mut self) -> Result<(), MonitorError> {
        Self::initialize_python()?;
        Ok(())
    }

    fn plot_1d(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<Complex>,
        resolution: float,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError> {
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
    fn plot_2d(
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<Complex>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError> {
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

    fn plot_modulation(
        modulation: Vec<float>,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError> {
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

    fn plot_phase<T: Transducer>(
        config: Self::PlotConfig,
        geometry: &Geometry<T>,
        phases: Vec<float>,
    ) -> Result<(), MonitorError> {
        Self::initialize_python()?;

        let trans_x = geometry
            .transducers()
            .map(|t| t.position().x)
            .collect::<Vec<_>>();
        let trans_y = geometry
            .transducers()
            .map(|t| t.position().y)
            .collect::<Vec<_>>();

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
                phases,
                config,
                autd3_core::autd3_device::AUTD3::TRANS_SPACING,
            ))?;
            Ok(())
        })?;

        Ok(())
    }

    fn animate_1d(
        observe_points: Vec<float>,
        acoustic_pressures: Vec<Vec<Complex>>,
        resolution: float,
        x_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError> {
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
    fn animate_2d(
        observe_x: Vec<float>,
        observe_y: Vec<float>,
        acoustic_pressures: Vec<Vec<Complex>>,
        resolution: float,
        x_label: &str,
        y_label: &str,
        config: Self::PlotConfig,
    ) -> Result<(), MonitorError> {
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
}
