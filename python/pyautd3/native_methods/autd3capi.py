# This file was automatically generated from header file
import threading
import ctypes
import os
from enum import IntEnum

class Singleton(type):
    _instances = {}
    _lock = threading.Lock()

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            with cls._lock:
                if cls not in cls._instances:
                    cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]


class NativeMethods(metaclass=Singleton):
    def init_path(self, bin_location: str, bin_prefix: str, bin_ext: str):
        self.bin = bin_location
        self.prefix = bin_prefix
        self.ext = bin_ext

    def init_dll(self):
        if hasattr(self, 'dll'):
            return
        self.dll = ctypes.CDLL(os.path.join(self.bin, f'{self.prefix}autd3capi{self.ext}'))

        self.NUM_TRANS_IN_UNIT = 249
        self.NUM_TRANS_IN_X = 18
        self.NUM_TRANS_IN_Y = 14
        self.TRANS_SPACING_MM = 10.16
        self.DEVICE_HEIGHT = 151.4
        self.DEVICE_WIDTH = 192.0
        self.FPGA_CLK_FREQ = 163840000
        self.FPGA_SUB_CLK_FREQ = 20480000
        self.ERR = - 1
        self.TRUE = 1
        self.FALSE = 0

        self.dll.AUTDCreateGeometryBuilder.argtypes = [] 
        self.dll.AUTDCreateGeometryBuilder.restype = ctypes.c_void_p

        self.dll.AUTDAddDevice.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDAddDevice.restype = None

        self.dll.AUTDAddDeviceQuaternion.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDAddDeviceQuaternion.restype = None

        self.dll.AUTDBuildGeometry.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDBuildGeometry.restype = ctypes.c_void_p

        self.dll.AUTDOpenController.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDOpenController.restype = ctypes.c_void_p

        self.dll.AUTDClose.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDClose.restype = ctypes.c_bool

        self.dll.AUTDFreeController.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFreeController.restype = None

        self.dll.AUTDSetReadsFPGAInfo.argtypes = [ctypes.c_void_p, ctypes.c_bool] 
        self.dll.AUTDSetReadsFPGAInfo.restype = None

        self.dll.AUTDSetForceFan.argtypes = [ctypes.c_void_p, ctypes.c_bool] 
        self.dll.AUTDSetForceFan.restype = None

        self.dll.AUTDGetSoundSpeed.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetSoundSpeed.restype = ctypes.c_double

        self.dll.AUTDSetSoundSpeed.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDSetSoundSpeed.restype = None

        self.dll.AUTDSetSoundSpeedFromTemp.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDSetSoundSpeedFromTemp.restype = None

        self.dll.AUTDGetTransFrequency.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGetTransFrequency.restype = ctypes.c_double

        self.dll.AUTDSetTransFrequency.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_double, ctypes.c_char_p] 
        self.dll.AUTDSetTransFrequency.restype = ctypes.c_bool

        self.dll.AUTDGetTransCycle.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGetTransCycle.restype = ctypes.c_uint16

        self.dll.AUTDSetTransCycle.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint16, ctypes.c_char_p] 
        self.dll.AUTDSetTransCycle.restype = ctypes.c_bool

        self.dll.AUTDGetWavelength.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGetWavelength.restype = ctypes.c_double

        self.dll.AUTDGetAttenuation.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetAttenuation.restype = ctypes.c_double

        self.dll.AUTDSetAttenuation.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDSetAttenuation.restype = None

        self.dll.AUTDGetFPGAInfo.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.c_char_p] 
        self.dll.AUTDGetFPGAInfo.restype = ctypes.c_bool

        self.dll.AUTDNumTransducers.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDNumTransducers.restype = ctypes.c_uint32

        self.dll.AUTDNumDevices.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDNumDevices.restype = ctypes.c_uint32

        self.dll.AUTDGeometryCenter.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDGeometryCenter.restype = None

        self.dll.AUTDGeometryCenterOf.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDGeometryCenterOf.restype = None

        self.dll.AUTDTransPosition.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDTransPosition.restype = None

        self.dll.AUTDTransXDirection.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDTransXDirection.restype = None

        self.dll.AUTDTransYDirection.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDTransYDirection.restype = None

        self.dll.AUTDTransZDirection.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDTransZDirection.restype = None

        self.dll.AUTDGetTransModDelay.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGetTransModDelay.restype = ctypes.c_uint16

        self.dll.AUTDSetTransModDelay.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint16] 
        self.dll.AUTDSetTransModDelay.restype = None

        self.dll.AUTDGetFirmwareInfoListPointer.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDGetFirmwareInfoListPointer.restype = ctypes.c_void_p

        self.dll.AUTDGetFirmwareInfo.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_char_p, ctypes.POINTER(ctypes.c_bool), ctypes.POINTER(ctypes.c_bool)] 
        self.dll.AUTDGetFirmwareInfo.restype = None

        self.dll.AUTDFreeFirmwareInfoListPointer.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFreeFirmwareInfoListPointer.restype = None

        self.dll.AUTDGetLatestFirmware.argtypes = [ctypes.c_char_p] 
        self.dll.AUTDGetLatestFirmware.restype = None

        self.dll.AUTDGainNull.argtypes = [] 
        self.dll.AUTDGainNull.restype = ctypes.c_void_p

        self.dll.AUTDGainGrouped.argtypes = [] 
        self.dll.AUTDGainGrouped.restype = ctypes.c_void_p

        self.dll.AUTDGainGroupedAdd.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_void_p] 
        self.dll.AUTDGainGroupedAdd.restype = None

        self.dll.AUTDGainFocus.argtypes = [ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainFocus.restype = ctypes.c_void_p

        self.dll.AUTDGainBesselBeam.argtypes = [ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainBesselBeam.restype = ctypes.c_void_p

        self.dll.AUTDGainPlaneWave.argtypes = [ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainPlaneWave.restype = ctypes.c_void_p

        self.dll.AUTDGainTransducerTest.argtypes = [] 
        self.dll.AUTDGainTransducerTest.restype = ctypes.c_void_p

        self.dll.AUTDGainTransducerTestSet.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainTransducerTestSet.restype = None

        self.dll.AUTDGainCustom.argtypes = [ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64] 
        self.dll.AUTDGainCustom.restype = ctypes.c_void_p

        self.dll.AUTDDeleteGain.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteGain.restype = None

        self.dll.AUTDModulationStatic.argtypes = [ctypes.c_double] 
        self.dll.AUTDModulationStatic.restype = ctypes.c_void_p

        self.dll.AUTDModulationSine.argtypes = [ctypes.c_uint32, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDModulationSine.restype = ctypes.c_void_p

        self.dll.AUTDModulationSineSquared.argtypes = [ctypes.c_uint32, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDModulationSineSquared.restype = ctypes.c_void_p

        self.dll.AUTDModulationSineLegacy.argtypes = [ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDModulationSineLegacy.restype = ctypes.c_void_p

        self.dll.AUTDModulationSquare.argtypes = [ctypes.c_uint32, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDModulationSquare.restype = ctypes.c_void_p

        self.dll.AUTDModulationCustom.argtypes = [ctypes.POINTER(ctypes.c_double), ctypes.c_uint64, ctypes.c_uint32] 
        self.dll.AUTDModulationCustom.restype = ctypes.c_void_p

        self.dll.AUTDModulationSamplingFrequencyDivision.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDModulationSamplingFrequencyDivision.restype = ctypes.c_uint32

        self.dll.AUTDModulationSetSamplingFrequencyDivision.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDModulationSetSamplingFrequencyDivision.restype = None

        self.dll.AUTDModulationSamplingFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDModulationSamplingFrequency.restype = ctypes.c_double

        self.dll.AUTDDeleteModulation.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteModulation.restype = None

        self.dll.AUTDFocusSTM.argtypes = [] 
        self.dll.AUTDFocusSTM.restype = ctypes.c_void_p

        self.dll.AUTDFocusSTMAdd.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_uint8] 
        self.dll.AUTDFocusSTMAdd.restype = None

        self.dll.AUTDFocusSTMSetFrequency.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDFocusSTMSetFrequency.restype = ctypes.c_double

        self.dll.AUTDFocusSTMGetStartIdx.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFocusSTMGetStartIdx.restype = ctypes.c_int32

        self.dll.AUTDFocusSTMGetFinishIdx.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFocusSTMGetFinishIdx.restype = ctypes.c_int32

        self.dll.AUTDFocusSTMSetStartIdx.argtypes = [ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDFocusSTMSetStartIdx.restype = None

        self.dll.AUTDFocusSTMSetFinishIdx.argtypes = [ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDFocusSTMSetFinishIdx.restype = None

        self.dll.AUTDFocusSTMFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFocusSTMFrequency.restype = ctypes.c_double

        self.dll.AUTDFocusSTMSamplingFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFocusSTMSamplingFrequency.restype = ctypes.c_double

        self.dll.AUTDFocusSTMSamplingFrequencyDivision.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFocusSTMSamplingFrequencyDivision.restype = ctypes.c_uint32

        self.dll.AUTDFocusSTMSetSamplingFrequencyDivision.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDFocusSTMSetSamplingFrequencyDivision.restype = None

        self.dll.AUTDDeleteFocusSTM.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteFocusSTM.restype = None

        self.dll.AUTDGainSTM.argtypes = [] 
        self.dll.AUTDGainSTM.restype = ctypes.c_void_p

        self.dll.AUTDGainSTMAdd.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDGainSTMAdd.restype = None

        self.dll.AUTDGainSTMSetMode.argtypes = [ctypes.c_void_p, GainSTMMode] 
        self.dll.AUTDGainSTMSetMode.restype = None

        self.dll.AUTDGainSTMSetFrequency.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainSTMSetFrequency.restype = ctypes.c_double

        self.dll.AUTDGainSTMGetStartIdx.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainSTMGetStartIdx.restype = ctypes.c_int32

        self.dll.AUTDGainSTMGetFinishIdx.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainSTMGetFinishIdx.restype = ctypes.c_int32

        self.dll.AUTDGainSTMSetStartIdx.argtypes = [ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDGainSTMSetStartIdx.restype = None

        self.dll.AUTDGainSTMSetFinishIdx.argtypes = [ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDGainSTMSetFinishIdx.restype = None

        self.dll.AUTDGainSTMFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainSTMFrequency.restype = ctypes.c_double

        self.dll.AUTDGainSTMSamplingFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainSTMSamplingFrequency.restype = ctypes.c_double

        self.dll.AUTDGainSTMSamplingFrequencyDivision.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainSTMSamplingFrequencyDivision.restype = ctypes.c_uint32

        self.dll.AUTDGainSTMSetSamplingFrequencyDivision.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGainSTMSetSamplingFrequencyDivision.restype = None

        self.dll.AUTDDeleteGainSTM.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteGainSTM.restype = None

        self.dll.AUTDSynchronize.argtypes = [] 
        self.dll.AUTDSynchronize.restype = ctypes.c_void_p

        self.dll.AUTDClear.argtypes = [] 
        self.dll.AUTDClear.restype = ctypes.c_void_p

        self.dll.AUTDUpdateFlags.argtypes = [] 
        self.dll.AUTDUpdateFlags.restype = ctypes.c_void_p

        self.dll.AUTDStop.argtypes = [] 
        self.dll.AUTDStop.restype = ctypes.c_void_p

        self.dll.AUTDModDelayConfig.argtypes = [] 
        self.dll.AUTDModDelayConfig.restype = ctypes.c_void_p

        self.dll.AUTDDeleteSpecialData.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteSpecialData.restype = None

        self.dll.AUTDCreateSilencer.argtypes = [ctypes.c_uint16] 
        self.dll.AUTDCreateSilencer.restype = ctypes.c_void_p

        self.dll.AUTDDeleteSilencer.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteSilencer.restype = None

        self.dll.AUTDCreateAmplitudes.argtypes = [ctypes.c_double] 
        self.dll.AUTDCreateAmplitudes.restype = ctypes.c_void_p

        self.dll.AUTDDeleteAmplitudes.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteAmplitudes.restype = None

        self.dll.AUTDSend.argtypes = [ctypes.c_void_p, TransMode, ctypes.c_void_p, ctypes.c_void_p, ctypes.c_int64, ctypes.c_char_p] 
        self.dll.AUTDSend.restype = ctypes.c_int32

        self.dll.AUTDSendSpecial.argtypes = [ctypes.c_void_p, TransMode, ctypes.c_void_p, ctypes.c_int64, ctypes.c_char_p] 
        self.dll.AUTDSendSpecial.restype = ctypes.c_int32

        self.dll.AUTDLinkDebug.argtypes = [] 
        self.dll.AUTDLinkDebug.restype = ctypes.c_void_p

        self.dll.AUTDLinkDebugLogLevel.argtypes = [ctypes.c_void_p, Level] 
        self.dll.AUTDLinkDebugLogLevel.restype = ctypes.c_void_p

        self.dll.AUTDLinkDebugLogFunc.argtypes = [ctypes.c_void_p, Level, ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDLinkDebugLogFunc.restype = ctypes.c_void_p

        self.dll.AUTDLinkDebugTimeout.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkDebugTimeout.restype = ctypes.c_void_p

        self.dll.AUTDLinkDebugBuild.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDLinkDebugBuild.restype = ctypes.c_void_p

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
