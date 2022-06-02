// File: c_api.cpp
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <algorithm>
#include <cstdint>
#include <cstring>
#include <string>
#include <utility>

#include "./autd3_c_api.h"
#include "autd3.hpp"
#include "custom.hpp"
#include "wrapper.hpp"
#include "wrapper_link.hpp"

#define AUTD3_CAPI_TRY(action)    \
  try {                           \
    action;                       \
  } catch (std::exception & ex) { \
    last_error() = ex.what();     \
    return false;                 \
  }

#define AUTD3_CAPI_TRY2(action)   \
  try {                           \
    action;                       \
  } catch (std::exception & ex) { \
    last_error() = ex.what();     \
    return -1;                    \
  }

using T = autd3::core::DynamicTransducer;
using Controller = autd3::ControllerX<T>;

std::string& last_error() {
  static std::string msg("");  // NOLINT
  return msg;
}
autd3::Vector3 to_vec3(const double x, const double y, const double z) { return {x, y, z}; }
autd3::Quaternion to_quaternion(const double w, const double x, const double y, const double z) { return {w, x, y, z}; }

int32_t AUTDGetLastError(char* error) {
  const auto& error_ = last_error();
  const auto size = static_cast<int32_t>(error_.size() + 1);
  if (error == nullptr) return size;
  std::char_traits<char>::copy(error, error_.c_str(), size);
  return size;
}

void AUTDCreateController(void** out) { *out = new Controller; }

bool AUTDOpenController(void* const handle, void* const link) {
  auto* const wrapper = static_cast<Controller*>(handle);
  auto* w_link = static_cast<LinkWrapper*>(link);
  autd3::LinkPtr link_ = std::move(w_link->ptr);
  link_delete(w_link);
  AUTD3_CAPI_TRY(return wrapper->open(std::move(link_)))
}

int32_t AUTDAddDevice(void* const handle, const double x, const double y, const double z, const double rz1, const double ry, const double rz2) {
  auto* const wrapper = static_cast<Controller*>(handle);
  const auto res = wrapper->geometry().add_device(to_vec3(x, y, z), to_vec3(rz1, ry, rz2));
  return static_cast<int32_t>(res);
}

int32_t AUTDAddDeviceQuaternion(void* const handle, const double x, const double y, const double z, const double qw, const double qx, const double qy,
                                const double qz) {
  auto* const wrapper = static_cast<Controller*>(handle);
  const auto res = wrapper->geometry().add_device(to_vec3(x, y, z), to_quaternion(qw, qx, qy, qz));
  return static_cast<int32_t>(res);
}

int32_t AUTDClose(void* const handle) {
  auto* const wrapper = static_cast<Controller*>(handle);
  AUTD3_CAPI_TRY2(return wrapper->close() ? 1 : 0)
}

int32_t AUTDClear(void* const handle) {
  auto* wrapper = static_cast<Controller*>(handle);
  AUTD3_CAPI_TRY2(return wrapper->clear() ? 1 : 0)
}

int32_t AUTDSynchronize(void* const handle) {
  auto* wrapper = static_cast<Controller*>(handle);
  AUTD3_CAPI_TRY2(return wrapper->synchronize() ? 1 : 0)
}
void AUTDFreeController(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  delete wrapper;
}
bool AUTDIsOpen(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->is_open();
}
bool AUTDGetForceFan(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->force_fan;
}
bool AUTDGetReadsFPGAInfo(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->reads_fpga_info;
}
bool AUTDGetCheckAck(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->check_ack;
}
void AUTDSetForceFan(void* const handle, const bool force) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->force_fan = force;
}
void AUTDSetReadsFPGAInfo(void* const handle, const bool reads_fpga_info) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->reads_fpga_info = reads_fpga_info;
}
void AUTDSetCheckAck(void* const handle, const bool check_ack) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->check_ack = check_ack;
}

double AUTDGetTransFrequency(const void* const handle, const int32_t device_idx, const int32_t local_trans_idx) {
  const auto* const wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry()[device_idx][local_trans_idx].frequency();
}

uint16_t AUTDGetTransCycle(const void* const handle, const int32_t device_idx, const int32_t local_trans_idx) {
  const auto* const wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry()[device_idx][local_trans_idx].cycle();
}

double AUTDGetSoundSpeed(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry().sound_speed;
}
void AUTDSetSoundSpeed(void* const handle, const double sound_speed) {
  auto* wrapper = static_cast<Controller*>(handle);
  wrapper->geometry().sound_speed = sound_speed;
}
double AUTDGetWavelength(const void* const handle, const int32_t device_idx, const int32_t local_trans_idx, const double sound_speed) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry()[device_idx][local_trans_idx].wavelength(sound_speed);
}
double AUTDGetAttenuation(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry().attenuation;
}
void AUTDSetAttenuation(void* const handle, const double attenuation) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry().attenuation = attenuation;
}
bool AUTDGetFPGAInfo(void* const handle, uint8_t* out) {
  auto* const wrapper = static_cast<Controller*>(handle);
  AUTD3_CAPI_TRY({
    const auto& res = wrapper->read_fpga_info();
    std::memcpy(out, &res[0], res.size());
    return !res.empty();
  })
}

int32_t AUTDUpdateFlags(void* const handle) {
  auto* const wrapper = static_cast<Controller*>(handle);
  AUTD3_CAPI_TRY2(return wrapper->update_flag() ? 1 : 0)
}

int32_t AUTDNumDevices(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto res = wrapper->geometry().num_devices();
  return static_cast<int32_t>(res);
}

void AUTDTransPosition(const void* const handle, const int32_t device_idx, const int32_t local_trans_idx, double* x, double* y, double* z) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto& pos = wrapper->geometry()[device_idx][local_trans_idx].position();
  *x = pos.x();
  *y = pos.y();
  *z = pos.z();
}

void AUTDTransXDirection(const void* const handle, const int32_t device_idx, const int32_t local_trans_idx, double* x, double* y, double* z) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto& pos = wrapper->geometry()[device_idx][local_trans_idx].x_direction();
  *x = pos.x();
  *y = pos.y();
  *z = pos.z();
}
void AUTDTransYDirection(const void* const handle, const int32_t device_idx, const int32_t local_trans_idx, double* x, double* y, double* z) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto& pos = wrapper->geometry()[device_idx][local_trans_idx].y_direction();
  *x = pos.x();
  *y = pos.y();
  *z = pos.z();
}
void AUTDTransZDirection(const void* const handle, const int32_t device_idx, const int32_t local_trans_idx, double* x, double* y, double* z) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto& pos = wrapper->geometry()[device_idx][local_trans_idx].z_direction();
  *x = pos.x();
  *y = pos.y();
  *z = pos.z();
}

int32_t AUTDGetFirmwareInfoListPointer(void* const handle, void** out) {
  auto* const wrapper = static_cast<Controller*>(handle);
  const auto size = static_cast<int32_t>(wrapper->geometry().num_devices());
  AUTD3_CAPI_TRY2({
    const auto res = wrapper->firmware_infos();
    if (res.empty()) {
      last_error() = "filed to get some infos";
      return -1;
    }
    auto* list = firmware_info_list_create(res);
    *out = list;
    return size;
  })
}

void AUTDGetFirmwareInfo(const void* const p_firm_info_list, const int32_t index, char* info) {
  const auto* wrapper = static_cast<const FirmwareInfoListWrapper*>(p_firm_info_list);
  const auto& info_ = wrapper->list[index].to_string();
  std::char_traits<char>::copy(info, info_.c_str(), info_.size() + 1);
}
void AUTDFreeFirmwareInfoListPointer(const void* const p_firm_info_list) {
  const auto* wrapper = static_cast<const FirmwareInfoListWrapper*>(p_firm_info_list);
  firmware_info_list_delete(wrapper);
}

void AUTDGainNull(void** gain) {
  auto* g = new autd3::gain::Null<T>;
  *gain = g;
}

void AUTDGainGrouped(void** gain, const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  auto* g = new autd3::gain::Grouped(wrapper->geometry());
  *gain = g;
}

void AUTDGainGroupedAdd(void* grouped_gain, const int32_t device_id, void* gain) {
  auto* const gg = dynamic_cast<autd3::gain::Grouped<T>*>(static_cast<autd3::Gain<T>*>(grouped_gain));
  auto* const g = static_cast<autd3::Gain<T>*>(gain);
  gg->add(device_id, *g);
}

void AUTDGainFocus(void** gain, const double x, const double y, const double z, const double amp) {
  *gain = new autd3::gain::Focus<T>(to_vec3(x, y, z), amp);
}
void AUTDGainBesselBeam(void** gain, const double x, const double y, const double z, const double n_x, const double n_y, const double n_z,
                        const double theta_z, const double amp) {
  *gain = new autd3::gain::BesselBeam<T>(to_vec3(x, y, z), to_vec3(n_x, n_y, n_z), theta_z, amp);
}
void AUTDGainPlaneWave(void** gain, const double n_x, const double n_y, const double n_z, const double amp) {
  *gain = new autd3::gain::PlaneWave<T>(to_vec3(n_x, n_y, n_z), amp);
}

void AUTDGainCustom(void** gain, const double* amp, const double* phase, const uint64_t size) {
  *gain = new CustomGain(amp, phase, static_cast<size_t>(size));
}

void AUTDDeleteGain(const void* const gain) {
  const auto* g = static_cast<const autd3::Gain<T>*>(gain);
  delete g;
}

void AUTDModulationStatic(void** mod, const double amp) { *mod = new autd3::modulation::Static(amp); }

void AUTDModulationSquare(void** mod, const int32_t freq, const double low, const double high, const double duty) {
  *mod = new autd3::modulation::Square(freq, low, high, duty);
}
void AUTDModulationSine(void** mod, const int32_t freq, const double amp, const double offset) {
  *mod = new autd3::modulation::Sine(freq, amp, offset);
}
void AUTDModulationSineSquared(void** mod, const int32_t freq, const double amp, const double offset) {
  *mod = new autd3::modulation::SineSquared(freq, amp, offset);
}
void AUTDModulationSineLegacy(void** mod, const double freq, const double amp, const double offset) {
  *mod = new autd3::modulation::SineLegacy(freq, amp, offset);
}
void AUTDModulationCustom(void** mod, const uint8_t* buffer, const uint64_t size, const uint32_t freq_div) {
  *mod = new CustomModulation(buffer, static_cast<size_t>(size), freq_div);
}
uint32_t AUTDModulationSamplingFrequencyDivision(const void* const mod) {
  const auto* const m = static_cast<const autd3::Modulation*>(mod);
  return m->sampling_frequency_division();
}
void AUTDModulationSetSamplingFrequencyDivision(void* const mod, const uint32_t freq_div) {
  auto* const m = static_cast<autd3::Modulation*>(mod);
  m->sampling_frequency_division() = freq_div;
}
double AUTDModulationSamplingFrequency(const void* const mod) {
  const auto* const m = static_cast<const autd3::Modulation*>(mod);
  return m->sampling_frequency();
}
void AUTDDeleteModulation(const void* const mod) {
  const auto* m = static_cast<const autd3::Modulation*>(mod);
  delete m;
}

void AUTDPointSTM(void** out) { *out = new autd3::PointSTM<T>; }
void AUTDGainSTM(void** out, const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  *out = new autd3::GainSTM<T>(wrapper->geometry());
}
bool AUTDPointSTMAdd(void* const stm, const double x, const double y, const double z, const uint8_t shift) {
  auto* const stm_w = static_cast<autd3::PointSTM<T>*>(stm);
  AUTD3_CAPI_TRY({
    stm_w->add(to_vec3(x, y, z), shift);
    return true;
  })
}
bool AUTDGainSTMAdd(void* const stm, void* const gain) {
  auto* const stm_w = static_cast<autd3::GainSTM<T>*>(stm);
  auto* const g = static_cast<autd3::Gain<T>*>(gain);
  AUTD3_CAPI_TRY({
    stm_w->add(*g);
    return true;
  })
}
uint16_t AUTDGetGainSTMMode(void* const stm) {
  auto* const stm_w = static_cast<autd3::GainSTM<T>*>(stm);
  return static_cast<uint16_t>(stm_w->mode());
}
void AUTDSetGainSTMMode(void* const stm, uint16_t mode) {
  auto* const stm_w = static_cast<autd3::GainSTM<T>*>(stm);
  stm_w->mode() = static_cast<autd3::Mode>(mode);
}
double AUTDSTMSetFrequency(void* const stm, const double freq) {
  auto* const stm_w = static_cast<autd3::core::STM<T>*>(stm);
  return stm_w->set_frequency(freq);
}
double AUTDSTMFrequency(const void* const stm) {
  const auto* const stm_w = static_cast<const autd3::core::STM<T>*>(stm);
  return stm_w->frequency();
}
double AUTDSTMSamplingFrequency(const void* const stm) {
  const auto* const stm_w = static_cast<const autd3::core::STM<T>*>(stm);
  return stm_w->sampling_frequency();
}
uint32_t AUTDSTMSamplingFrequencyDivision(const void* const stm) {
  const auto* const stm_w = static_cast<const autd3::core::STM<T>*>(stm);
  return stm_w->sampling_frequency_division();
}
void AUTDSTMSetSamplingFrequencyDivision(void* const stm, const uint32_t freq_div) {
  auto* const stm_w = static_cast<autd3::core::STM<T>*>(stm);
  stm_w->sampling_frequency_division() = freq_div;
}
void AUTDDeleteSTM(const void* const stm) {
  const auto* const stm_w = static_cast<const autd3::core::STM<T>*>(stm);
  delete stm_w;
}

int32_t AUTDStop(void* const handle) {
  auto* const wrapper = static_cast<Controller*>(handle);
  AUTD3_CAPI_TRY2(return wrapper->stop() ? 1 : 0)
}

void AUTDCreateSilencer(void** out, const uint16_t step, const uint16_t cycle) { *out = new autd3::SilencerConfig(step, cycle); }
void AUTDDeleteSilencer(const void* config) {
  const auto* const config_ = static_cast<const autd3::SilencerConfig*>(config);
  delete config_;
}

int32_t AUTDSendHeader(void* const handle, void* const header) {
  auto* const wrapper = static_cast<Controller*>(handle);
  auto* const h = static_cast<autd3::core::DatagramHeader*>(header);
  AUTD3_CAPI_TRY2(return wrapper->send(*h) ? 1 : 0)
}
int32_t AUTDSendBody(void* const handle, void* const body) {
  auto* const wrapper = static_cast<Controller*>(handle);
  auto* const b = static_cast<autd3::core::DatagramBody<T>*>(body);
  AUTD3_CAPI_TRY2(return wrapper->send(*b) ? 1 : 0)
}
int32_t AUTDSendHeaderBody(void* const handle, void* const header, void* const body) {
  auto* const wrapper = static_cast<Controller*>(handle);
  auto* const h = static_cast<autd3::core::DatagramHeader*>(header);
  auto* const b = static_cast<autd3::core::DatagramBody<T>*>(body);
  AUTD3_CAPI_TRY2(return wrapper->send(*h, *b) ? 1 : 0)
}

void AUTDSetTransFrequency(void* const handle, const int32_t device_idx, const int32_t local_trans_idx, const double frequency) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry()[device_idx][local_trans_idx].set_frequency(frequency);
}
void AUTDSetTransCycle(void* const handle, const int32_t device_idx, const int32_t local_trans_idx, const uint16_t cycle) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry()[device_idx][local_trans_idx].set_cycle(cycle);
}

void AUTDSetModDelay(void* const handle, const int32_t device_idx, const int32_t local_trans_idx, const uint16_t delay) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry()[device_idx][local_trans_idx].mod_delay() = delay;
}
void AUTDCreateModDelayConfig(void** out) { *out = new autd3::ModDelayConfig<T>(); }
void AUTDDeleteModDelayConfig(const void* config) {
  const auto* const config_ = static_cast<const autd3::ModDelayConfig<T>*>(config);
  delete config_;
}

void AUTDCreateAmplitudes(void** out, void* const handle, const double amp) {
  auto* const wrapper = static_cast<Controller*>(handle);
  *out = new autd3::core::Amplitudes<T>(wrapper->geometry(), amp);
}
void AUTDDeleteAmplitudes(IN const void* amplitudes) {
  const auto* const amps_ = static_cast<const autd3::core::Amplitudes<T>*>(amplitudes);
  delete amps_;
}

void AUTDSetMode(const uint8_t mode) { T::mode() = static_cast<autd3::core::TransducerMode>(mode); }
