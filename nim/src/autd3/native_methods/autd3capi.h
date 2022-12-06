// This file was automatically generated from header file

#ifdef C2NIM
#  dynlib dll
#  cdecl
#  if defined(windows)
#    define dll "bin/autd3capi.dll"
#  elif defined(macosx)
#    define dll "bin/libautd3capi.dylib"
#  else
#    define dll "bin/libautd3capi.so"
#  endif
#endif

void AUTDSetLogLevel(int32 level);
void AUTDSetDefaultLogger(void* out, void* flush);
bool AUTDCreateController(void** out, uint8 driver_version);
bool AUTDOpenController(void* handle, void* link);
void AUTDAddDevice(void* handle, float64 x, float64 y, float64 z, float64 rz1, float64 ry, float64 rz2);
void AUTDAddDeviceQuaternion(void* handle, float64 x, float64 y, float64 z, float64 qw, float64 qx, float64 qy, float64 qz);
bool AUTDClose(void* handle);
void AUTDFreeController(void* handle);
bool AUTDIsOpen(void* handle);
bool AUTDGetForceFan(void* handle);
bool AUTDGetReadsFPGAInfo(void* handle);
uint64 AUTDGetAckCheckTimeout(void* handle);
uint64 AUTDGetSendInterval(void* handle);
void AUTDSetReadsFPGAInfo(void* handle, bool reads_fpga_info);
void AUTDSetAckCheckTimeout(void* handle, uint64 timeout);
void AUTDSetSendInterval(void* handle, uint64 interval);
void AUTDSetForceFan(void* handle, bool force);
float64 AUTDGetSoundSpeed(void* handle);
void AUTDSetSoundSpeed(void* handle, float64 sound_speed);
void AUTDSetSoundSpeedFromTemp(void* handle, float64 temp, float64 k, float64 r, float64 m);
float64 AUTDGetTransFrequency(void* handle, int32 trans_idx);
void AUTDSetTransFrequency(void* handle, int32 trans_idx, float64 frequency);
uint16 AUTDGetTransCycle(void* handle, int32 trans_idx);
void AUTDSetTransCycle(void* handle, int32 trans_idx, uint16 cycle);
float64 AUTDGetWavelength(void* handle, int32 trans_idx);
float64 AUTDGetAttenuation(void* handle);
void AUTDSetAttenuation(void* handle, float64 attenuation);
bool AUTDGetFPGAInfo(void* handle, uint8* out);
int32 AUTDNumTransducers(void* handle);
void AUTDTransPosition(void* handle, int32 trans_idx, float64* x, float64* y, float64* z);
void AUTDTransXDirection(void* handle, int32 trans_idx, float64* x, float64* y, float64* z);
void AUTDTransYDirection(void* handle, int32 trans_idx, float64* x, float64* y, float64* z);
void AUTDTransZDirection(void* handle, int32 trans_idx, float64* x, float64* y, float64* z);
int32 AUTDGetFirmwareInfoListPointer(void* handle, void** out);
void AUTDGetFirmwareInfo(void* p_firm_info_list, int32 index, char* info);
void AUTDFreeFirmwareInfoListPointer(void* p_firm_info_list);
void AUTDGainNull(void** gain);
void AUTDGainGrouped(void** gain, void* handle);
void AUTDGainGroupedAdd(void* grouped_gain, int32 device_id, void* gain);
void AUTDGainFocus(void** gain, float64 x, float64 y, float64 z, float64 amp);
void AUTDGainBesselBeam(void** gain, float64 x, float64 y, float64 z, float64 n_x, float64 n_y, float64 n_z, float64 theta_z, float64 amp);
void AUTDGainPlaneWave(void** gain, float64 n_x, float64 n_y, float64 n_z, float64 amp);
void AUTDGainTransducerTest(void** gain);
void AUTDGainTransducerTestSet(void* gain, int32 tr_idx, float64 amp, float64 phase);
void AUTDGainCustom(void** gain, float64* amp, float64* phase, uint64 size);
void AUTDDeleteGain(void* gain);
void AUTDModulationStatic(void** mod, float64 amp);
void AUTDModulationSine(void** mod, int32 freq, float64 amp, float64 offset);
void AUTDModulationSineSquared(void** mod, int32 freq, float64 amp, float64 offset);
void AUTDModulationSineLegacy(void** mod, float64 freq, float64 amp, float64 offset);
void AUTDModulationSquare(void** mod, int32 freq, float64 low, float64 high, float64 duty);
void AUTDModulationLPF(void** mod, void* mod_in);
void AUTDModulationCustom(void** mod, uint8* buffer, uint64 size, uint32 freq_div);
uint32 AUTDModulationSamplingFrequencyDivision(void* mod);
void AUTDModulationSetSamplingFrequencyDivision(void* mod, uint32 freq_div);
float64 AUTDModulationSamplingFrequency(void* mod);
void AUTDDeleteModulation(void* mod);
void AUTDFocusSTM(void** out, float64 sound_speed);
void AUTDGainSTM(void** out, void* handle);
void AUTDFocusSTMAdd(void* stm, float64 x, float64 y, float64 z, uint8 shift);
void AUTDGainSTMAdd(void* stm, void* gain);
uint16 AUTDGetGainSTMMode(void* stm);
void AUTDSetGainSTMMode(void* stm, uint16 mode);
float64 AUTDSTMSetFrequency(void* stm, float64 freq);
float64 AUTDSTMFrequency(void* stm);
float64 AUTDSTMSamplingFrequency(void* stm);
uint32 AUTDSTMSamplingFrequencyDivision(void* stm);
void AUTDSTMSetSamplingFrequencyDivision(void* stm, uint32 freq_div);
void AUTDDeleteSTM(void* stm);
void AUTDSynchronize(void** out);
void AUTDClear(void** out);
void AUTDUpdateFlags(void** out);
void AUTDStop(void** out);
void AUTDModDelayConfig(void** out);
void AUTDDeleteSpecialData(void* data);
void AUTDCreateSilencer(void** out, uint16 step, uint16 cycle);
void AUTDDeleteSilencer(void* config);
bool AUTDSend(void* handle, void* header, void* body);
bool AUTDSendSpecial(void* handle, void* special);
void AUTDSendAsync(void* handle, void* header, void* body);
void AUTDSendSpecialAsync(void* handle, void* special);
uint16 AUTDGetModDelay(void* handle, int32 trans_idx);
void AUTDSetModDelay(void* handle, int32 trans_idx, uint16 delay);
void AUTDCreateAmplitudes(void** out, float64 amp);
void AUTDDeleteAmplitudes(void* amplitudes);
void AUTDSetMode(void* handle, uint8 mode);
void AUTDSoftwareSTM(void** out);
void AUTDSoftwareSTMSetStrategy(void* stm, uint8 strategy);
void AUTDSoftwareSTMAdd(void* stm, void* gain);
void AUTDSoftwareSTMStart(void** handle, void* stm, void* cnt);
void AUTDSoftwareSTMFinish(void* handle);
float64 AUTDSoftwareSTMSetFrequency(void* stm, float64 freq);
float64 AUTDSoftwareSTMFrequency(void* stm);
uint64 AUTDSoftwareSTMPeriod(void* stm);
float64 AUTDSoftwareSTMSamplingFrequency(void* stm);
uint64 AUTDSoftwareSTMSamplingPeriod(void* stm);
void AUTDSoftwareSTMSetSamplingPeriod(void* stm, uint64 period);
void AUTDDeleteSoftwareSTM(void* stm);
