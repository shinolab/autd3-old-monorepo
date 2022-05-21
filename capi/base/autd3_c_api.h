// File: autd3_c_api.h
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "./header.h"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD int32_t AUTDGetLastError(char* error);
EXPORT_AUTD void AUTDCreateController(void** out);
EXPORT_AUTD bool AUTDOpenController(void* handle, void* link);
EXPORT_AUTD int32_t AUTDAddDevice(void* handle, double x, double y, double z, double rz1, double ry, double rz2);
EXPORT_AUTD int32_t AUTDAddDeviceQuaternion(void* handle, double x, double y, double z, double qw, double qx, double qy, double qz);
EXPORT_AUTD int32_t AUTDClose(void* handle);
EXPORT_AUTD int32_t AUTDClear(void* handle);
EXPORT_AUTD int32_t AUTDSynchronize(void* handle);
EXPORT_AUTD void AUTDFreeController(const void* handle);
EXPORT_AUTD bool AUTDIsOpen(const void* handle);
EXPORT_AUTD bool AUTDGetForceFan(const void* handle);
EXPORT_AUTD bool AUTDGetReadsFPGAInfo(const void* handle);
EXPORT_AUTD bool AUTDGetCheckAck(const void* handle);
EXPORT_AUTD void AUTDSetReadsFPGAInfo(void* handle, bool reads_fpga_info);
EXPORT_AUTD void AUTDSetCheckAck(void* handle, bool check_ack);
EXPORT_AUTD void AUTDSetForceFan(void* handle, bool force);
EXPORT_AUTD double AUTDGetSoundSpeed(const void* handle);
EXPORT_AUTD double AUTDSetSoundSpeed(void* handle, double sound_speed);
EXPORT_AUTD double AUTDGetTransFrequency(const void* handle, int32_t device_idx, int32_t local_trans_idx);
EXPORT_AUTD void AUTDSetTransFrequency(void* handle, int32_t device_idx, int32_t local_trans_idx, double frequency);
EXPORT_AUTD uint16_t AUTDGetTransCycle(const void* handle, int32_t device_idx, int32_t local_trans_idx);
EXPORT_AUTD void AUTDSetTransCycle(void* handle, int32_t device_idx, int32_t local_trans_idx, uint16_t cycle);
EXPORT_AUTD double AUTDGetWavelength(const void* handle, int32_t device_idx, int32_t local_trans_idx, double sound_speed);
EXPORT_AUTD double AUTDGetAttenuation(const void* handle);
EXPORT_AUTD void AUTDSetAttenuation(void* handle, double attenuation);
EXPORT_AUTD bool AUTDGetFPGAInfo(void* handle, uint8_t* out);
EXPORT_AUTD int32_t AUTDUpdateFlags(void* handle);
EXPORT_AUTD int32_t AUTDNumDevices(const void* handle);
EXPORT_AUTD void AUTDTransPosition(const void* handle, int32_t device_idx, int32_t local_trans_idx, double* x, double* y, double* z);
EXPORT_AUTD void AUTDTransXDirection(const void* handle, int32_t device_idx, int32_t local_trans_idx, double* x, double* y, double* z);
EXPORT_AUTD void AUTDTransYDirection(const void* handle, int32_t device_idx, int32_t local_trans_idx, double* x, double* y, double* z);
EXPORT_AUTD void AUTDTransZDirection(const void* handle, int32_t device_idx, int32_t local_trans_idx, double* x, double* y, double* z);
EXPORT_AUTD int32_t AUTDGetFirmwareInfoListPointer(void* handle, void** out);
EXPORT_AUTD void AUTDGetFirmwareInfo(const void* p_firm_info_list, int32_t index, char* info);
EXPORT_AUTD void AUTDFreeFirmwareInfoListPointer(const void* p_firm_info_list);
EXPORT_AUTD void AUTDGainNull(void** gain);
EXPORT_AUTD void AUTDGainGrouped(void** gain, const void* handle);
EXPORT_AUTD void AUTDGainGroupedAdd(void* grouped_gain, int32_t device_id, void* gain);
EXPORT_AUTD void AUTDGainFocus(void** gain, double x, double y, double z, double amp);
EXPORT_AUTD void AUTDGainBesselBeam(void** gain, double x, double y, double z, double n_x, double n_y, double n_z, double theta_z, double amp);
EXPORT_AUTD void AUTDGainPlaneWave(void** gain, double n_x, double n_y, double n_z, double amp);
EXPORT_AUTD void AUTDGainCustom(void** gain, const double* amp, const double* phase, uint64_t size);
EXPORT_AUTD void AUTDDeleteGain(const void* gain);
EXPORT_AUTD void AUTDModulationStatic(void** mod, double amp);
EXPORT_AUTD void AUTDModulationSine(void** mod, int32_t freq, double amp, double offset);
EXPORT_AUTD void AUTDModulationSineSquared(void** mod, int32_t freq, double amp, double offset);
EXPORT_AUTD void AUTDModulationSineLegacy(void** mod, double freq, double amp, double offset);
EXPORT_AUTD void AUTDModulationSquare(void** mod, int32_t freq, double low, double high, double duty);
EXPORT_AUTD void AUTDModulationCustom(void** mod, const uint8_t* buffer, uint64_t size, uint32_t freq_div);
EXPORT_AUTD uint32_t AUTDModulationSamplingFrequencyDivision(const void* mod);
EXPORT_AUTD void AUTDModulationSetSamplingFrequencyDivision(void* mod, uint32_t freq_div);
EXPORT_AUTD double AUTDModulationSamplingFrequency(const void* mod);
EXPORT_AUTD void AUTDDeleteModulation(const void* mod);
EXPORT_AUTD void AUTDPointSTM(void** out);
EXPORT_AUTD void AUTDGainSTM(void** out, const void* handle);
EXPORT_AUTD bool AUTDPointSTMAdd(void* stm, double x, double y, double z, uint8_t shift);
EXPORT_AUTD bool AUTDGainSTMAdd(void* stm, void* gain);
EXPORT_AUTD double AUTDSTMSetFrequency(void* stm, double freq);
EXPORT_AUTD double AUTDSTMFrequency(const void* stm);
EXPORT_AUTD double AUTDSTMSamplingFrequency(const void* stm);
EXPORT_AUTD uint32_t AUTDSTMSamplingFrequencyDivision(const void* stm);
EXPORT_AUTD void AUTDSTMSetSamplingFrequencyDivision(void* stm, uint32_t freq_div);
EXPORT_AUTD void AUTDDeleteSTM(const void* stm);
EXPORT_AUTD int32_t AUTDStop(void* handle);
EXPORT_AUTD void AUTDCreateSilencer(void** out, uint16_t step, uint16_t cycle);
EXPORT_AUTD void AUTDDeleteSilencer(const void* config);
EXPORT_AUTD int32_t AUTDSendHeader(void* handle, void* header);
EXPORT_AUTD int32_t AUTDSendBody(void* handle, void* body);
EXPORT_AUTD int32_t AUTDSendHeaderBody(void* handle, void* header, void* body);

#ifdef __cplusplus
}
#endif
