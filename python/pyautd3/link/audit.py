"""
File: debug copy.py
Project: link
Created Date: 18/09/2023
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import ctypes
from datetime import timedelta

import numpy as np

from pyautd3.internal.link import Link, LinkBuilder
from pyautd3.native_methods.autd3capi import LinkAuditBuilderPtr
from pyautd3.native_methods.autd3capi import NativeMethods as LinkAudit
from pyautd3.native_methods.autd3capi_def import ControllerPtr, LinkBuilderPtr, LinkPtr

__all__ = []  # type: ignore[var-annotated]


class Audit(Link):
    _ptr: LinkPtr

    class _Builder(LinkBuilder):
        _builder: LinkAuditBuilderPtr

        def __init__(self: "Audit._Builder") -> None:
            self._builder = LinkAudit().link_audit()

        def with_timeout(self: "Audit._Builder", timeout: timedelta) -> "Audit._Builder":
            self._builder = LinkAudit().link_audit_with_timeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
            return self

        def _link_builder_ptr(self: "Audit._Builder") -> LinkBuilderPtr:
            return LinkAudit().link_audit_into_builder(self._builder)

        def _resolve_link(self: "Audit._Builder", ptr: ControllerPtr) -> "Audit":
            return Audit(LinkAudit().link_get(ptr))

    def __init__(self: "Audit", ptr: LinkPtr) -> None:
        super().__init__(ptr)

    @staticmethod
    def builder() -> _Builder:
        return Audit._Builder()

    def down(self: "Audit") -> None:
        LinkAudit().link_audit_down(self._ptr)

    def is_open(self: "Audit") -> bool:
        return bool(LinkAudit().link_audit_is_open(self._ptr))

    def is_force_fan(self: "Audit", idx: int) -> bool:
        return bool(LinkAudit().link_audit_fpga_is_force_fan(self._ptr, idx))

    def last_timeout_ns(self: "Audit") -> int:
        return int(LinkAudit().link_audit_last_timeout_ns(self._ptr))

    def up(self: "Audit") -> None:
        LinkAudit().link_audit_up(self._ptr)

    def break_down(self: "Audit") -> None:
        LinkAudit().link_audit_break_down(self._ptr)

    def update(self: "Audit", idx: int) -> None:
        LinkAudit().link_audit_cpu_update(self._ptr, idx)

    def silencer_step_intensity(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_silencer_step_intensity(self._ptr, idx))

    def silencer_step_phase(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_silencer_step_phase(self._ptr, idx))

    def debug_output_idx(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_debug_output_idx(self._ptr, idx))

    def assert_thermal_sensor(self: "Audit", idx: int) -> None:
        LinkAudit().link_audit_fpga_assert_thermal_sensor(self._ptr, idx)

    def deassert_thermal_sensor(self: "Audit", idx: int) -> None:
        LinkAudit().link_audit_fpga_deassert_thermal_sensor(self._ptr, idx)

    def modulation(self: "Audit", idx: int) -> np.ndarray:
        n = int(LinkAudit().link_audit_fpga_modulation_cycle(self._ptr, idx))
        buf = np.zeros([n]).astype(ctypes.c_uint8)
        LinkAudit().link_audit_fpga_modulation(self._ptr, idx, np.ctypeslib.as_ctypes(buf))
        return buf

    def modulation_frequency_division(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_modulation_frequency_division(self._ptr, idx))

    def mod_delays(self: "Audit", idx: int) -> np.ndarray:
        n = int(LinkAudit().link_audit_cpu_num_transducers(self._ptr, idx))
        buf = np.zeros([n]).astype(ctypes.c_uint16)
        LinkAudit().link_audit_fpga_mod_delays(self._ptr, idx, np.ctypeslib.as_ctypes(buf))
        return buf

    def intensities_and_phases(self: "Audit", idx: int, stm_idx: int) -> tuple[np.ndarray, np.ndarray]:
        n = int(LinkAudit().link_audit_cpu_num_transducers(self._ptr, idx))
        intensities = np.zeros([n]).astype(ctypes.c_uint8)
        phases = np.zeros([n]).astype(ctypes.c_uint8)
        LinkAudit().link_audit_fpga_intensities_and_phases(
            self._ptr,
            idx,
            stm_idx,
            np.ctypeslib.as_ctypes(intensities),
            np.ctypeslib.as_ctypes(phases),
        )
        return (intensities, phases)

    def stm_cycle(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_cycle(self._ptr, idx))

    def is_stm_gain_mode(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_is_stm_gain_mode(self._ptr, idx))

    def stm_freqency_division(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_frequency_division(self._ptr, idx))

    def stm_start_idx(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_start_idx(self._ptr, idx))

    def stm_finish_idx(self: "Audit", idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_finish_idx(self._ptr, idx))
