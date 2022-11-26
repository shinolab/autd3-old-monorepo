// File: c_api.cpp
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <algorithm>
#include <cstdint>
#include <cstring>
#include <string>
#include <utility>

#include "../../src/spdlog.hpp"
#include "./autd3_c_api.h"
#include "autd3.hpp"
#include "autd3/driver/v2_2/driver.hpp"
#include "autd3/driver/v2_3/driver.hpp"
#include "autd3/driver/v2_4/driver.hpp"
#include "autd3/driver/v2_5/driver.hpp"
#include "autd3/driver/v2_6/driver.hpp"
#include "autd3/modulation/lpf.hpp"
#include "custom.hpp"
#include "custom_sink.hpp"
#include "wrapper.hpp"
#include "wrapper_link.hpp"

using Controller = autd3::Controller;

autd3::Vector3 to_vec3(const double x, const double y, const double z) { return {x, y, z}; }
autd3::Quaternion to_quaternion(const double w, const double x, const double y, const double z) { return {w, x, y, z}; }

std::unique_ptr<const autd3::driver::Driver> get_driver(const uint8_t driver_version) {
  switch (driver_version) {
    case 0x00:
      return std::make_unique<autd3::DriverLatest>();
    case 0x82:
      return std::make_unique<autd3::driver::DriverV2_2>();
    case 0x83:
      return std::make_unique<autd3::driver::DriverV2_3>();
    case 0x84:
      return std::make_unique<autd3::driver::DriverV2_4>();
    case 0x85:
      return std::make_unique<autd3::driver::DriverV2_5>();
    case 0x86:
      return std::make_unique<autd3::driver::DriverV2_6>();
    default:
      spdlog::error("unknown driver version: {}", driver_version);
      return nullptr;
  }
}

void AUTDSetLogLevel(const int32_t level) { spdlog::set_level(static_cast<spdlog::level::level_enum>(level)); }

void AUTDSetDefaultLogger(void* out, void* flush) {
  auto custom_sink = std::make_shared<autd3::capi::custom_sink_mt>(out, flush);
  const auto logger = std::make_shared<spdlog::logger>("AUTD3 Logger", custom_sink);
  set_default_logger(logger);
}

bool AUTDCreateController(void** out, const uint8_t driver_version) {
  auto driver = get_driver(driver_version);
  if (driver == nullptr) return false;
  *out = new Controller(std::move(driver));
  return true;
}

bool AUTDOpenController(void* const handle, void* const link) {
  auto* const wrapper = static_cast<Controller*>(handle);
  auto* w_link = static_cast<LinkWrapper*>(link);
  autd3::LinkPtr link_ = std::move(w_link->ptr);
  link_delete(w_link);
  return wrapper->open(std::move(link_));
}

void AUTDAddDevice(void* const handle, const double x, const double y, const double z, const double rz1, const double ry, const double rz2) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry().add_device(autd3::AUTD3(to_vec3(x, y, z), to_vec3(rz1, ry, rz2)));
}

void AUTDAddDeviceQuaternion(void* const handle, const double x, const double y, const double z, const double qw, const double qx, const double qy,
                             const double qz) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry().add_device(autd3::AUTD3(to_vec3(x, y, z), to_quaternion(qw, qx, qy, qz)));
}

bool AUTDClose(void* const handle) {
  auto* const wrapper = static_cast<Controller*>(handle);
  return wrapper->close();
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
uint64_t AUTDGetAckCheckTimeout(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(wrapper->get_ack_check_timeout()).count());
}
uint64_t AUTDGetSendInterval(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(wrapper->get_send_interval()).count());
}
void AUTDSetForceFan(void* const handle, const bool force) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->force_fan = force;
}
void AUTDSetReadsFPGAInfo(void* const handle, const bool reads_fpga_info) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->reads_fpga_info = reads_fpga_info;
}
void AUTDSetAckCheckTimeout(void* const handle, const uint64_t timeout) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->set_ack_check_timeout(std::chrono::nanoseconds(timeout));
}
void AUTDSetSendInterval(void* const handle, const uint64_t interval) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->set_send_interval(std::chrono::nanoseconds(interval));
}
double AUTDGetTransFrequency(const void* const handle, const int32_t trans_idx) {
  const auto* const wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry()[trans_idx].frequency();
}

uint16_t AUTDGetTransCycle(const void* const handle, const int32_t trans_idx) {
  const auto* const wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry()[trans_idx].cycle();
}

double AUTDGetSoundSpeed(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->get_sound_speed();
}

void AUTDSetSoundSpeed(void* const handle, const double sound_speed) {
  auto* wrapper = static_cast<Controller*>(handle);
  wrapper->set_sound_speed(sound_speed);
}

double AUTDGetWavelength(const void* const handle, const int32_t trans_idx) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry()[trans_idx].wavelength();
}

double AUTDGetAttenuation(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  return wrapper->get_attenuation();
}

void AUTDSetAttenuation(void* const handle, const double attenuation) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->set_attenuation(attenuation);
}

bool AUTDGetFPGAInfo(void* const handle, uint8_t* out) {
  auto* const wrapper = static_cast<Controller*>(handle);
  const auto& res = wrapper->read_fpga_info();
  std::memcpy(out, res.data(), res.size());
  return !res.empty();
}

int32_t AUTDNumDevices(const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto res = wrapper->geometry().num_devices();
  return static_cast<int32_t>(res);
}

void AUTDTransPosition(const void* const handle, const int32_t trans_idx, double* x, double* y, double* z) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto& pos = wrapper->geometry()[trans_idx].position();
  *x = pos.x();
  *y = pos.y();
  *z = pos.z();
}

void AUTDTransXDirection(const void* const handle, const int32_t trans_idx, double* x, double* y, double* z) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto& pos = wrapper->geometry()[trans_idx].x_direction();
  *x = pos.x();
  *y = pos.y();
  *z = pos.z();
}
void AUTDTransYDirection(const void* const handle, const int32_t trans_idx, double* x, double* y, double* z) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto& pos = wrapper->geometry()[trans_idx].y_direction();
  *x = pos.x();
  *y = pos.y();
  *z = pos.z();
}
void AUTDTransZDirection(const void* const handle, const int32_t trans_idx, double* x, double* y, double* z) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  const auto& pos = wrapper->geometry()[trans_idx].z_direction();
  *x = pos.x();
  *y = pos.y();
  *z = pos.z();
}

int32_t AUTDGetFirmwareInfoListPointer(void* const handle, void** out) {
  auto* const wrapper = static_cast<Controller*>(handle);
  const auto res = wrapper->firmware_infos();
  if (res.empty()) return 0;
  auto* list = firmware_info_list_create(res);
  *out = list;
  return res.size();
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
  auto* g = new autd3::gain::Null;
  *gain = g;
}

void AUTDGainGrouped(void** gain, const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  auto* g = new autd3::gain::Grouped(wrapper->geometry());
  *gain = g;
}

void AUTDGainGroupedAdd(void* grouped_gain, const int32_t device_id, void* gain) {
  auto* const gg = static_cast<autd3::gain::Grouped*>(grouped_gain);
  auto* const g = static_cast<autd3::Gain*>(gain);
  gg->add(device_id, *g);
}

void AUTDGainFocus(void** gain, const double x, const double y, const double z, const double amp) {
  *gain = new autd3::gain::Focus(to_vec3(x, y, z), amp);
}
void AUTDGainBesselBeam(void** gain, const double x, const double y, const double z, const double n_x, const double n_y, const double n_z,
                        const double theta_z, const double amp) {
  *gain = new autd3::gain::BesselBeam(to_vec3(x, y, z), to_vec3(n_x, n_y, n_z), theta_z, amp);
}
void AUTDGainPlaneWave(void** gain, const double n_x, const double n_y, const double n_z, const double amp) {
  *gain = new autd3::gain::PlaneWave(to_vec3(n_x, n_y, n_z), amp);
}

void AUTDGainTransducerTest(void** gain) { *gain = new autd3::gain::TransducerTest(); }

void AUTDGainTransducerTestSet(void* gain, const int32_t tr_idx, const double amp, const double phase) {
  auto* const g = static_cast<autd3::gain::TransducerTest*>(gain);
  g->set(tr_idx, amp, phase);
}

void AUTDGainCustom(void** gain, const double* amp, const double* phase, const uint64_t size) {
  *gain = new CustomGain(amp, phase, static_cast<size_t>(size));
}

void AUTDDeleteGain(const void* const gain) {
  const auto* g = static_cast<const autd3::Gain*>(gain);
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

void AUTDModulationLPF(void** mod, void* mod_in) {
  auto* m = static_cast<autd3::Modulation*>(mod_in);
  *mod = new autd3::modulation::LPF(*m);
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

void AUTDPointSTM(void** out, const double sound_speed) { *out = new autd3::PointSTM(sound_speed); }
void AUTDGainSTM(void** out, const void* const handle) {
  const auto* wrapper = static_cast<const Controller*>(handle);
  *out = new autd3::GainSTM(wrapper->geometry());
}
void AUTDPointSTMAdd(void* const stm, const double x, const double y, const double z, const uint8_t shift) {
  auto* const stm_w = static_cast<autd3::PointSTM*>(stm);
  stm_w->add(to_vec3(x, y, z), shift);
}
void AUTDGainSTMAdd(void* const stm, void* const gain) {
  auto* const stm_w = static_cast<autd3::GainSTM*>(stm);
  auto* const g = static_cast<autd3::Gain*>(gain);
  stm_w->add(*g);
}
uint16_t AUTDGetGainSTMMode(void* const stm) {
  auto* const stm_w = static_cast<autd3::GainSTM*>(stm);
  return static_cast<uint16_t>(stm_w->mode());
}
void AUTDSetGainSTMMode(void* const stm, uint16_t mode) {
  auto* const stm_w = static_cast<autd3::GainSTM*>(stm);
  stm_w->mode() = static_cast<autd3::GainSTMMode>(mode);
}
double AUTDSTMSetFrequency(void* const stm, const double freq) {
  auto* const stm_w = static_cast<autd3::core::STM*>(stm);
  return stm_w->set_frequency(freq);
}
double AUTDSTMFrequency(const void* const stm) {
  const auto* const stm_w = static_cast<const autd3::core::STM*>(stm);
  return stm_w->frequency();
}
double AUTDSTMSamplingFrequency(const void* const stm) {
  const auto* const stm_w = static_cast<const autd3::core::STM*>(stm);
  return stm_w->sampling_frequency();
}
uint32_t AUTDSTMSamplingFrequencyDivision(const void* const stm) {
  const auto* const stm_w = static_cast<const autd3::core::STM*>(stm);
  return stm_w->sampling_frequency_division();
}
void AUTDSTMSetSamplingFrequencyDivision(void* const stm, const uint32_t freq_div) {
  auto* const stm_w = static_cast<autd3::core::STM*>(stm);
  stm_w->sampling_frequency_division() = freq_div;
}
void AUTDDeleteSTM(const void* const stm) {
  const auto* const stm_w = static_cast<const autd3::core::STM*>(stm);
  delete stm_w;
}

void AUTDSynchronize(void** out) { *out = new autd3::Synchronize; }

void AUTDClear(void** out) { *out = new autd3::Clear; }

void AUTDUpdateFlags(void** out) { *out = new autd3::UpdateFlag; }

void AUTDStop(void** out) { *out = new autd3::Stop; }

void AUTDModDelayConfig(void** out) { *out = new autd3::ModDelayConfig; }

void AUTDDeleteSpecialData(const void* const data) {
  const auto* const d = static_cast<const autd3::SpecialData*>(data);
  delete d;
}

void AUTDCreateSilencer(void** out, const uint16_t step, const uint16_t cycle) { *out = new autd3::SilencerConfig(step, cycle); }
void AUTDDeleteSilencer(const void* config) {
  const auto* const config_ = static_cast<const autd3::SilencerConfig*>(config);
  delete config_;
}

bool AUTDSend(void* const handle, void* const header, void* const body) {
  if (header == nullptr && body == nullptr) return false;
  auto* const wrapper = static_cast<Controller*>(handle);
  auto* const h = static_cast<autd3::core::DatagramHeader*>(header);
  auto* const b = static_cast<autd3::core::DatagramBody*>(body);
  if (header == nullptr) return wrapper->send(*b);
  if (body == nullptr) return wrapper->send(*h);
  return wrapper->send(*h, *b);
}

bool AUTDSendSpecial(void* const handle, void* const special) {
  auto* const wrapper = static_cast<Controller*>(handle);
  auto* const s = static_cast<autd3::SpecialData*>(special);
  return wrapper->send(s);
}

void AUTDSendAsync(void* const handle, void* header, void* body) {
  if (header == nullptr && body == nullptr) return;
  auto* const wrapper = static_cast<Controller*>(handle);
  auto h = std::unique_ptr<autd3::core::DatagramHeader>(static_cast<autd3::core::DatagramHeader*>(header));
  auto b = std::unique_ptr<autd3::core::DatagramBody>(static_cast<autd3::core::DatagramBody*>(body));
  if (header == nullptr) return wrapper->send_async(std::make_unique<autd3::core::NullHeader>(), std::move(b));
  if (body == nullptr) return wrapper->send_async(std::move(h), std::make_unique<autd3::core::NullBody>());
  wrapper->send_async(std::move(h), std::move(b));
}

void AUTDSendSpecialAsync(void* const handle, void* const special) {
  auto* const wrapper = static_cast<Controller*>(handle);
  auto* const s = static_cast<autd3::SpecialData*>(special);
  wrapper->send_async(s);
  delete s;
}

void AUTDSetTransFrequency(void* const handle, const int32_t trans_idx, const double frequency) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry()[trans_idx].set_frequency(frequency);
}

void AUTDSetTransCycle(void* const handle, const int32_t trans_idx, const uint16_t cycle) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry()[trans_idx].set_cycle(cycle);
}

uint16_t AUTDGetModDelay(const void* const handle, const int32_t trans_idx) {
  const auto* const wrapper = static_cast<const Controller*>(handle);
  return wrapper->geometry()[trans_idx].mod_delay();
}

void AUTDSetModDelay(void* const handle, const int32_t trans_idx, const uint16_t delay) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry()[trans_idx].mod_delay() = delay;
}

void AUTDCreateAmplitudes(void** out, const double amp) { *out = new autd3::core::Amplitudes(amp); }
void AUTDDeleteAmplitudes(IN const void* amplitudes) {
  const auto* const amps_ = static_cast<const autd3::core::Amplitudes*>(amplitudes);
  delete amps_;
}

void AUTDSetMode(void* const handle, const uint8_t mode) {
  auto* const wrapper = static_cast<Controller*>(handle);
  switch (mode) {
    case 0:
      *wrapper << autd3::legacy_mode;
      break;
    case 1:
      *wrapper << autd3::normal_mode;
      break;
    case 2:
      *wrapper << autd3::normal_phase_mode;
      break;
    default:
      break;
  }
}

void AUTDSoftwareSTM(void** out) { *out = new autd3::SoftwareSTM; }
void AUTDSoftwareSTMSetStrategy(void* stm, const uint8_t strategy) {
  static_cast<autd3::SoftwareSTM*>(stm)->timer_strategy =
      autd3::SoftwareSTM::TimerStrategy(static_cast<autd3::SoftwareSTM::TimerStrategy::VALUE>(strategy));
}
EXPORT_AUTD void AUTDSoftwareSTMAdd(void* stm, void* gain) {
  static_cast<autd3::SoftwareSTM*>(stm)->add(std::shared_ptr<autd3::core::Gain>(static_cast<autd3::core::Gain*>(gain)));
}
EXPORT_AUTD void AUTDSoftwareSTMStart(void** handle, void* stm, void* cnt) {
  *handle = new autd3::SoftwareSTM::SoftwareSTMThreadHandle(static_cast<autd3::SoftwareSTM*>(stm)->start(*static_cast<Controller*>(cnt)));
}
EXPORT_AUTD void AUTDSoftwareSTMFinish(void* handle) {
  auto* h = static_cast<autd3::SoftwareSTM::SoftwareSTMThreadHandle*>(handle);
  h->finish();
}
EXPORT_AUTD double AUTDSoftwareSTMSetFrequency(void* stm, const double freq) { return static_cast<autd3::SoftwareSTM*>(stm)->set_frequency(freq); }
EXPORT_AUTD double AUTDSoftwareSTMFrequency(const void* stm) { return static_cast<const autd3::SoftwareSTM*>(stm)->frequency(); }
EXPORT_AUTD uint64_t AUTDSoftwareSTMPeriod(const void* stm) { return static_cast<const autd3::SoftwareSTM*>(stm)->period(); }
EXPORT_AUTD double AUTDSoftwareSTMSamplingFrequency(const void* stm) { return static_cast<const autd3::SoftwareSTM*>(stm)->sampling_frequency(); }
EXPORT_AUTD uint64_t AUTDSoftwareSTMSamplingPeriod(const void* stm) { return static_cast<const autd3::SoftwareSTM*>(stm)->sampling_period_ns(); }
EXPORT_AUTD void AUTDSoftwareSTMSetSamplingPeriod(void* stm, const uint64_t period) {
  static_cast<autd3::SoftwareSTM*>(stm)->sampling_period_ns() = period;
}
EXPORT_AUTD void AUTDDeleteSoftwareSTM(const void* stm) {
  const auto* const stm_ = static_cast<const autd3::SoftwareSTM*>(stm);
  delete stm_;
}
