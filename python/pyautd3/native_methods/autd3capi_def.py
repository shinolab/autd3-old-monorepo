# This file is autogenerated
import threading
import ctypes
import os
from enum import IntEnum

class GainSTMMode(IntEnum):
    PhaseDutyFull = 0
    PhaseFull = 1
    PhaseHalf = 2

    @classmethod
    def from_param(cls, obj):
        return int(obj)


class TransMode(IntEnum):
    Legacy = 0
    Advanced = 1
    AdvancedPhase = 2

    @classmethod
    def from_param(cls, obj):
        return int(obj)


class Level(IntEnum):
    Critical = 0
    Error = 1
    Warn = 2
    Info = 3
    Debug = 4
    Trace = 5
    Off = 6

    @classmethod
    def from_param(cls, obj):
        return int(obj)


class TimerStrategy(IntEnum):
    Sleep = 0
    BusyWait = 1
    NativeTimer = 2

    @classmethod
    def from_param(cls, obj):
        return int(obj)


class ControllerPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class GeometryPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class DevicePtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class TransducerPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class LinkPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class DatagramPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class DatagramSpecialPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class GainPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class ModulationPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class STMPropsPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class BackendPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class ConstraintPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class GainCalcDrivesMapPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class GroupGainMapPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class GroupKVMapPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


NUM_TRANS_IN_UNIT: int = 249
NUM_TRANS_IN_X: int = 18
NUM_TRANS_IN_Y: int = 14
TRANS_SPACING_MM: float = 10.16
DEVICE_HEIGHT_MM: float = 151.4
DEVICE_WIDTH_MM: float = 192.0
FPGA_CLK_FREQ: int = 163840000
FPGA_SUB_CLK_FREQ: int = 20480000
AUTD3_ERR: int = -1
AUTD3_TRUE: int = 1
AUTD3_FALSE: int = 0
