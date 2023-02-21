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
proc AUTDSetLogLevel*(level: int32) {.cdecl, importc: "AUTDSetLogLevel", dynlib: dll.}
proc AUTDSetDefaultLogger*(`out`: pointer; flush: pointer) {.cdecl,
    importc: "AUTDSetDefaultLogger", dynlib: dll.}
proc AUTDCreateGeometryBuilder*(`out`: ptr pointer) {.cdecl,
    importc: "AUTDCreateGeometryBuilder", dynlib: dll.}
proc AUTDAddDevice*(geometry_builder: pointer; x: float64; y: float64; z: float64;
                   rz1: float64; ry: float64; rz2: float64): bool {.cdecl,
    importc: "AUTDAddDevice", dynlib: dll.}
proc AUTDAddDeviceQuaternion*(geometry_builder: pointer; x: float64; y: float64;
                             z: float64; qw: float64; qx: float64; qy: float64;
                             qz: float64): bool {.cdecl,
    importc: "AUTDAddDeviceQuaternion", dynlib: dll.}
proc AUTDBuildGeometry*(`out`: ptr pointer; geometry_builder: pointer) {.cdecl,
    importc: "AUTDBuildGeometry", dynlib: dll.}
proc AUTDFreeGeometry*(geometry: pointer) {.cdecl, importc: "AUTDFreeGeometry",
    dynlib: dll.}
proc AUTDOpenController*(`out`: ptr pointer; geometry: pointer; link: pointer): bool {.
    cdecl, importc: "AUTDOpenController", dynlib: dll.}
proc AUTDGetGeometry*(geometry: ptr pointer; cnt: pointer) {.cdecl,
    importc: "AUTDGetGeometry", dynlib: dll.}
proc AUTDClose*(handle: pointer): bool {.cdecl, importc: "AUTDClose", dynlib: dll.}
proc AUTDFreeController*(handle: pointer) {.cdecl, importc: "AUTDFreeController",
    dynlib: dll.}
proc AUTDIsOpen*(handle: pointer): bool {.cdecl, importc: "AUTDIsOpen", dynlib: dll.}
proc AUTDGetForceFan*(handle: pointer): bool {.cdecl, importc: "AUTDGetForceFan",
    dynlib: dll.}
proc AUTDGetReadsFPGAInfo*(handle: pointer): bool {.cdecl,
    importc: "AUTDGetReadsFPGAInfo", dynlib: dll.}
proc AUTDGetAckCheckTimeout*(handle: pointer): uint64 {.cdecl,
    importc: "AUTDGetAckCheckTimeout", dynlib: dll.}
proc AUTDGetSendInterval*(handle: pointer): uint64 {.cdecl,
    importc: "AUTDGetSendInterval", dynlib: dll.}
proc AUTDSetReadsFPGAInfo*(handle: pointer; reads_fpga_info: bool) {.cdecl,
    importc: "AUTDSetReadsFPGAInfo", dynlib: dll.}
proc AUTDSetAckCheckTimeout*(handle: pointer; timeout: uint64) {.cdecl,
    importc: "AUTDSetAckCheckTimeout", dynlib: dll.}
proc AUTDSetSendInterval*(handle: pointer; interval: uint64) {.cdecl,
    importc: "AUTDSetSendInterval", dynlib: dll.}
proc AUTDSetForceFan*(handle: pointer; force: bool) {.cdecl,
    importc: "AUTDSetForceFan", dynlib: dll.}
proc AUTDGetSoundSpeed*(geometry: pointer): float64 {.cdecl,
    importc: "AUTDGetSoundSpeed", dynlib: dll.}
proc AUTDSetSoundSpeed*(geometry: pointer; sound_speed: float64) {.cdecl,
    importc: "AUTDSetSoundSpeed", dynlib: dll.}
proc AUTDSetSoundSpeedFromTemp*(cnt: pointer; temp: float64; k: float64; r: float64;
                               m: float64) {.cdecl,
    importc: "AUTDSetSoundSpeedFromTemp", dynlib: dll.}
proc AUTDGetTransFrequency*(geometry: pointer; trans_idx: int32): float64 {.cdecl,
    importc: "AUTDGetTransFrequency", dynlib: dll.}
proc AUTDSetTransFrequency*(geometry: pointer; trans_idx: int32; frequency: float64) {.
    cdecl, importc: "AUTDSetTransFrequency", dynlib: dll.}
proc AUTDGetTransCycle*(geometry: pointer; trans_idx: int32): uint16 {.cdecl,
    importc: "AUTDGetTransCycle", dynlib: dll.}
proc AUTDSetTransCycle*(geometry: pointer; trans_idx: int32; cycle: uint16) {.cdecl,
    importc: "AUTDSetTransCycle", dynlib: dll.}
proc AUTDGetWavelength*(geometry: pointer; trans_idx: int32): float64 {.cdecl,
    importc: "AUTDGetWavelength", dynlib: dll.}
proc AUTDGetAttenuation*(geometry: pointer): float64 {.cdecl,
    importc: "AUTDGetAttenuation", dynlib: dll.}
proc AUTDSetAttenuation*(geometry: pointer; attenuation: float64) {.cdecl,
    importc: "AUTDSetAttenuation", dynlib: dll.}
proc AUTDGetFPGAInfo*(handle: pointer; `out`: ptr uint8): bool {.cdecl,
    importc: "AUTDGetFPGAInfo", dynlib: dll.}
proc AUTDNumTransducers*(geometry: pointer): int32 {.cdecl,
    importc: "AUTDNumTransducers", dynlib: dll.}
proc AUTDNumDevices*(geometry: pointer): int32 {.cdecl, importc: "AUTDNumDevices",
    dynlib: dll.}
proc AUTDGeometryCenter*(geometry: pointer; x: ptr float64; y: ptr float64;
                        z: ptr float64) {.cdecl, importc: "AUTDGeometryCenter",
                                       dynlib: dll.}
proc AUTDGeometryCenterOf*(geometry: pointer; dev_idx: int32; x: ptr float64;
                          y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDGeometryCenterOf", dynlib: dll.}
proc AUTDTransPosition*(geometry: pointer; trans_idx: int32; x: ptr float64;
                       y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDTransPosition", dynlib: dll.}
proc AUTDTransXDirection*(geometry: pointer; trans_idx: int32; x: ptr float64;
                         y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDTransXDirection", dynlib: dll.}
proc AUTDTransYDirection*(geometry: pointer; trans_idx: int32; x: ptr float64;
                         y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDTransYDirection", dynlib: dll.}
proc AUTDTransZDirection*(geometry: pointer; trans_idx: int32; x: ptr float64;
                         y: ptr float64; z: ptr float64) {.cdecl,
    importc: "AUTDTransZDirection", dynlib: dll.}
proc AUTDGetFirmwareInfoListPointer*(handle: pointer; `out`: ptr pointer): int32 {.
    cdecl, importc: "AUTDGetFirmwareInfoListPointer", dynlib: dll.}
proc AUTDGetFirmwareInfo*(p_firm_info_list: pointer; index: int32; info: cstring;
                         matches_version: ptr bool; is_supported: ptr bool) {.cdecl,
    importc: "AUTDGetFirmwareInfo", dynlib: dll.}
proc AUTDFreeFirmwareInfoListPointer*(p_firm_info_list: pointer) {.cdecl,
    importc: "AUTDFreeFirmwareInfoListPointer", dynlib: dll.}
proc AUTDGetLatestFirmware*(latest_version: cstring) {.cdecl,
    importc: "AUTDGetLatestFirmware", dynlib: dll.}
proc AUTDGainNull*(gain: ptr pointer) {.cdecl, importc: "AUTDGainNull", dynlib: dll.}
proc AUTDGainGrouped*(gain: ptr pointer) {.cdecl, importc: "AUTDGainGrouped",
                                       dynlib: dll.}
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
proc AUTDGainTransducerTest*(gain: ptr pointer) {.cdecl,
    importc: "AUTDGainTransducerTest", dynlib: dll.}
proc AUTDGainTransducerTestSet*(gain: pointer; tr_idx: int32; amp: float64;
                               phase: float64) {.cdecl,
    importc: "AUTDGainTransducerTestSet", dynlib: dll.}
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
proc AUTDModulationLPF*(`mod`: ptr pointer; mod_in: pointer) {.cdecl,
    importc: "AUTDModulationLPF", dynlib: dll.}
proc AUTDModulationCustom*(`mod`: ptr pointer; buffer: ptr float64; size: uint64;
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
proc AUTDFocusSTM*(`out`: ptr pointer) {.cdecl, importc: "AUTDFocusSTM", dynlib: dll.}
proc AUTDGainSTM*(`out`: ptr pointer) {.cdecl, importc: "AUTDGainSTM", dynlib: dll.}
proc AUTDFocusSTMAdd*(stm: pointer; x: float64; y: float64; z: float64; shift: uint8) {.
    cdecl, importc: "AUTDFocusSTMAdd", dynlib: dll.}
proc AUTDGainSTMAdd*(stm: pointer; gain: pointer) {.cdecl, importc: "AUTDGainSTMAdd",
    dynlib: dll.}
proc AUTDGetGainSTMMode*(stm: pointer): uint16 {.cdecl,
    importc: "AUTDGetGainSTMMode", dynlib: dll.}
proc AUTDSetGainSTMMode*(stm: pointer; mode: uint16) {.cdecl,
    importc: "AUTDSetGainSTMMode", dynlib: dll.}
proc AUTDSTMSetFrequency*(stm: pointer; freq: float64): float64 {.cdecl,
    importc: "AUTDSTMSetFrequency", dynlib: dll.}
proc AUTDSTMGetStartIdx*(stm: pointer): int32 {.cdecl, importc: "AUTDSTMGetStartIdx",
    dynlib: dll.}
proc AUTDSTMGetFinishIdx*(stm: pointer): int32 {.cdecl,
    importc: "AUTDSTMGetFinishIdx", dynlib: dll.}
proc AUTDSTMSetStartIdx*(stm: pointer; start_idx: int32) {.cdecl,
    importc: "AUTDSTMSetStartIdx", dynlib: dll.}
proc AUTDSTMSetFinishIdx*(stm: pointer; finish_idx: int32) {.cdecl,
    importc: "AUTDSTMSetFinishIdx", dynlib: dll.}
proc AUTDSTMFrequency*(stm: pointer): float64 {.cdecl, importc: "AUTDSTMFrequency",
    dynlib: dll.}
proc AUTDSTMSamplingFrequency*(stm: pointer): float64 {.cdecl,
    importc: "AUTDSTMSamplingFrequency", dynlib: dll.}
proc AUTDSTMSamplingFrequencyDivision*(stm: pointer): uint32 {.cdecl,
    importc: "AUTDSTMSamplingFrequencyDivision", dynlib: dll.}
proc AUTDSTMSetSamplingFrequencyDivision*(stm: pointer; freq_div: uint32) {.cdecl,
    importc: "AUTDSTMSetSamplingFrequencyDivision", dynlib: dll.}
proc AUTDDeleteSTM*(stm: pointer) {.cdecl, importc: "AUTDDeleteSTM", dynlib: dll.}
proc AUTDSynchronize*(`out`: ptr pointer) {.cdecl, importc: "AUTDSynchronize",
                                        dynlib: dll.}
proc AUTDClear*(`out`: ptr pointer) {.cdecl, importc: "AUTDClear", dynlib: dll.}
proc AUTDUpdateFlags*(`out`: ptr pointer) {.cdecl, importc: "AUTDUpdateFlags",
                                        dynlib: dll.}
proc AUTDStop*(`out`: ptr pointer) {.cdecl, importc: "AUTDStop", dynlib: dll.}
proc AUTDModDelayConfig*(`out`: ptr pointer) {.cdecl, importc: "AUTDModDelayConfig",
    dynlib: dll.}
proc AUTDDeleteSpecialData*(data: pointer) {.cdecl,
    importc: "AUTDDeleteSpecialData", dynlib: dll.}
proc AUTDCreateSilencer*(`out`: ptr pointer; step: uint16; cycle: uint16) {.cdecl,
    importc: "AUTDCreateSilencer", dynlib: dll.}
proc AUTDDeleteSilencer*(config: pointer) {.cdecl, importc: "AUTDDeleteSilencer",
    dynlib: dll.}
proc AUTDSend*(handle: pointer; header: pointer; body: pointer): bool {.cdecl,
    importc: "AUTDSend", dynlib: dll.}
proc AUTDSendSpecial*(handle: pointer; special: pointer): bool {.cdecl,
    importc: "AUTDSendSpecial", dynlib: dll.}
proc AUTDGetTransModDelay*(geometry: pointer; trans_idx: int32): uint16 {.cdecl,
    importc: "AUTDGetTransModDelay", dynlib: dll.}
proc AUTDSetTransModDelay*(geometry: pointer; trans_idx: int32; delay: uint16) {.cdecl,
    importc: "AUTDSetTransModDelay", dynlib: dll.}
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
proc AUTDSoftwareSTMFinish*(handle: pointer) {.cdecl,
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