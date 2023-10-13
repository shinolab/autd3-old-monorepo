'''
File: visualizer.py
Project: link
Created Date: 13/10/2023
Author: Shun Suzuki
-----
Last Modified: 13/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

from abc import ABCMeta, abstractmethod
import ctypes
from typing import Iterable, Optional, Tuple

import numpy as np

from pyautd3.native_methods.autd3capi_link_visualizer import (
    Backend, CMap, ConfigPtr, Directivity, LinkBuilderPtr,
    NativeMethods as LinkVisualizer,
)
from pyautd3.native_methods.autd3capi import NativeMethods as Base

from pyautd3.autd_error import AUTDError
from pyautd3.internal.link import LinkBuilder
from pyautd3.native_methods.autd3capi_def import AUTD3_ERR, LinkPtr
from pyautd3.geometry import Geometry


class IPlotBackend(metaclass=ABCMeta):
    _backend: Backend

    def __init__(self, backend: Backend):
        self._backend = backend


class PlottersBackend(IPlotBackend):
    def __init__(self):
        super().__init__(Backend.Plotters)


class PythonBackend(IPlotBackend):
    def __init__(self):
        super().__init__(Backend.Python)


class NullBackend(IPlotBackend):
    def __init__(self):
        super().__init__(Backend.Null)


class IDirectivity(metaclass=ABCMeta):
    _directivity: Directivity

    def __init__(self, directivity: Directivity):
        self._directivity = directivity


class Sphere(IDirectivity):
    def __init__(self):
        super().__init__(Directivity.Sphere)


class T4010A1(IDirectivity):
    def __init__(self):
        super().__init__(Directivity.T4010A1)


class PlotRange:
    x_start: float
    x_end: float
    y_start: float
    y_end: float
    z_start: float
    z_end: float
    resolution: float

    def __init__(self, **kwargs):
        self.x_start = kwargs.get("x_start", 0)
        self.x_end = kwargs.get("x_end", 0)
        self.y_start = kwargs.get("y_start", -0)
        self.y_end = kwargs.get("y_end", 0)
        self.z_start = kwargs.get("z_start", -0)
        self.z_end = kwargs.get("z_end", 0)
        self.resolution = kwargs.get("resolution", 1)


class IPlotConfig(metaclass=ABCMeta):
    @abstractmethod
    def _ptr(self) -> ConfigPtr:
        pass

    @abstractmethod
    def _backend(self) -> Backend:
        pass


class PlotConfig(IPlotConfig):
    figsize: Optional[Tuple[int, int]]
    cbar_size: Optional[float]
    font_size: Optional[int]
    label_area_size: Optional[int]
    margin: Optional[int]
    ticks_step: Optional[float]
    cmap: Optional[CMap]
    fname: Optional[str]

    def __init__(self, **kwargs):
        self.figsize = kwargs.get("figsize", None)
        self.cbar_size = kwargs.get("cbar_size", None)
        self.font_size = kwargs.get("font_size", None)
        self.label_area_size = kwargs.get("label_area_size", None)
        self.margin = kwargs.get("margin", None)
        self.ticks_step = kwargs.get("ticks_step", None)
        self.cmap = kwargs.get("cmap", None)
        self.fname = kwargs.get("fname", None)

    def _ptr(self) -> ConfigPtr:
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
        err = ctypes.create_string_buffer(256)
        if self.fname is not None:
            ptr = LinkVisualizer().link_visualizer_plot_config_with_f_name(ptr, self.fname.encode("utf-8"), err)
        if ptr._0 is None:
            raise AUTDError(err)
        return ConfigPtr(ptr._0)

    def _backend(self):
        return Backend.Plotters


class PyPlotConfig(IPlotConfig):
    figsize: Optional[Tuple[int, int]]
    dpi: Optional[int]
    cbar_position: Optional[str]
    cbar_size: Optional[str]
    cbar_pad: Optional[str]
    fontsize: Optional[int]
    ticks_step: Optional[float]
    cmap: Optional[str]
    show: Optional[bool]
    fname: Optional[str]

    def __init__(self, **kwargs):
        self.figsize = kwargs.get("figsize", None)
        self.dpi = kwargs.get("dpi", None)
        self.cbar_position = kwargs.get("cbar_position", None)
        self.cbar_size = kwargs.get("cbar_size", None)
        self.cbar_pad = kwargs.get("cbar_pad", None)
        self.fontsize = kwargs.get("fontsize", None)
        self.ticks_step = kwargs.get("ticks_step", None)
        self.cmap = kwargs.get("cmap", None)
        self.show = kwargs.get("show", None)
        self.fname = kwargs.get("fname", None)

    def _ptr(self) -> ConfigPtr:
        err = ctypes.create_string_buffer(256)
        ptr = LinkVisualizer().link_visualizer_py_plot_config_default()
        if self.figsize is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_fig_size(ptr, self.figsize[0], self.figsize[1])
        if self.dpi is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_dpi(ptr, self.dpi)
        if self.cbar_position is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_c_bar_position(ptr, self.cbar_position.encode("utf-8"), err)
            if ptr._0 is None:
                raise AUTDError(err)
        if self.cbar_size is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_c_bar_size(ptr, self.cbar_size.encode("utf-8"), err)
            if ptr._0 is None:
                raise AUTDError(err)
        if self.cbar_pad is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_c_bar_pad(ptr, self.cbar_pad.encode("utf-8"), err)
            if ptr._0 is None:
                raise AUTDError(err)
        if self.fontsize is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_font_size(ptr, self.fontsize)
        if self.ticks_step is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_ticks_step(ptr, self.ticks_step)
        if self.cmap is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_c_map(ptr, self.cmap.encode("utf-8"), err)
            if ptr._0 is None:
                raise AUTDError(err)
        if self.show is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_show(ptr, self.show)
        if self.fname is not None:
            ptr = LinkVisualizer().link_visualizer_py_plot_config_with_f_name(ptr, self.fname.encode("utf-8"), err)
            if ptr._0 is None:
                raise AUTDError(err)
        return ConfigPtr(ptr._0)

    def _backend(self):
        return Backend.Python


class NullPlotConfig(IPlotConfig):
    def _ptr(self) -> ConfigPtr:
        return ConfigPtr(LinkVisualizer().link_visualizer_null_plot_config_default()._0)

    def _backend(self):
        return Backend.Null


class Visualizer:
    """Link for visualizing

    """

    _ptr: LinkPtr
    _backend: Backend
    _directivity: Directivity

    class _Builder(LinkBuilder):
        _backend: Backend
        _directivity: Directivity
        _gpu_idx: Optional[int]

        def __init__(self, backend: Optional[Backend] = None, directivity: Optional[Directivity] = None):
            self._backend = backend if backend is not None else Backend.Plotters
            self._directivity = directivity if directivity is not None else Directivity.Sphere
            self._gpu_idx = None

        def _ptr(self) -> LinkBuilderPtr:
            if self._backend == Backend.Plotters:
                if self._directivity == Directivity.Sphere:
                    return LinkVisualizer().link_visualizer_sphere_plotters(
                        self._gpu_idx is not None, self._gpu_idx if self._gpu_idx is not None else 0)
                elif self._directivity == Directivity.T4010A1:
                    return LinkVisualizer().link_visualizer_t_4010_a_1_plotters(
                        self._gpu_idx is not None, self._gpu_idx if self._gpu_idx is not None else 0)
            elif self._backend == Backend.Python:
                if self._directivity == Directivity.Sphere:
                    return LinkVisualizer().link_visualizer_sphere_python(
                        self._gpu_idx is not None, self._gpu_idx if self._gpu_idx is not None else 0)
                elif self._directivity == Directivity.T4010A1:
                    return LinkVisualizer().link_visualizer_t_4010_a_1_python(
                        self._gpu_idx is not None, self._gpu_idx if self._gpu_idx is not None else 0)
            else:
                if self._directivity == Directivity.Sphere:
                    return LinkVisualizer().link_visualizer_sphere_null(
                        self._gpu_idx is not None, self._gpu_idx if self._gpu_idx is not None else 0)
                elif self._directivity == Directivity.T4010A1:
                    return LinkVisualizer().link_visualizer_t_4010_a_1_null(
                        self._gpu_idx is not None, self._gpu_idx if self._gpu_idx is not None else 0)

        def _resolve_link(self, obj):
            obj.link = Visualizer(Base().link_get(obj._ptr), self._backend, self._directivity)

        def with_gpu(self, gpu_idx: int) -> "Visualizer._Builder":
            """Set GPU index

            Arguments:
            - `gpu_idx` - GPU index
            """

            self._gpu_idx = gpu_idx
            return self

        def with_backend(self, backend: IPlotBackend) -> "Visualizer._Builder":
            """Set backend

            Arguments:
            - `backend` - Backend
            """

            self._backend = backend._backend
            return self

        def with_directivity(self, directivity: IDirectivity) -> "Visualizer._Builder":
            """Set directivity

            Arguments:
            - `directivity` - Directivity
            """

            self._directivity = directivity._directivity
            return self

    def __init__(self, ptr: LinkPtr, backend: Backend, directivity: Directivity):
        self._ptr = ptr
        self._backend = backend
        self._directivity = directivity

    @staticmethod
    def builder() -> _Builder:
        return Visualizer._Builder()

    @staticmethod
    def plotters() -> _Builder:
        return Visualizer._Builder(Backend.Plotters)

    @staticmethod
    def python() -> _Builder:
        return Visualizer._Builder(Backend.Python)

    @staticmethod
    def null() -> _Builder:
        return Visualizer._Builder(Backend.Null)

    def phases_of(self, idx: int) -> np.ndarray:
        size = LinkVisualizer().link_visualizer_phases_of(self._ptr, self._backend, self._directivity, idx, None)  # type: ignore
        phases = np.zeros(int(size), dtype=np.float64)
        LinkVisualizer().link_visualizer_phases_of(
            self._ptr,
            self._backend,
            self._directivity,
            idx,
            np.ctypeslib.as_ctypes(phases))  # type: ignore
        return phases

    def phases(self) -> np.ndarray:
        return self.phases_of(0)

    def duties_of(self, idx: int) -> np.ndarray:
        size = LinkVisualizer().link_visualizer_duties_of(self._ptr, self._backend, self._directivity, idx, None)  # type: ignore
        phases = np.zeros(int(size), dtype=np.float64)
        LinkVisualizer().link_visualizer_duties_of(
            self._ptr,
            self._backend,
            self._directivity,
            idx,
            np.ctypeslib.as_ctypes(phases))  # type: ignore
        return phases

    def duties(self) -> np.ndarray:
        return self.duties_of(0)

    def modulation_raw(self):
        size = LinkVisualizer().link_visualizer_modulation_raw(self._ptr, self._backend, self._directivity, None)
        modulation = np.zeros(size, dtype=np.float64)
        LinkVisualizer().link_visualizer_modulation_raw(
            self._ptr,
            self._backend,
            self._directivity,
            np.ctypeslib.as_ctypes(modulation))
        return modulation

    def modulation(self):
        size = LinkVisualizer().link_visualizer_modulation(self._ptr, self._backend, self._directivity, None)
        modulation = np.zeros(size, dtype=np.float64)
        LinkVisualizer().link_visualizer_modulation(
            self._ptr,
            self._backend,
            self._directivity,
            np.ctypeslib.as_ctypes(modulation))
        return modulation

    def calc_field_of(self, points_iter: Iterable[np.ndarray], geometry: Geometry, idx: int) -> np.ndarray:
        points = np.fromiter(points_iter, dtype=np.ndarray)
        points_len = len(points)
        points = np.ravel(np.stack(points))  # type: ignore
        buf = np.zeros(points_len * 2, dtype=np.float64)
        LinkVisualizer().link_visualizer_calc_field_of(
            self._ptr,
            self._backend,
            self._directivity,
            np.ctypeslib.as_ctypes(points),
            points_len,
            geometry.ptr(),
            idx,
            np.ctypeslib.as_ctypes(buf))  # type: ignore
        return np.fromiter([buf[2 * i] + buf[2 * i + 1] * 1j for i in range(points_len)], dtype=np.complex128, count=points_len)

    def calc_field(self, points_iter: Iterable[np.ndarray], geometry: Geometry) -> np.ndarray:
        return self.calc_field_of(points_iter, geometry, 0)

    def plot_field_of(self, config: IPlotConfig, range: PlotRange, geometry: Geometry, idx: int):
        if self._backend != config._backend():
            raise AUTDError("Invalid plot config type.")
        err = ctypes.create_string_buffer(256)
        if LinkVisualizer().link_visualizer_plot_field_of(
            self._ptr,
            self._backend,
            self._directivity,
            config._ptr(),
            LinkVisualizer().link_visualizer_plot_range(
                range.x_start,
                range.x_end,
                range.y_start,
                range.y_end,
                range.z_start,
                range.z_end,
                range.resolution),
            geometry.ptr(),
            idx,
            err
        ) == AUTD3_ERR:
            raise AUTDError(err)

    def plot_field(self, config: IPlotConfig, range: PlotRange, geometry: Geometry):
        self.plot_field_of(config, range, geometry, 0)

    def plot_phase_of(self, config: IPlotConfig, geometry: Geometry, idx: int):
        if self._backend != config._backend():
            raise AUTDError("Invalid plot config type.")
        err = ctypes.create_string_buffer(256)
        if LinkVisualizer().link_visualizer_plot_phase_of(
            self._ptr,
            self._backend,
            self._directivity,
            config._ptr(),
            geometry.ptr(),
            idx,
            err
        ) == AUTD3_ERR:
            raise AUTDError(err)

    def plot_phase(self, config: IPlotConfig, geometry: Geometry):
        self.plot_phase_of(config, geometry, 0)

    def plot_modulation_raw(self, config: IPlotConfig):
        if self._backend != config._backend():
            raise AUTDError("Invalid plot config type.")
        err = ctypes.create_string_buffer(256)
        if LinkVisualizer().link_visualizer_plot_modulation_raw(
            self._ptr,
            self._backend,
            self._directivity,
            config._ptr(),
            err
        ) == AUTD3_ERR:
            raise AUTDError(err)

    def plot_modulation(self, config: IPlotConfig):
        if self._backend != config._backend():
            raise AUTDError("Invalid plot config type.")
        err = ctypes.create_string_buffer(256)
        if LinkVisualizer().link_visualizer_plot_modulation(
            self._ptr,
            self._backend,
            self._directivity,
            config._ptr(),
            err
        ) == AUTD3_ERR:
            raise AUTDError(err)
