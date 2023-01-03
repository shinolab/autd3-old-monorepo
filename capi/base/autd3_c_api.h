// File: autd3_c_api.h
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "./header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDSetLogLevel(IN int32_t level);
EXPORT_AUTD void AUTDSetDefaultLogger(IN void* out, IN void* flush);
EXPORT_AUTD bool AUTDCreateController(OUT void** out, IN uint8_t driver_version);
EXPORT_AUTD bool AUTDOpenController(IN void* handle, IN void* link);
EXPORT_AUTD void AUTDAddDevice(IN void* handle, IN autd3_float_t x, IN autd3_float_t y, IN autd3_float_t z, IN autd3_float_t rz1, IN autd3_float_t ry,
                               IN autd3_float_t rz2);
EXPORT_AUTD void AUTDAddDeviceQuaternion(IN void* handle, IN autd3_float_t x, IN autd3_float_t y, IN autd3_float_t z, IN autd3_float_t qw,
                                         IN autd3_float_t qx, IN autd3_float_t qy, IN autd3_float_t qz);
EXPORT_AUTD bool AUTDClose(IN void* handle);
EXPORT_AUTD void AUTDFreeController(IN const void* handle);
EXPORT_AUTD bool AUTDIsOpen(IN const void* handle);
EXPORT_AUTD bool AUTDGetForceFan(IN const void* handle);
EXPORT_AUTD bool AUTDGetReadsFPGAInfo(IN const void* handle);
EXPORT_AUTD uint64_t AUTDGetAckCheckTimeout(IN const void* handle);
EXPORT_AUTD uint64_t AUTDGetSendInterval(IN const void* handle);
EXPORT_AUTD void AUTDSetReadsFPGAInfo(IN void* handle, IN bool reads_fpga_info);
EXPORT_AUTD void AUTDSetAckCheckTimeout(IN void* handle, IN uint64_t timeout);
EXPORT_AUTD void AUTDSetSendInterval(IN void* handle, IN uint64_t interval);
EXPORT_AUTD void AUTDSetForceFan(IN void* handle, IN bool force);
EXPORT_AUTD autd3_float_t AUTDGetSoundSpeed(IN const void* handle);
EXPORT_AUTD void AUTDSetSoundSpeed(IN void* handle, IN autd3_float_t sound_speed);
EXPORT_AUTD void AUTDSetSoundSpeedFromTemp(IN void* handle, IN autd3_float_t temp, IN autd3_float_t k, IN autd3_float_t r, IN autd3_float_t m);
EXPORT_AUTD autd3_float_t AUTDGetTransFrequency(IN const void* handle, IN int32_t trans_idx);
EXPORT_AUTD void AUTDSetTransFrequency(IN void* handle, IN int32_t trans_idx, IN autd3_float_t frequency);
EXPORT_AUTD uint16_t AUTDGetTransCycle(IN const void* handle, IN int32_t trans_idx);
EXPORT_AUTD void AUTDSetTransCycle(IN void* handle, IN int32_t trans_idx, IN uint16_t cycle);
EXPORT_AUTD autd3_float_t AUTDGetWavelength(IN const void* handle, IN int32_t trans_idx);
EXPORT_AUTD autd3_float_t AUTDGetAttenuation(IN const void* handle);
EXPORT_AUTD void AUTDSetAttenuation(IN void* handle, IN autd3_float_t attenuation);
EXPORT_AUTD bool AUTDGetFPGAInfo(IN void* handle, IN uint8_t* out);
EXPORT_AUTD int32_t AUTDNumTransducers(IN const void* handle);
EXPORT_AUTD int32_t AUTDNumDevices(IN const void* handle);
EXPORT_AUTD void AUTDGeometryCenter(IN const void* handle, OUT autd3_float_t* x, OUT autd3_float_t* y, OUT autd3_float_t* z);
EXPORT_AUTD void AUTDGeometryCenterOf(IN const void* handle, IN int32_t dev_idx, OUT autd3_float_t* x, OUT autd3_float_t* y, OUT autd3_float_t* z);
EXPORT_AUTD void AUTDTransPosition(IN const void* handle, IN int32_t trans_idx, OUT autd3_float_t* x, OUT autd3_float_t* y, OUT autd3_float_t* z);
EXPORT_AUTD void AUTDTransXDirection(IN const void* handle, IN int32_t trans_idx, OUT autd3_float_t* x, OUT autd3_float_t* y, OUT autd3_float_t* z);
EXPORT_AUTD void AUTDTransYDirection(IN const void* handle, IN int32_t trans_idx, OUT autd3_float_t* x, OUT autd3_float_t* y, OUT autd3_float_t* z);
EXPORT_AUTD void AUTDTransZDirection(IN const void* handle, IN int32_t trans_idx, OUT autd3_float_t* x, OUT autd3_float_t* y, OUT autd3_float_t* z);
EXPORT_AUTD int32_t AUTDGetFirmwareInfoListPointer(IN void* handle, OUT void** out);
EXPORT_AUTD void AUTDGetFirmwareInfo(IN const void* p_firm_info_list, IN int32_t index, OUT char* info);
EXPORT_AUTD void AUTDFreeFirmwareInfoListPointer(IN const void* p_firm_info_list);
EXPORT_AUTD void AUTDGainNull(OUT void** gain);
EXPORT_AUTD void AUTDGainGrouped(OUT void** gain, IN const void* handle);
EXPORT_AUTD void AUTDGainGroupedAdd(IN void* grouped_gain, IN int32_t device_id, IN void* gain);
EXPORT_AUTD void AUTDGainFocus(OUT void** gain, IN autd3_float_t x, IN autd3_float_t y, IN autd3_float_t z, IN autd3_float_t amp);
EXPORT_AUTD void AUTDGainBesselBeam(OUT void** gain, IN autd3_float_t x, IN autd3_float_t y, IN autd3_float_t z, IN autd3_float_t n_x,
                                    IN autd3_float_t n_y, IN autd3_float_t n_z, IN autd3_float_t theta_z, IN autd3_float_t amp);
EXPORT_AUTD void AUTDGainPlaneWave(OUT void** gain, IN autd3_float_t n_x, IN autd3_float_t n_y, IN autd3_float_t n_z, IN autd3_float_t amp);
EXPORT_AUTD void AUTDGainTransducerTest(OUT void** gain);
EXPORT_AUTD void AUTDGainTransducerTestSet(IN void* gain, IN int32_t tr_idx, IN autd3_float_t amp, IN autd3_float_t phase);
EXPORT_AUTD void AUTDGainCustom(OUT void** gain, IN const autd3_float_t* amp, IN const autd3_float_t* phase, IN uint64_t size);
EXPORT_AUTD void AUTDDeleteGain(IN const void* gain);
EXPORT_AUTD void AUTDModulationStatic(OUT void** mod, IN autd3_float_t amp);
EXPORT_AUTD void AUTDModulationSine(OUT void** mod, IN int32_t freq, IN autd3_float_t amp, IN autd3_float_t offset);
EXPORT_AUTD void AUTDModulationSineSquared(OUT void** mod, IN int32_t freq, IN autd3_float_t amp, IN autd3_float_t offset);
EXPORT_AUTD void AUTDModulationSineLegacy(OUT void** mod, IN autd3_float_t freq, IN autd3_float_t amp, IN autd3_float_t offset);
EXPORT_AUTD void AUTDModulationSquare(OUT void** mod, IN int32_t freq, IN autd3_float_t low, IN autd3_float_t high, IN autd3_float_t duty);
EXPORT_AUTD void AUTDModulationLPF(OUT void** mod, IN void* mod_in);
EXPORT_AUTD void AUTDModulationCustom(OUT void** mod, IN const uint8_t* buffer, IN uint64_t size, IN uint32_t freq_div);
EXPORT_AUTD uint32_t AUTDModulationSamplingFrequencyDivision(IN const void* mod);
EXPORT_AUTD void AUTDModulationSetSamplingFrequencyDivision(IN void* mod, IN uint32_t freq_div);
EXPORT_AUTD autd3_float_t AUTDModulationSamplingFrequency(IN const void* mod);
EXPORT_AUTD void AUTDDeleteModulation(IN const void* mod);
EXPORT_AUTD void AUTDFocusSTM(OUT void** out);
EXPORT_AUTD void AUTDGainSTM(OUT void** out, IN const void* handle);
EXPORT_AUTD void AUTDFocusSTMAdd(IN void* stm, IN autd3_float_t x, IN autd3_float_t y, IN autd3_float_t z, IN uint8_t shift);
EXPORT_AUTD void AUTDGainSTMAdd(IN void* stm, IN void* gain);
EXPORT_AUTD uint16_t AUTDGetGainSTMMode(IN void* stm);
EXPORT_AUTD void AUTDSetGainSTMMode(IN void* stm, IN uint16_t mode);
EXPORT_AUTD autd3_float_t AUTDSTMSetFrequency(IN void* stm, IN autd3_float_t freq);
EXPORT_AUTD int32_t AUTDSTMGetStartIdx(IN const void* stm);
EXPORT_AUTD int32_t AUTDSTMGetFinishIdx(IN const void* stm);
EXPORT_AUTD void AUTDSTMSetStartIdx(IN void* stm, IN int32_t start_idx);
EXPORT_AUTD void AUTDSTMSetFinishIdx(IN void* stm, IN int32_t finish_idx);
EXPORT_AUTD autd3_float_t AUTDSTMFrequency(IN const void* stm);
EXPORT_AUTD autd3_float_t AUTDSTMSamplingFrequency(IN const void* stm);
EXPORT_AUTD uint32_t AUTDSTMSamplingFrequencyDivision(IN const void* stm);
EXPORT_AUTD void AUTDSTMSetSamplingFrequencyDivision(IN void* stm, IN uint32_t freq_div);
EXPORT_AUTD void AUTDDeleteSTM(IN const void* stm);
EXPORT_AUTD void AUTDSynchronize(OUT void** out);
EXPORT_AUTD void AUTDClear(OUT void** out);
EXPORT_AUTD void AUTDUpdateFlags(OUT void** out);
EXPORT_AUTD void AUTDStop(OUT void** out);
EXPORT_AUTD void AUTDModDelayConfig(OUT void** out);
EXPORT_AUTD void AUTDDeleteSpecialData(IN const void* data);
EXPORT_AUTD void AUTDCreateSilencer(OUT void** out, IN uint16_t step, IN uint16_t cycle);
EXPORT_AUTD void AUTDDeleteSilencer(IN const void* config);
EXPORT_AUTD bool AUTDSend(IN void* handle, IN void* header, IN void* body);
EXPORT_AUTD bool AUTDSendSpecial(IN void* handle, IN void* special);
EXPORT_AUTD void AUTDSendAsync(IN void* handle, IN void* header, IN void* body);
EXPORT_AUTD void AUTDSendSpecialAsync(IN void* handle, IN void* special);
EXPORT_AUTD uint16_t AUTDGetModDelay(IN const void* handle, IN int32_t trans_idx);
EXPORT_AUTD void AUTDSetModDelay(IN void* handle, IN int32_t trans_idx, IN uint16_t delay);
EXPORT_AUTD void AUTDCreateAmplitudes(OUT void** out, IN autd3_float_t amp);
EXPORT_AUTD void AUTDDeleteAmplitudes(IN const void* amplitudes);
EXPORT_AUTD void AUTDSetMode(IN void* handle, IN uint8_t mode);
EXPORT_AUTD void AUTDSoftwareSTM(OUT void** out);
EXPORT_AUTD void AUTDSoftwareSTMSetStrategy(IN void* stm, IN uint8_t strategy);
EXPORT_AUTD void AUTDSoftwareSTMAdd(IN void* stm, IN void* gain);
EXPORT_AUTD void AUTDSoftwareSTMStart(OUT void** handle, IN void* stm, IN void* cnt);
EXPORT_AUTD void AUTDSoftwareSTMFinish(IN void* handle);
EXPORT_AUTD autd3_float_t AUTDSoftwareSTMSetFrequency(IN void* stm, IN autd3_float_t freq);
EXPORT_AUTD autd3_float_t AUTDSoftwareSTMFrequency(IN const void* stm);
EXPORT_AUTD uint64_t AUTDSoftwareSTMPeriod(IN const void* stm);
EXPORT_AUTD autd3_float_t AUTDSoftwareSTMSamplingFrequency(IN const void* stm);
EXPORT_AUTD uint64_t AUTDSoftwareSTMSamplingPeriod(IN const void* stm);
EXPORT_AUTD void AUTDSoftwareSTMSetSamplingPeriod(IN void* stm, IN uint64_t period);
EXPORT_AUTD void AUTDDeleteSoftwareSTM(IN const void* stm);
#ifdef __cplusplus
}
#endif
