"""
File: visualizer.py
Project: link
Created Date: 13/10/2023
Author: Shun Suzuki
-----
Last Modified: 13/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import ctypes
from abc import ABCMeta, abstractmethod
from collections.abc import Iterable

import numpy as np

from pyautd3.autd_error import AUTDError, InvalidPlotConfigError
from pyautd3.geometry import Geometry
from pyautd3.internal.link import Link, LinkBuilder
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import AUTD3_ERR, ControllerPtr, LinkPtr
from pyautd3.native_methods.autd3capi_link_visualizer import (
    Backend,
    CMap,
    ConfigPtr,
    Directivity,
    LinkBuilderPtr,
)
from pyautd3.native_methods.autd3capi_link_visualizer import (
    NativeMethods as LinkVisualizer,
)

__all__ = [
    "PlottersBackend",
    "PythonBackend",
    "NullBackend",
    "Sphere",
    "T4010A1",
    "PlotRange",
    "PlotConfig",
    "PyPlotConfig",
    "NullPlotConfig",
    "Visualizer",
]  # type: ignore[var-annotated]


class IPlotBackend(metaclass=ABCMeta):
    _backend: Backend

    def __init__(self: "IPlotBackend", backend: Backend) -> None:
        self._backend = backend


class PlottersBackend(IPlotBackend):
    """Plotters backend."""

    def __init__(self: "PlottersBackend") -> None:
        super().__init__(Backend.Plotters)


class PythonBackend(IPlotBackend):
    """Python backend."""

    def __init__(self: "PythonBackend") -> None:
        super().__init__(Backend.Python)


class NullBackend(IPlotBackend):
    """Null backend."""

    def __init__(self: "NullBackend") -> None:
        super().__init__(Backend.Null)


class IDirectivity(metaclass=ABCMeta):
    _directivity: Directivity

    def __init__(self: "IDirectivity", directivity: Directivity) -> None:
        self._directivity = directivity


class Sphere(IDirectivity):
    """Sphere directivity."""

    def __init__(self: "Sphere") -> None:
        super().__init__(Directivity.Sphere)


class T4010A1(IDirectivity):
    """T4010A1 directivity."""

    def __init__(self: "T4010A1") -> None:
        super().__init__(Directivity.T4010A1)


class PlotRange:
    """Plot range."""

    x_start: float
    x_end: float
    y_start: float
    y_end: float
    z_start: float
    z_end: float
    resolution: float

    def __init__(
        self: "PlotRange",
        *,
        x_start: float = -0,
        x_end: float = 0,
        y_start: float = -0,
        y_end: float = 0,
        z_start: float = -0,
        z_end: float = 0,
        resolution: float = 1,
    ) -> None:
        self.x_start = x_start
        self.x_end = x_end
        self.y_start = y_start
        self.y_end = y_end
        self.z_start = z_start
        self.z_end = z_end
        self.resolution = resolution


class IPlotConfig(metaclass=ABCMeta):
    @abstractmethod
    def _config_ptr(self: "IPlotConfig") -> ConfigPtr:
        pass

    @abstractmethod
    def _backend(self: "IPlotConfig") -> Backend:
        pass


class PlotConfig(IPlotConfig):
    """Plot config for PlottersBackend."""

    figsize: tuple[int, int] | None
    cbar_size: float | None
    font_size: int | None
    label_area_size: int | None
    margin: int | None
    ticks_step: float | None
    cmap: CMap | None
    fname: str | None

    def __init__(
        self: "PlotConfig",
        *,
        figsize: tuple[int, int] | None = None,
        cbar_size: float | None = None,
        font_size: int | None = None,
        label_area_size: int | None = None,
        margin: int | None = None,
        ticks_step: float | None = None,
        cmap: CMap | None = None,
        fname: str | None = None,
    ) -> None:
        self.figsize = figsize
        self.cbar_size = cbar_size
        self.font_size = font_size
        self.label_area_size = label_area_size
        self.margin = margin
        self.ticks_step = ticks_step
        self.cmap = cmap
        self.fname = fname

    def _config_ptr(self: "PlotConfig") -> ConfigPtr:
        err = ctypes.create_string_buffer(256)
        ptr = LinkVisualizer().link_visualizer_plot_config_default()
        if self.figsize is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_fig_size(ptr, self.figsize[0], self.figsize[1])
        if self.cbar_size is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_c_bar_size(ptr, self.cbar_size)
        if self.font_size is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_font_size(ptr, self.font_size)
        if self.label_area_size is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_label_area_size(ptr, self.label_area_size)
        if self.margin is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_margin(ptr, self.margin)
        if self.ticks_step is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_ticks_step(ptr, self.ticks_step)
        if self.cmap is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_c_map(ptr, self.cmap)
        if self.fname is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_f_name(ptr, self.fname.encode("utf-8"), err)
        if ptr._0 is None:
            raise AUTDError(err)
        return ConfigPtr(ptr._0)

    def _backend(self: "PlotConfig") -> Backend:
        return Backend.Plotters


class PyPlotConfig(IPlotConfig):
    """Plot config for PythonBackend."""

    figsize: tuple[int, int] | None
    dpi: int | None
    cbar_position: str | None
    cbar_size: str | None
    cbar_pad: str | None
    fontsize: int | None
    ticks_step: float | None
    cmap: str | None
    show: bool | None
    fname: str | None

    def __init__(
        self: "PyPlotConfig",
        *,
        figsize: tuple[int, int] | None = None,
        dpi: int | None = None,
        cbar_position: str | None = None,
        cbar_size: str | None = None,
        cbar_pad: str | None = None,
        fontsize: int | None = None,
        ticks_step: float | None = None,
        cmap: str | None = None,
        show: bool | None = None,
        fname: str | None = None,
    ) -> None:
        self.figsize = figsize
        self.dpi = dpi
        self.cbar_position = cbar_position
        self.cbar_size = cbar_size
        self.cbar_pad = cbar_pad
        self.fontsize = fontsize
        self.ticks_step = ticks_step
        self.cmap = cmap
        self.show = show
        self.fname = fname

    def _config_ptr(self: "PyPlotConfig") -> ConfigPtr:
        err = ctypes.create_string_buffer(256)
        ptr = LinkVisualizer().link_visualizer_py_plot_config_default()
        if self.figsize is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_fig_size(ptr, self.figsize[0], self.figsize[1])
        if self.dpi is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_dpi(ptr, self.dpi)
        if self.cbar_position is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_c_bar_position(ptr, self.cbar_position.encode("utf-8"), err)
        if self.cbar_size is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_c_bar_size(ptr, self.cbar_size.encode("utf-8"), err)
        if self.cbar_pad is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_c_bar_pad(ptr, self.cbar_pad.encode("utf-8"), err)
        if self.fontsize is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_font_size(ptr, self.fontsize)
        if self.ticks_step is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_ticks_step(ptr, self.ticks_step)
        if self.cmap is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_c_map(ptr, self.cmap.encode("utf-8"), err)
        if self.show is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_show(ptr, self.show)
        if self.fname is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_f_name(ptr, self.fname.encode("utf-8"), err)
        if ptr._0 is None:
            raise AUTDError(err)
        return ConfigPtr(ptr._0)

    def _backend(self: "PyPlotConfig") -> Backend:
        return Backend.Python


class NullPlotConfig(IPlotConfig):
    """Plot config for NullBackend."""

    def _config_ptr(self: "NullPlotConfig") -> ConfigPtr:
        return ConfigPtr(LinkVisualizer().link_visualizer_null_plot_config_default()._0)

    def _backend(self: "NullPlotConfig") -> Backend:
        return Backend.Null


class Visualizer(Link):
    """Link for visualizing."""

    _ptr: LinkPtr
    _backend: Backend
    _directivity: Directivity

    class _Builder(LinkBuilder["Visualizer"]):
        _backend: Backend
        _directivity: Directivity
        _gpu_idx: int | None

        def __init__(self: "Visualizer._Builder", backend: Backend | None = None, directivity: Directivity | None = None) -> None:
            self._backend = backend if backend is not None else Backend.Plotters
            self._directivity = directivity if directivity is not None else Directivity.Sphere
            self._gpu_idx = None

        def _link_builder_ptr(self: "Visualizer._Builder") -> LinkBuilderPtr:
            match self._backend:
                case Backend.Plotters:
                    match self._directivity:
                        case Directivity.Sphere:
                            return LinkVisualizer().link_visualizer_sphere_plotters(
                                self._gpu_idx is not None,
                                self._gpu_idx if self._gpu_idx is not None else 0,
                            )
                        case Directivity.T4010A1:
                            return LinkVisualizer().link_visualizer_t_4010_a_1_plotters(
                                self._gpu_idx is not None,
                                self._gpu_idx if self._gpu_idx is not None else 0,
                            )
                case Backend.Python:
                    match self._directivity:
                        case Directivity.Sphere:
                            return LinkVisualizer().link_visualizer_sphere_python(
                                self._gpu_idx is not None,
                                self._gpu_idx if self._gpu_idx is not None else 0,
                            )
                        case Directivity.T4010A1:
                            return LinkVisualizer().link_visualizer_t_4010_a_1_python(
                                self._gpu_idx is not None,
                                self._gpu_idx if self._gpu_idx is not None else 0,
                            )
                case Backend.Null:
                    match self._directivity:
                        case Directivity.Sphere:
                            return LinkVisualizer().link_visualizer_sphere_null(
                                self._gpu_idx is not None,
                                self._gpu_idx if self._gpu_idx is not None else 0,
                            )
                        case Directivity.T4010A1:
                            return LinkVisualizer().link_visualizer_t_4010_a_1_null(
                                self._gpu_idx is not None,
                                self._gpu_idx if self._gpu_idx is not None else 0,
                            )

        def _resolve_link(self: "Visualizer._Builder", ptr: ControllerPtr) -> "Visualizer":
            return Visualizer(Base().link_get(ptr), self._backend, self._directivity)

        def with_gpu(self: "Visualizer._Builder", gpu_idx: int) -> "Visualizer._Builder":
            """Set GPU index.

            Arguments:
            ---------
                gpu_idx: GPU index
            """
            self._gpu_idx = gpu_idx
            return self

        def with_backend(self: "Visualizer._Builder", backend: IPlotBackend) -> "Visualizer._Builder":
            """Set backend.

            Arguments:
            ---------
                backend: Backend
            """
            self._backend = backend._backend
            return self

        def with_directivity(self: "Visualizer._Builder", directivity: IDirectivity) -> "Visualizer._Builder":
            """Set directivity.

            Arguments:
            ---------
                directivity: Directivity
            """
            self._directivity = directivity._directivity
            return self

    def __init__(self: "Visualizer", ptr: LinkPtr, backend: Backend, directivity: Directivity) -> None:
        super().__init__(ptr)
        self._backend = backend
        self._directivity = directivity

    @staticmethod
    def builder() -> _Builder:
        """Create visualizer link builder."""
        return Visualizer._Builder()

    @staticmethod
    def plotters() -> _Builder:
        """Create visualizer link builder with PlottersBackend."""
        return Visualizer._Builder(Backend.Plotters)

    @staticmethod
    def python() -> _Builder:
        """Create visualizer link builder with PythonBackend."""
        return Visualizer._Builder(Backend.Python)

    @staticmethod
    def null() -> _Builder:
        """Create visualizer link builder with NullBackend."""
        return Visualizer._Builder(Backend.Null)

    def phases_of(self: "Visualizer", idx: int) -> np.ndarray:
        """Get phases of specifig STM index."""
        size = LinkVisualizer().link_visualizer_phases_of(self._ptr, self._backend, self._directivity, idx, None)
        phases = np.zeros(int(size)).astype(ctypes.c_double)
        LinkVisualizer().link_visualizer_phases_of(self._ptr, self._backend, self._directivity, idx, np.ctypeslib.as_ctypes(phases))
        return phases

    def phases(self: "Visualizer") -> np.ndarray:
        """Get phases."""
        return self.phases_of(0)

    def duties_of(self: "Visualizer", idx: int) -> np.ndarray:
        """Get duties of specifig STM index."""
        size = LinkVisualizer().link_visualizer_duties_of(self._ptr, self._backend, self._directivity, idx, None)
        phases = np.zeros(int(size)).astype(ctypes.c_double)
        LinkVisualizer().link_visualizer_duties_of(self._ptr, self._backend, self._directivity, idx, np.ctypeslib.as_ctypes(phases))
        return phases

    def duties(self: "Visualizer") -> np.ndarray:
        """Get duties."""
        return self.duties_of(0)

    def modulation_raw(self: "Visualizer") -> np.ndarray:
        """Get raw modulation data."""
        size = LinkVisualizer().link_visualizer_modulation_raw(self._ptr, self._backend, self._directivity, None)
        modulation = np.zeros(int(size)).astype(ctypes.c_double)
        LinkVisualizer().link_visualizer_modulation_raw(self._ptr, self._backend, self._directivity, np.ctypeslib.as_ctypes(modulation))
        return modulation

    def modulation(self: "Visualizer") -> np.ndarray:
        """Get modulation data."""
        size = LinkVisualizer().link_visualizer_modulation(self._ptr, self._backend, self._directivity, None)
        modulation = np.zeros(int(size)).astype(ctypes.c_double)
        LinkVisualizer().link_visualizer_modulation(self._ptr, self._backend, self._directivity, np.ctypeslib.as_ctypes(modulation))
        return modulation

    def calc_field_of(self: "Visualizer", points_iter: Iterable[np.ndarray], geometry: Geometry, idx: int) -> np.ndarray:
        """Calculate field of specific STM index."""
        points = np.fromiter(points_iter, dtype=np.ndarray)
        points_len = len(points)
        points = np.ravel(np.stack(points))  # type: ignore[call-overload]
        buf = np.zeros(points_len * 2).astype(ctypes.c_double)
        err = ctypes.create_string_buffer(256)
        if (
            LinkVisualizer().link_visualizer_calc_field_of(
                self._ptr,
                self._backend,
                self._directivity,
                np.ctypeslib.as_ctypes(points),
                points_len,
                geometry._geometry_ptr(),
                idx,
                np.ctypeslib.as_ctypes(buf),
                err,
            )
            == AUTD3_ERR
        ):
            raise AUTDError(err)
        return np.fromiter([buf[2 * i] + buf[2 * i + 1] * 1j for i in range(points_len)], dtype=np.complex128, count=points_len)

    def calc_field(self: "Visualizer", points_iter: Iterable[np.ndarray], geometry: Geometry) -> np.ndarray:
        """Calculate field."""
        return self.calc_field_of(points_iter, geometry, 0)

    def plot_field_of(self: "Visualizer", config: IPlotConfig, plot_range: PlotRange, geometry: Geometry, idx: int) -> None:
        """Plot field of specific STM index."""
        if self._backend != config._backend():
            raise InvalidPlotConfigError
        err = ctypes.create_string_buffer(256)
        if (
            LinkVisualizer().link_visualizer_plot_field_of(
                self._ptr,
                self._backend,
                self._directivity,
                config._config_ptr(),
                LinkVisualizer().link_visualizer_plot_range(
                    plot_range.x_start,
                    plot_range.x_end,
                    plot_range.y_start,
                    plot_range.y_end,
                    plot_range.z_start,
                    plot_range.z_end,
                    plot_range.resolution,
                ),
                geometry._geometry_ptr(),
                idx,
                err,
            )
            == AUTD3_ERR
        ):
            raise AUTDError(err)

    def plot_field(self: "Visualizer", config: IPlotConfig, plot_range: PlotRange, geometry: Geometry) -> None:
        """Plot field."""
        self.plot_field_of(config, plot_range, geometry, 0)

    def plot_phase_of(self: "Visualizer", config: IPlotConfig, geometry: Geometry, idx: int) -> None:
        """Plot phase of specific STM index."""
        if self._backend != config._backend():
            raise InvalidPlotConfigError
        err = ctypes.create_string_buffer(256)
        if (
            LinkVisualizer().link_visualizer_plot_phase_of(
                self._ptr,
                self._backend,
                self._directivity,
                config._config_ptr(),
                geometry._geometry_ptr(),
                idx,
                err,
            )
            == AUTD3_ERR
        ):
            raise AUTDError(err)

    def plot_phase(self: "Visualizer", config: IPlotConfig, geometry: Geometry) -> None:
        """Plot phase."""
        self.plot_phase_of(config, geometry, 0)

    def plot_modulation_raw(self: "Visualizer", config: IPlotConfig) -> None:
        """Plot raw modulation."""
        if self._backend != config._backend():
            raise InvalidPlotConfigError
        err = ctypes.create_string_buffer(256)
        if LinkVisualizer().link_visualizer_plot_modulation_raw(self._ptr, self._backend, self._directivity, config._config_ptr(), err) == AUTD3_ERR:
            raise AUTDError(err)

    def plot_modulation(self: "Visualizer", config: IPlotConfig) -> None:
        """Plot modulation."""
        if self._backend != config._backend():
            raise InvalidPlotConfigError
        err = ctypes.create_string_buffer(256)
        if LinkVisualizer().link_visualizer_plot_modulation(self._ptr, self._backend, self._directivity, config._config_ptr(), err) == AUTD3_ERR:
            raise AUTDError(err)
