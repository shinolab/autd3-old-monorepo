'''
File: debug copy.py
Project: link
Created Date: 18/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
from typing import Tuple

import numpy as np


from pyautd3.internal.link import LinkBuilder
from pyautd3.native_methods.autd3capi import NativeMethods as LinkAudit
from pyautd3.native_methods.autd3capi import LinkAuditBuilderPtr
from pyautd3.native_methods.autd3capi_def import LinkBuilderPtr, LinkPtr


class Audit:
    _ptr: LinkPtr

    class _Builder(LinkBuilder):
        _ptr: LinkAuditBuilderPtr

        def __init__(self):
            self._ptr = LinkAudit().link_audit()

        def with_timeout(self, timeout: timedelta) -> "Audit._Builder":
            self._ptr = LinkAudit().link_audit_with_timeout(
                self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
            )
            return self

        def ptr(self) -> LinkBuilderPtr:
            return LinkAudit().link_audit_into_builder(self._ptr)

        def resolve_link(self, obj):
            obj.link = lambda: Audit(LinkAudit().link_get(obj._ptr))

    def __init__(self, ptr: LinkPtr):
        self._ptr = ptr

    @staticmethod
    def builder() -> _Builder:
        return Audit._Builder()

    def down(self):
        return LinkAudit().link_audit_down(self._ptr)

    def is_open(self) -> bool:
        return bool(LinkAudit().link_audit_is_open(self._ptr))

    def last_timeout_ns(self) -> int:
        return int(LinkAudit().link_audit_last_timeout_ns(self._ptr))

    def up(self):
        return LinkAudit().link_audit_up(self._ptr)

    def break_down(self):
        return LinkAudit().link_audit_break_down(self._ptr)

    def update(self, idx: int):
        LinkAudit().link_audit_cpu_update(self._ptr, idx)

    def fpga_flags(self, idx: int) -> int:
        return int(LinkAudit().link_audit_cpu_fpga_flags(self._ptr, idx))

    def is_legacy(self, idx: int) -> bool:
        return bool(LinkAudit().link_audit_fpga_is_legacy_mode(self._ptr, idx))

    def silencer_step(self, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_silencer_step(self._ptr, idx))

    def assert_thermal_sensor(self, idx: int):
        LinkAudit().link_audit_fpga_assert_thermal_sensor(self._ptr, idx)

    def deassert_thermal_sensor(self, idx: int):
        LinkAudit().link_audit_fpga_deassert_thermal_sensor(self._ptr, idx)

    def modulation(self, idx: int) -> np.ndarray:
        n = int(LinkAudit().link_audit_fpga_modulation_cycle(self._ptr, idx))
        buf = np.zeros([n]).astype(np.uint8)
        LinkAudit().link_audit_fpga_modulation(self._ptr, idx, np.ctypeslib.as_ctypes(buf))  # type: ignore
        return buf

    def modulation_frequency_division(self, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_modulation_frequency_division(self._ptr, idx))

    def cycles(self, idx: int) -> np.ndarray:
        n = int(LinkAudit().link_audit_cpu_num_transducers(self._ptr, idx))
        buf = np.zeros([n]).astype(np.uint16)
        LinkAudit().link_audit_fpga_cycles(self._ptr, idx, np.ctypeslib.as_ctypes(buf))  # type: ignore
        return buf

    def mod_delays(self, idx: int) -> np.ndarray:
        n = int(LinkAudit().link_audit_cpu_num_transducers(self._ptr, idx))
        buf = np.zeros([n]).astype(np.uint16)
        LinkAudit().link_audit_fpga_mod_delays(self._ptr, idx, np.ctypeslib.as_ctypes(buf))  # type: ignore
        return buf

    def duty_filters(self, idx: int) -> np.ndarray:
        n = int(LinkAudit().link_audit_cpu_num_transducers(self._ptr, idx))
        buf = np.zeros([n]).astype(np.int16)
        LinkAudit().link_audit_fpga_duty_filters(self._ptr, idx, np.ctypeslib.as_ctypes(buf))  # type: ignore
        return buf

    def phase_filters(self, idx: int) -> np.ndarray:
        n = int(LinkAudit().link_audit_cpu_num_transducers(self._ptr, idx))
        buf = np.zeros([n]).astype(np.int16)
        LinkAudit().link_audit_fpga_phase_filters(self._ptr, idx, np.ctypeslib.as_ctypes(buf))  # type: ignore
        return buf

    def duties_and_phases(self, idx: int, stm_idx: int) -> Tuple[np.ndarray, np.ndarray]:
        n = int(LinkAudit().link_audit_cpu_num_transducers(self._ptr, idx))
        duties = np.zeros([n]).astype(np.uint16)
        phases = np.zeros([n]).astype(np.uint16)
        LinkAudit().link_audit_fpga_duties_and_phases(self._ptr,
                                                      idx, stm_idx, np.ctypeslib.as_ctypes(duties), np.ctypeslib.as_ctypes(phases))  # type: ignore
        return (duties, phases)

    def stm_cycle(self, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_cycle(self._ptr, idx))

    def is_stm_gain_mode(self, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_is_stm_gain_mode(self._ptr, idx))

    def stm_freqency_division(self, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_frequency_division(self._ptr, idx))

    def stm_start_idx(self, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_start_idx(self._ptr, idx))

    def stm_finish_idx(self, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_finish_idx(self._ptr, idx))
