# This file was automatically generated from header file
import threading
import ctypes
import os


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

        self.dll.AUTDGetLastError.argtypes = [ctypes.c_char_p] 
        self.dll.AUTDGetLastError.restype = ctypes.c_int32

        self.dll.AUTDCreateController.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDCreateController.restype = None

        self.dll.AUTDOpenController.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDOpenController.restype = ctypes.c_bool

        self.dll.AUTDAddDevice.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDAddDevice.restype = ctypes.c_int32

        self.dll.AUTDAddDeviceQuaternion.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDAddDeviceQuaternion.restype = ctypes.c_int32

        self.dll.AUTDClose.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDClose.restype = ctypes.c_int32

        self.dll.AUTDFreeController.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFreeController.restype = None

        self.dll.AUTDIsOpen.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDIsOpen.restype = ctypes.c_bool

        self.dll.AUTDGetForceFan.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetForceFan.restype = ctypes.c_bool

        self.dll.AUTDGetReadsFPGAInfo.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetReadsFPGAInfo.restype = ctypes.c_bool

        self.dll.AUTDGetCheckTrials.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetCheckTrials.restype = ctypes.c_int32

        self.dll.AUTDGetSendInterval.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetSendInterval.restype = ctypes.c_int32

        self.dll.AUTDSetReadsFPGAInfo.argtypes = [ctypes.c_void_p, ctypes.c_bool] 
        self.dll.AUTDSetReadsFPGAInfo.restype = None

        self.dll.AUTDSetCheckTrials.argtypes = [ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDSetCheckTrials.restype = None

        self.dll.AUTDSetSendInterval.argtypes = [ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDSetSendInterval.restype = None

        self.dll.AUTDSetForceFan.argtypes = [ctypes.c_void_p, ctypes.c_bool] 
        self.dll.AUTDSetForceFan.restype = None

        self.dll.AUTDGetSoundSpeed.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetSoundSpeed.restype = ctypes.c_double

        self.dll.AUTDSetSoundSpeed.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDSetSoundSpeed.restype = None

        self.dll.AUTDGetTransFrequency.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32] 
        self.dll.AUTDGetTransFrequency.restype = ctypes.c_double

        self.dll.AUTDSetTransFrequency.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32, ctypes.c_double] 
        self.dll.AUTDSetTransFrequency.restype = None

        self.dll.AUTDGetTransCycle.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32] 
        self.dll.AUTDGetTransCycle.restype = ctypes.c_uint16

        self.dll.AUTDSetTransCycle.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32, ctypes.c_uint16] 
        self.dll.AUTDSetTransCycle.restype = None

        self.dll.AUTDGetWavelength.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32] 
        self.dll.AUTDGetWavelength.restype = ctypes.c_double

        self.dll.AUTDGetAttenuation.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetAttenuation.restype = ctypes.c_double

        self.dll.AUTDSetAttenuation.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDSetAttenuation.restype = None

        self.dll.AUTDGetFPGAInfo.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8)] 
        self.dll.AUTDGetFPGAInfo.restype = ctypes.c_bool

        self.dll.AUTDNumDevices.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDNumDevices.restype = ctypes.c_int32

        self.dll.AUTDTransPosition.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDTransPosition.restype = None

        self.dll.AUTDTransXDirection.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDTransXDirection.restype = None

        self.dll.AUTDTransYDirection.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDTransYDirection.restype = None

        self.dll.AUTDTransZDirection.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double)] 
        self.dll.AUTDTransZDirection.restype = None

        self.dll.AUTDGetFirmwareInfoListPointer.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDGetFirmwareInfoListPointer.restype = ctypes.c_int32

        self.dll.AUTDGetFirmwareInfo.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_char_p] 
        self.dll.AUTDGetFirmwareInfo.restype = None

        self.dll.AUTDFreeFirmwareInfoListPointer.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFreeFirmwareInfoListPointer.restype = None

        self.dll.AUTDGainNull.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDGainNull.restype = None

        self.dll.AUTDGainGrouped.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p] 
        self.dll.AUTDGainGrouped.restype = None

        self.dll.AUTDGainGroupedAdd.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_void_p] 
        self.dll.AUTDGainGroupedAdd.restype = None

        self.dll.AUTDGainFocus.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainFocus.restype = None

        self.dll.AUTDGainBesselBeam.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainBesselBeam.restype = None

        self.dll.AUTDGainPlaneWave.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainPlaneWave.restype = None

        self.dll.AUTDGainTransducerTest.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_int32, ctypes.c_int32, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainTransducerTest.restype = None

        self.dll.AUTDGainCustom.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64] 
        self.dll.AUTDGainCustom.restype = None

        self.dll.AUTDDeleteGain.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteGain.restype = None

        self.dll.AUTDModulationStatic.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_double] 
        self.dll.AUTDModulationStatic.restype = None

        self.dll.AUTDModulationSine.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_int32, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDModulationSine.restype = None

        self.dll.AUTDModulationSineSquared.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_int32, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDModulationSineSquared.restype = None

        self.dll.AUTDModulationSineLegacy.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDModulationSineLegacy.restype = None

        self.dll.AUTDModulationSquare.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_int32, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDModulationSquare.restype = None

        self.dll.AUTDModulationLPF.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p] 
        self.dll.AUTDModulationLPF.restype = None

        self.dll.AUTDModulationCustom.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.POINTER(ctypes.c_uint8), ctypes.c_uint64, ctypes.c_uint32] 
        self.dll.AUTDModulationCustom.restype = None

        self.dll.AUTDModulationSamplingFrequencyDivision.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDModulationSamplingFrequencyDivision.restype = ctypes.c_uint32

        self.dll.AUTDModulationSetSamplingFrequencyDivision.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDModulationSetSamplingFrequencyDivision.restype = None

        self.dll.AUTDModulationSamplingFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDModulationSamplingFrequency.restype = ctypes.c_double

        self.dll.AUTDDeleteModulation.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteModulation.restype = None

        self.dll.AUTDPointSTM.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDPointSTM.restype = None

        self.dll.AUTDGainSTM.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p] 
        self.dll.AUTDGainSTM.restype = None

        self.dll.AUTDPointSTMAdd.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_uint8] 
        self.dll.AUTDPointSTMAdd.restype = ctypes.c_bool

        self.dll.AUTDGainSTMAdd.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDGainSTMAdd.restype = ctypes.c_bool

        self.dll.AUTDGetGainSTMMode.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGetGainSTMMode.restype = ctypes.c_uint16

        self.dll.AUTDSetGainSTMMode.argtypes = [ctypes.c_void_p, ctypes.c_uint16] 
        self.dll.AUTDSetGainSTMMode.restype = None

        self.dll.AUTDSTMSetFrequency.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDSTMSetFrequency.restype = ctypes.c_double

        self.dll.AUTDSTMFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSTMFrequency.restype = ctypes.c_double

        self.dll.AUTDSTMSamplingFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSTMSamplingFrequency.restype = ctypes.c_double

        self.dll.AUTDSTMSamplingFrequencyDivision.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSTMSamplingFrequencyDivision.restype = ctypes.c_uint32

        self.dll.AUTDSTMSetSamplingFrequencyDivision.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDSTMSetSamplingFrequencyDivision.restype = None

        self.dll.AUTDDeleteSTM.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteSTM.restype = None

        self.dll.AUTDSynchronize.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDSynchronize.restype = None

        self.dll.AUTDClear.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDClear.restype = None

        self.dll.AUTDUpdateFlags.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDUpdateFlags.restype = None

        self.dll.AUTDStop.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDStop.restype = None

        self.dll.AUTDModDelayConfig.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDModDelayConfig.restype = None

        self.dll.AUTDDeleteSpecialData.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteSpecialData.restype = None

        self.dll.AUTDCreateSilencer.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_uint16, ctypes.c_uint16] 
        self.dll.AUTDCreateSilencer.restype = None

        self.dll.AUTDDeleteSilencer.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteSilencer.restype = None

        self.dll.AUTDSend.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDSend.restype = ctypes.c_int32

        self.dll.AUTDSendSpecial.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDSendSpecial.restype = ctypes.c_int32

        self.dll.AUTDSendAsync.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDSendAsync.restype = None

        self.dll.AUTDSendSpecialAsync.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDSendSpecialAsync.restype = None

        self.dll.AUTDGetModDelay.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32] 
        self.dll.AUTDGetModDelay.restype = ctypes.c_uint16

        self.dll.AUTDSetModDelay.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_int32, ctypes.c_uint16] 
        self.dll.AUTDSetModDelay.restype = None

        self.dll.AUTDCreateAmplitudes.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_double] 
        self.dll.AUTDCreateAmplitudes.restype = None

        self.dll.AUTDDeleteAmplitudes.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteAmplitudes.restype = None

        self.dll.AUTDSetMode.argtypes = [ctypes.c_void_p, ctypes.c_uint8] 
        self.dll.AUTDSetMode.restype = None

        self.dll.AUTDSoftwareSTM.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDSoftwareSTM.restype = None

        self.dll.AUTDSoftwareSTMSetStrategy.argtypes = [ctypes.c_void_p, ctypes.c_uint8] 
        self.dll.AUTDSoftwareSTMSetStrategy.restype = None

        self.dll.AUTDSoftwareSTMAdd.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDSoftwareSTMAdd.restype = None

        self.dll.AUTDSoftwareSTMStart.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDSoftwareSTMStart.restype = None

        self.dll.AUTDSoftwareSTMFinish.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSoftwareSTMFinish.restype = None

        self.dll.AUTDSoftwareSTMSetFrequency.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDSoftwareSTMSetFrequency.restype = ctypes.c_double

        self.dll.AUTDSoftwareSTMFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSoftwareSTMFrequency.restype = ctypes.c_double

        self.dll.AUTDSoftwareSTMPeriod.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSoftwareSTMPeriod.restype = ctypes.c_uint64

        self.dll.AUTDSoftwareSTMSamplingFrequency.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSoftwareSTMSamplingFrequency.restype = ctypes.c_double

        self.dll.AUTDSoftwareSTMSamplingPeriod.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSoftwareSTMSamplingPeriod.restype = ctypes.c_uint64

        self.dll.AUTDSoftwareSTMSetSamplingPeriod.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDSoftwareSTMSetSamplingPeriod.restype = None

        self.dll.AUTDDeleteSoftwareSTM.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteSoftwareSTM.restype = None
