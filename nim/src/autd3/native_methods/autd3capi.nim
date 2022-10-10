##  This file was automatically generated from header file

{.deadCodeElim: on.}
when defined(windows):
  const
    dll* = "bin/autd3capi.dll"
elif defined(macosx):
  const
    dll* = "bin/libautd3capi.dylib"
else:
  const
    dll* = "bin/libautd3capi.so"
proc AUTDGetLastError*(error: cstring): int32 {.cdecl, importc: "AUTDGetLastError",
    dynlib: dll.}
proc AUTDCreateController*(`out`: ptr pointer) {.cdecl,
    importc: "AUTDCreateController", dynlib: dll.}
proc AUTDOpenController*(handle: pointer; link: pointer): bool {.cdecl,
    importc: "AUTDOpenController", dynlib: dll.}
proc AUTDAddDevice*(handle: pointer; x: float64; y: float64; z: float64; rz1: float64;
                   ry: float64; rz2: float64): int32 {.cdecl,
    importc: "AUTDAddDevice", dynlib: dll.}
proc AUTDAddDeviceQuaternion*(handle: pointer; x: float64; y: float64; z: float64;
                             qw: float64; qx: float64; qy: float64; qz: float64): int32 {.
    cdecl, importc: "AUTDAddDeviceQuaternion", dynlib: dll.}
proc AUTDClose*(handle: pointer): int32 {.cdecl, importc: "AUTDClose", dynlib: dll.}
proc AUTDClear*(handle: pointer): int32 {.cdecl, importc: "AUTDClear", dynlib: dll.}
proc AUTDSynchronize*(handle: pointer): int32 {.cdecl, importc: "AUTDSynchronize",
    dynlib: dll.}
proc AUTDFreeController*(handle: pointer) {.cdecl, importc: "AUTDFreeController",
    dynlib: dll.}
proc AUTDIsOpen*(handle: pointer): bool {.cdecl, importc: "AUTDIsOpen", dynlib: dll.}
proc AUTDGetForceFan*(handle: pointer): bool {.cdecl, importc: "AUTDGetForceFan",
    dynlib: dll.}
proc AUTDGetReadsFPGAInfo*(handle: pointer): bool {.cdecl,
    importc: "AUTDGetReadsFPGAInfo", dynlib: dll.}
proc AUTDGetCheckTrials*(handle: pointer): int32 {.cdecl,
    importc: "AUTDGetCheckTrials", dynlib: dll.}
proc AUTDGetSendInterval*(handle: pointer): int32 {.cdecl,
    importc: "AUTDGetSendInterval", dynlib: dll.}
proc AUTDSetReadsFPGAInfo*(handle: pointer; reads_fpga_info: bool) {.cdecl,
    importc: "AUTDSetReadsFPGAInfo", dynlib: dll.}
proc AUTDSetCheckTrials*(handle: pointer; trials: int32) {.cdecl,
    importc: "AUTDSetCheckTrials", dynlib: dll.}
proc AUTDSetSendInterval*(handle: pointer; interval: int32) {.cdecl,
    importc: "AUTDSetSendInterval", dynlib: dll.}
proc AUTDSetForceFan*(handle: pointer; force: bool) {.cdecl,
    importc: "AUTDSetForceFan", dynlib: dll.}
proc AUTDGetSoundSpeed*(handle: pointer): float64 {.cdecl,
    importc: "AUTDGetSoundSpeed", dynlib: dll.}
proc AUTDSetSoundSpeed*(handle: pointer; sound_speed: float64) {.cdecl,
    importc: "AUTDSetSoundSpeed", dynlib: dll.}
proc AUTDGetTransFrequency*(handle: pointer; device_idx: int32;
                           local_trans_idx: int32): float64 {.cdecl,
    importc: "AUTDGetTransFrequency", dynlib: dll.}
proc AUTDSetTransFrequency*(handle: pointer; device_idx: int32;
                           local_trans_idx: int32; frequency: float64) {.cdecl,
    importc: "AUTDSetTransFrequency", dynlib: dll.}
proc AUTDGetTransCycle*(handle: pointer; device_idx: int32; local_trans_idx: int32): uint16 {.
    cdecl, importc: "AUTDGetTransCycle", dynlib: dll.}
proc AUTDSetTransCycle*(handle: pointer; device_idx: int32; local_trans_idx: int32;
                       cycle: uint16) {.cdecl, importc: "AUTDSetTransCycle",
                                      dynlib: dll.}
proc AUTDGetWavelength*(handle: pointer; device_idx: int32; local_trans_idx: int32): float64 {.
    cdecl, importc: "AUTDGetWavelength", dynlib: dll.}
proc AUTDGetAttenuation*(handle: pointer): float64 {.cdecl,
    importc: "AUTDGetAttenuation", dynlib: dll.}
proc AUTDSetAttenuation*(handle: pointer; attenuation: float64) {.cdecl,
    importc: "AUTDSetAttenuation", dynlib: dll.}
proc AUTDGetFPGAInfo*(handle: pointer; `out`: ptr uint8): bool {.cdecl,
    importc: "AUTDGetFPGAInfo", dynlib: dll.}
proc AUTDUpdateFlags*(handle: pointer): int32 {.cdecl, importc: "AUTDUpdateFlags",
    dynlib: dll.}
proc AUTDNumDevices*(handle: pointer): int32 {.cdecl, importc: "AUTDNumDevices",
    dynlib: dll.}
proc AUTDTransPosition*(handle: pointer; device_idx: int32; local_trans_idx: int32;
                       x: ptr float64; y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDTransPosition", dynlib: dll.}
proc AUTDTransXDirection*(handle: pointer; device_idx: int32; local_trans_idx: int32;
                         x: ptr float64; y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDTransXDirection", dynlib: dll.}
proc AUTDTransYDirection*(handle: pointer; device_idx: int32; local_trans_idx: int32;
                         x: ptr float64; y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDTransYDirection", dynlib: dll.}
proc AUTDTransZDirection*(handle: pointer; device_idx: int32; local_trans_idx: int32;
                         x: ptr float64; y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDTransZDirection", dynlib: dll.}
proc AUTDGetFirmwareInfoListPointer*(handle: pointer; `out`: ptr pointer): int32 {.
    cdecl, importc: "AUTDGetFirmwareInfoListPointer", dynlib: dll.}
proc AUTDGetFirmwareInfo*(p_firm_info_list: pointer; index: int32; info: cstring) {.
    cdecl, importc: "AUTDGetFirmwareInfo", dynlib: dll.}
proc AUTDFreeFirmwareInfoListPointer*(p_firm_info_list: pointer) {.cdecl,
    importc: "AUTDFreeFirmwareInfoListPointer", dynlib: dll.}
proc AUTDGainNull*(gain: ptr pointer) {.cdecl, importc: "AUTDGainNull", dynlib: dll.}
proc AUTDGainGrouped*(gain: ptr pointer; handle: pointer) {.cdecl,
    importc: "AUTDGainGrouped", dynlib: dll.}
proc AUTDGainGroupedAdd*(grouped_gain: pointer; device_id: int32; gain: pointer) {.
    cdecl, importc: "AUTDGainGroupedAdd", dynlib: dll.}
proc AUTDGainFocus*(gain: ptr pointer; x: float64; y: float64; z: float64; amp: float64) {.
    cdecl, importc: "AUTDGainFocus", dynlib: dll.}
proc AUTDGainBesselBeam*(gain: ptr pointer; x: float64; y: float64; z: float64;
                        n_x: float64; n_y: float64; n_z: float64; theta_z: float64;
                        amp: float64) {.cdecl, importc: "AUTDGainBesselBeam",
                                      dynlib: dll.}
proc AUTDGainPlaneWave*(gain: ptr pointer; n_x: float64; n_y: float64; n_z: float64;
                       amp: float64) {.cdecl, importc: "AUTDGainPlaneWave",
                                     dynlib: dll.}
proc AUTDGainCustom*(gain: ptr pointer; amp: ptr float64; phase: ptr float64; size: uint64) {.
    cdecl, importc: "AUTDGainCustom", dynlib: dll.}
proc AUTDDeleteGain*(gain: pointer) {.cdecl, importc: "AUTDDeleteGain", dynlib: dll.}
proc AUTDModulationStatic*(`mod`: ptr pointer; amp: float64) {.cdecl,
    importc: "AUTDModulationStatic", dynlib: dll.}
proc AUTDModulationSine*(`mod`: ptr pointer; freq: int32; amp: float64; offset: float64) {.
    cdecl, importc: "AUTDModulationSine", dynlib: dll.}
proc AUTDModulationSineSquared*(`mod`: ptr pointer; freq: int32; amp: float64;
                               offset: float64) {.cdecl,
    importc: "AUTDModulationSineSquared", dynlib: dll.}
proc AUTDModulationSineLegacy*(`mod`: ptr pointer; freq: float64; amp: float64;
                              offset: float64) {.cdecl,
    importc: "AUTDModulationSineLegacy", dynlib: dll.}
proc AUTDModulationSquare*(`mod`: ptr pointer; freq: int32; low: float64; high: float64;
                          duty: float64) {.cdecl, importc: "AUTDModulationSquare",
    dynlib: dll.}
proc AUTDModulationCustom*(`mod`: ptr pointer; buffer: ptr uint8; size: uint64;
                          freq_div: uint32) {.cdecl,
    importc: "AUTDModulationCustom", dynlib: dll.}
proc AUTDModulationSamplingFrequencyDivision*(`mod`: pointer): uint32 {.cdecl,
    importc: "AUTDModulationSamplingFrequencyDivision", dynlib: dll.}
proc AUTDModulationSetSamplingFrequencyDivision*(`mod`: pointer; freq_div: uint32) {.
    cdecl, importc: "AUTDModulationSetSamplingFrequencyDivision", dynlib: dll.}
proc AUTDModulationSamplingFrequency*(`mod`: pointer): float64 {.cdecl,
    importc: "AUTDModulationSamplingFrequency", dynlib: dll.}
proc AUTDDeleteModulation*(`mod`: pointer) {.cdecl, importc: "AUTDDeleteModulation",
    dynlib: dll.}
proc AUTDPointSTM*(`out`: ptr pointer) {.cdecl, importc: "AUTDPointSTM", dynlib: dll.}
proc AUTDGainSTM*(`out`: ptr pointer; handle: pointer) {.cdecl, importc: "AUTDGainSTM",
    dynlib: dll.}
proc AUTDPointSTMAdd*(stm: pointer; x: float64; y: float64; z: float64; shift: uint8): bool {.
    cdecl, importc: "AUTDPointSTMAdd", dynlib: dll.}
proc AUTDGainSTMAdd*(stm: pointer; gain: pointer): bool {.cdecl,
    importc: "AUTDGainSTMAdd", dynlib: dll.}
proc AUTDGetGainSTMMode*(stm: pointer): uint16 {.cdecl,
    importc: "AUTDGetGainSTMMode", dynlib: dll.}
proc AUTDSetGainSTMMode*(stm: pointer; mode: uint16) {.cdecl,
    importc: "AUTDSetGainSTMMode", dynlib: dll.}
proc AUTDSTMSetFrequency*(stm: pointer; freq: float64): float64 {.cdecl,
    importc: "AUTDSTMSetFrequency", dynlib: dll.}
proc AUTDSTMFrequency*(stm: pointer): float64 {.cdecl, importc: "AUTDSTMFrequency",
    dynlib: dll.}
proc AUTDSTMSamplingFrequency*(stm: pointer): float64 {.cdecl,
    importc: "AUTDSTMSamplingFrequency", dynlib: dll.}
proc AUTDSTMSamplingFrequencyDivision*(stm: pointer): uint32 {.cdecl,
    importc: "AUTDSTMSamplingFrequencyDivision", dynlib: dll.}
proc AUTDSTMSetSamplingFrequencyDivision*(stm: pointer; freq_div: uint32) {.cdecl,
    importc: "AUTDSTMSetSamplingFrequencyDivision", dynlib: dll.}
proc AUTDDeleteSTM*(stm: pointer) {.cdecl, importc: "AUTDDeleteSTM", dynlib: dll.}
proc AUTDStop*(handle: pointer): int32 {.cdecl, importc: "AUTDStop", dynlib: dll.}
proc AUTDCreateSilencer*(`out`: ptr pointer; step: uint16; cycle: uint16) {.cdecl,
    importc: "AUTDCreateSilencer", dynlib: dll.}
proc AUTDDeleteSilencer*(config: pointer) {.cdecl, importc: "AUTDDeleteSilencer",
    dynlib: dll.}
proc AUTDSend*(handle: pointer; header: pointer; body: pointer): int32 {.cdecl,
    importc: "AUTDSend", dynlib: dll.}
proc AUTDSetModDelay*(handle: pointer; device_idx: int32; local_trans_idx: int32;
                     delay: uint16) {.cdecl, importc: "AUTDSetModDelay", dynlib: dll.}
proc AUTDCreateModDelayConfig*(`out`: ptr pointer) {.cdecl,
    importc: "AUTDCreateModDelayConfig", dynlib: dll.}
proc AUTDDeleteModDelayConfig*(config: pointer) {.cdecl,
    importc: "AUTDDeleteModDelayConfig", dynlib: dll.}
proc AUTDCreateAmplitudes*(`out`: ptr pointer; amp: float64) {.cdecl,
    importc: "AUTDCreateAmplitudes", dynlib: dll.}
proc AUTDDeleteAmplitudes*(amplitudes: pointer) {.cdecl,
    importc: "AUTDDeleteAmplitudes", dynlib: dll.}
proc AUTDSetMode*(handle: pointer; mode: uint8) {.cdecl, importc: "AUTDSetMode",
    dynlib: dll.}
proc AUTDSoftwareSTM*(`out`: ptr pointer) {.cdecl, importc: "AUTDSoftwareSTM",
                                        dynlib: dll.}
proc AUTDSoftwareSTMSetStrategy*(stm: pointer; strategy: uint8) {.cdecl,
    importc: "AUTDSoftwareSTMSetStrategy", dynlib: dll.}
proc AUTDSoftwareSTMAdd*(stm: pointer; gain: pointer) {.cdecl,
    importc: "AUTDSoftwareSTMAdd", dynlib: dll.}
proc AUTDSoftwareSTMStart*(handle: ptr pointer; stm: pointer; cnt: pointer) {.cdecl,
    importc: "AUTDSoftwareSTMStart", dynlib: dll.}
proc AUTDSoftwareSTMFinish*(cnt: ptr pointer; handle: pointer) {.cdecl,
    importc: "AUTDSoftwareSTMFinish", dynlib: dll.}
proc AUTDSoftwareSTMSetFrequency*(stm: pointer; freq: float64): float64 {.cdecl,
    importc: "AUTDSoftwareSTMSetFrequency", dynlib: dll.}
proc AUTDSoftwareSTMFrequency*(stm: pointer): float64 {.cdecl,
    importc: "AUTDSoftwareSTMFrequency", dynlib: dll.}
proc AUTDSoftwareSTMPeriod*(stm: pointer): uint64 {.cdecl,
    importc: "AUTDSoftwareSTMPeriod", dynlib: dll.}
proc AUTDSoftwareSTMSamplingFrequency*(stm: pointer): float64 {.cdecl,
    importc: "AUTDSoftwareSTMSamplingFrequency", dynlib: dll.}
proc AUTDSoftwareSTMSamplingPeriod*(stm: pointer): uint64 {.cdecl,
    importc: "AUTDSoftwareSTMSamplingPeriod", dynlib: dll.}
proc AUTDSoftwareSTMSetSamplingPeriod*(stm: pointer; period: uint64) {.cdecl,
    importc: "AUTDSoftwareSTMSetSamplingPeriod", dynlib: dll.}
proc AUTDDeleteSoftwareSTM*(stm: pointer) {.cdecl, importc: "AUTDDeleteSoftwareSTM",
    dynlib: dll.}