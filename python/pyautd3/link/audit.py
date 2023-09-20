'''
File: debug copy.py
Project: link
Created Date: 18/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
from typing import Tuple

import numpy as np

from .link import Link

from pyautd3.native_methods.autd3capi import NativeMethods as LinkAudit
from pyautd3.native_methods.autd3capi_def import ControllerPtr


class Audit(Link):
    def __init__(self):
        super().__init__(LinkAudit().link_audit())

    def with_timeout(self, timeout: timedelta) -> "Audit":
        self._ptr = LinkAudit().link_audit_with_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    @staticmethod
    def down(cnt_ptr: ControllerPtr):
        return LinkAudit().link_audit_down(LinkAudit().get_link(cnt_ptr))

    @staticmethod
    def is_open(cnt_ptr: ControllerPtr) -> bool:
        return LinkAudit().link_audit_is_open(LinkAudit().get_link(cnt_ptr))

    @staticmethod
    def last_timeout_ns(cnt_ptr: ControllerPtr) -> int:
        return LinkAudit().link_audit_last_timeout_ns(LinkAudit().get_link(cnt_ptr))

    @staticmethod
    def up(cnt_ptr: ControllerPtr):
        return LinkAudit().link_audit_up(LinkAudit().get_link(cnt_ptr))

    @staticmethod
    def break_down(cnt_ptr: ControllerPtr):
        return LinkAudit().link_audit_break_down(LinkAudit().get_link(cnt_ptr))

    @staticmethod
    def update(cnt_ptr: ControllerPtr, idx: int) -> int:
        return LinkAudit().link_audit_cpu_update(LinkAudit().get_link(cnt_ptr), idx)

    @staticmethod
    def fpga_flags(cnt_ptr: ControllerPtr, idx: int) -> int:
        return LinkAudit().link_audit_cpu_fpga_flags(LinkAudit().get_link(cnt_ptr), idx)

    @staticmethod
    def is_legacy(cnt_ptr: ControllerPtr, idx: int) -> bool:
        return LinkAudit().link_audit_fpga_is_legacy_mode(LinkAudit().get_link(cnt_ptr), idx)

    @staticmethod
    def silencer_step(cnt_ptr: ControllerPtr, idx: int) -> int:
        return LinkAudit().link_audit_fpga_silencer_step(LinkAudit().get_link(cnt_ptr), idx)

    @staticmethod
    def assert_thermal_sensor(cnt_ptr: ControllerPtr, idx: int) -> int:
        return LinkAudit().link_audit_fpga_assert_thermal_sensor(LinkAudit().get_link(cnt_ptr), idx)

    @staticmethod
    def deassert_thermal_sensor(cnt_ptr: ControllerPtr, idx: int) -> int:
        return LinkAudit().link_audit_fpga_deassert_thermal_sensor(LinkAudit().get_link(cnt_ptr), idx)

    @staticmethod
    def modulation(cnt_ptr: ControllerPtr, idx: int) -> np.ndarray:
        n = LinkAudit().link_audit_fpga_modulation_cycle(LinkAudit().get_link(cnt_ptr), idx)
        buf = np.zeros([n]).astype(np.uint8)
        LinkAudit().link_audit_fpga_modulation(LinkAudit().get_link(cnt_ptr), idx, np.ctypeslib.as_ctypes(buf))
        return buf

    @staticmethod
    def modulation_frequency_division(cnt_ptr: ControllerPtr, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_modulation_frequency_division(LinkAudit().get_link(cnt_ptr), idx))

    @staticmethod
    def cycles(cnt_ptr: ControllerPtr, idx: int) -> np.ndarray:
        n = LinkAudit().link_audit_cpu_num_transducers(LinkAudit().get_link(cnt_ptr), idx)
        buf = np.zeros([n]).astype(np.uint16)
        LinkAudit().link_audit_fpga_cycles(LinkAudit().get_link(cnt_ptr), idx, np.ctypeslib.as_ctypes(buf))
        return buf

    @staticmethod
    def mod_delays(cnt_ptr: ControllerPtr, idx: int) -> np.ndarray:
        n = LinkAudit().link_audit_cpu_num_transducers(LinkAudit().get_link(cnt_ptr), idx)
        buf = np.zeros([n]).astype(np.uint16)
        LinkAudit().link_audit_fpga_mod_delays(LinkAudit().get_link(cnt_ptr), idx, np.ctypeslib.as_ctypes(buf))
        return buf

    @staticmethod
    def duty_filters(cnt_ptr: ControllerPtr, idx: int) -> np.ndarray:
        n = LinkAudit().link_audit_cpu_num_transducers(LinkAudit().get_link(cnt_ptr), idx)
        buf = np.zeros([n]).astype(np.int16)
        LinkAudit().link_audit_fpga_duty_filters(LinkAudit().get_link(cnt_ptr), idx, np.ctypeslib.as_ctypes(buf))
        return buf

    @staticmethod
    def phase_filters(cnt_ptr: ControllerPtr, idx: int) -> np.ndarray:
        n = LinkAudit().link_audit_cpu_num_transducers(LinkAudit().get_link(cnt_ptr), idx)
        buf = np.zeros([n]).astype(np.int16)
        LinkAudit().link_audit_fpga_phase_filters(LinkAudit().get_link(cnt_ptr), idx, np.ctypeslib.as_ctypes(buf))
        return buf

    @staticmethod
    def duties_and_phases(cnt_ptr: ControllerPtr, idx: int, stm_idx: int) -> Tuple[np.ndarray, np.ndarray]:
        n = LinkAudit().link_audit_cpu_num_transducers(LinkAudit().get_link(cnt_ptr), idx)
        duties = np.zeros([n]).astype(np.uint16)
        phases = np.zeros([n]).astype(np.uint16)
        LinkAudit().link_audit_fpga_duties_and_phases(LinkAudit().get_link(cnt_ptr),
                                                      idx, stm_idx, np.ctypeslib.as_ctypes(duties), np.ctypeslib.as_ctypes(phases))
        return (duties, phases)

    @staticmethod
    def stm_cycle(cnt_ptr: ControllerPtr, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_cycle(LinkAudit().get_link(cnt_ptr), idx))

    @staticmethod
    def is_stm_gain_mode(cnt_ptr: ControllerPtr, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_is_stm_gain_mode(LinkAudit().get_link(cnt_ptr), idx))

    @staticmethod
    def stm_freqency_division(cnt_ptr: ControllerPtr, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_frequency_division(LinkAudit().get_link(cnt_ptr), idx))

    @staticmethod
    def stm_start_idx(cnt_ptr: ControllerPtr, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_start_idx(LinkAudit().get_link(cnt_ptr), idx))

    @staticmethod
    def stm_finish_idx(cnt_ptr: ControllerPtr, idx: int) -> int:
        return int(LinkAudit().link_audit_fpga_stm_finish_idx(LinkAudit().get_link(cnt_ptr), idx))
