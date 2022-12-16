// File: driver.cpp
// Project: v2_2
// Created Date: 22/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/driver/v2_2/driver.hpp"

#include "../../spdlog.hpp"
#include "autd3/driver/v2_2/defined.hpp"

namespace autd3::driver {
uint8_t DriverV2_2::version_num() const noexcept { return 0x82; }

void DriverV2_2::clear(TxDatagram& tx) const noexcept {
  tx.header().msg_id = MSG_CLEAR;
  tx.num_bodies = 0;
}

void DriverV2_2::null_header(const uint8_t msg_id, TxDatagram& tx) const noexcept {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SYNC);
  tx.header().size = 0;
}

void DriverV2_2::null_body(TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
  tx.num_bodies = 0;
}

void DriverV2_2::sync(const std::vector<uint16_t>& cycles, TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);
  tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SYNC);
  tx.num_bodies = tx.num_devices();

  std::memcpy(tx.bodies_raw_ptr(), cycles.data(), tx.bodies_size());
}

void DriverV2_2::mod_delay(const std::vector<uint16_t>& delays, TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.set(CPUControlFlags::MOD_DELAY);
  tx.num_bodies = tx.num_devices();

  std::memcpy(tx.bodies_raw_ptr(), delays.data(), tx.bodies_size());
}

bool DriverV2_2::modulation(const uint8_t msg_id, const std::vector<uint8_t>& mod_data, size_t& sent, const uint32_t freq_div, TxDatagram& tx) const {
  if (mod_data.size() > v2_2::MOD_BUF_SIZE_MAX) {
    spdlog::error("Modulation buffer overflow");
    return false;
  }

  const auto is_first_frame = sent == 0;
  const auto max_size = is_first_frame ? MOD_HEADER_INITIAL_DATA_SIZE : MOD_HEADER_SUBSEQUENT_DATA_SIZE;
  const auto mod_size = (std::min)(mod_data.size() - sent, max_size);
  const auto is_last_frame = sent + mod_size == mod_data.size();
  const auto* buf = mod_data.data() + sent;

  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.set(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_END);
  tx.header().size = static_cast<uint8_t>(mod_size);

  if (mod_size == 0) {
    tx.header().cpu_flag.remove(CPUControlFlags::MOD);
    return true;
  }

  if (is_first_frame) {
    if (freq_div < v2_2::MOD_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("Modulation frequency division is out of range. Minimum is {}, but you use {}.", v2_2::MOD_SAMPLING_FREQ_DIV_MIN, freq_div);
      return false;
    }

    tx.header().cpu_flag.set(CPUControlFlags::MOD_BEGIN);
    tx.header().mod_initial().freq_div = freq_div;
    std::memcpy(&tx.header().mod_initial().data[0], buf, mod_size);
  } else {
    std::memcpy(&tx.header().mod_subsequent().data[0], buf, mod_size);
  }

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::MOD_END);

  sent += mod_size;
  return true;
}

bool DriverV2_2::config_silencer(const uint8_t msg_id, const uint16_t cycle, const uint16_t step, TxDatagram& tx) const {
  if (cycle < v2_2::SILENCER_CYCLE_MIN) {
    spdlog::error("Silencer cycle is out of range. Minimum is {}, but you use {}.", v2_2::SILENCER_CYCLE_MIN, cycle);
    return false;
  }

  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SYNC);
  tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SILENCER);

  tx.header().silencer().cycle = cycle;
  tx.header().silencer().step = step;
  return true;
}

void DriverV2_2::normal_legacy_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.num_bodies = 0;
}

void DriverV2_2::normal_legacy_header(TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.num_bodies = 0;
}

void DriverV2_2::normal_legacy_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept {
  auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr());
  for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.num_devices();
}

void DriverV2_2::normal_header(TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.num_bodies = 0;
}

void DriverV2_2::normal_duty_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

  auto* p = reinterpret_cast<Duty*>(tx.bodies_raw_ptr());
  for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.num_devices();
}

void DriverV2_2::normal_phase_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

  auto* p = reinterpret_cast<Phase*>(tx.bodies_raw_ptr());
  for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.num_devices();
}

void DriverV2_2::focus_stm_header(TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_GAIN_MODE);

  tx.num_bodies = 0;
}

size_t DriverV2_2::focus_stm_send_size(const size_t total_size, const size_t sent, const std::vector<size_t>& device_map) const noexcept {
  const size_t tr_num = *std::min_element(device_map.begin(), device_map.end());
  const size_t data_len = tr_num * sizeof(uint16_t);
  const auto max_size = sent == 0 ? (data_len - sizeof(uint16_t) - sizeof(uint32_t) - sizeof(uint32_t)) / sizeof(STMFocus)
                                  : (data_len - sizeof(uint16_t)) / sizeof(STMFocus);
  return (std::min)(total_size - sent, max_size);
}

bool DriverV2_2::focus_stm_body(const std::vector<std::vector<STMFocus>>& points, size_t& sent, const size_t total_size, const uint32_t freq_div,
                                const double sound_speed, std::optional<uint16_t>, std::optional<uint16_t>, TxDatagram& tx) const {
  if (total_size > v2_2::FOCUS_STM_BUF_SIZE_MAX) {
    spdlog::error("FocusSTM out of buffer");
    return false;
  }

  if (points.empty() || points[0].empty()) return true;

  if (sent == 0) {
    if (freq_div < v2_2::FOCUS_STM_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("STM frequency division is out of range. Minimum is {}, but you use {}.", v2_2::FOCUS_STM_SAMPLING_FREQ_DIV_MIN, freq_div);
      return false;
    }

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
#ifdef AUTD3_USE_METER
    const auto sound_speed_internal = static_cast<uint32_t>(std::round(sound_speed * 1024.0));
#else
    const auto sound_speed_internal = static_cast<uint32_t>(std::round(sound_speed / 1e3 * 1024.0));
#endif
    for (size_t i = 0; i < tx.num_devices(); i++) {
      auto& d = tx.body(i);
      const auto& s = points.at(i);
      d.focus_stm_initial().set_size(static_cast<uint16_t>(s.size()));
      d.focus_stm_initial().set_freq_div(freq_div);
      d.focus_stm_initial().set_sound_speed(sound_speed_internal);
      d.focus_stm_initial().set_point(s);
    }
  } else {
    for (size_t i = 0; i < tx.num_devices(); i++) {
      auto& d = tx.body(i);
      const auto& s = points.at(i);
      d.focus_stm_subsequent().set_size(static_cast<uint16_t>(s.size()));
      d.focus_stm_subsequent().set_point(s);
    }
  }

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  const auto send_size = points[0].size();
  if (sent + send_size == total_size) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.num_bodies = tx.num_devices();

  sent += send_size;
  return true;
}

void DriverV2_2::gain_stm_legacy_header(TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  tx.num_bodies = 0;
}

bool DriverV2_2::gain_stm_legacy_body(const std::vector<std::vector<Drive>>& drives, size_t& sent, const uint32_t freq_div, const GainSTMMode mode,
                                      std::optional<uint16_t>, std::optional<uint16_t>, TxDatagram& tx) const {
  if (drives.size() > v2_2::GAIN_STM_LEGACY_BUF_SIZE_MAX) {
    spdlog::error("GainSTM out of buffer");
    return false;
  }

  bool is_last_frame = false;
  if (sent == 0) {
    if (freq_div < v2_2::GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("STM frequency division is out of range. Minimum is {}, but you use {}.", v2_2::GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN, freq_div);
      return false;
    }

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.num_devices(); i++) {
      tx.body(i).gain_stm_initial().set_freq_div(freq_div);
      tx.body(i).gain_stm_initial().set_mode(mode);
      tx.body(i).gain_stm_initial().set_cycle(drives.size());
    }
    sent++;
  } else {
    switch (mode) {
      case GainSTMMode::PhaseDutyFull:
        is_last_frame = sent + 1 >= drives.size() + 1;
        {
          auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(drives[sent - 1][i]);
        }
        sent++;
        break;
      case GainSTMMode::PhaseFull:
        is_last_frame = sent + 2 >= drives.size() + 1;
        {
          auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(0, drives[sent - 1][i]);
        }
        sent++;
        if (sent - 1 < drives.size()) {
          auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(1, drives[sent - 1][i]);
          sent++;
        }
        break;
      case GainSTMMode::PhaseHalf:
        is_last_frame = sent + 4 >= drives.size() + 1;
        {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(0, drives[sent - 1][i]);
        }
        sent++;
        if (sent - 1 < drives.size()) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(1, drives[sent - 1][i]);
          sent++;
        }
        if (sent - 1 < drives.size()) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(2, drives[sent - 1][i]);
          sent++;
        }
        if (sent - 1 < drives.size()) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(3, drives[sent - 1][i]);
          sent++;
        }
        break;
    }
  }

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.num_bodies = tx.num_devices();
  return true;
}

void DriverV2_2::gain_stm_normal_header(TxDatagram& tx) const noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  tx.num_bodies = 0;
}

bool DriverV2_2::gain_stm_normal_phase(const std::vector<std::vector<Drive>>& drives, const size_t sent, const uint32_t freq_div,
                                       const GainSTMMode mode, std::optional<uint16_t>, std::optional<uint16_t>, TxDatagram& tx) const {
  if (drives.size() > v2_2::GAIN_STM_BUF_SIZE_MAX) {
    spdlog::error("GainSTM out of buffer");
    return false;
  }

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26813)
#endif
  if (mode == GainSTMMode::PhaseHalf) {
    spdlog::error("PhaseHalf is not supported in normal mode");
    return false;
  }
#ifdef _MSC_VER
#pragma warning(pop)
#endif

  tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

  if (sent == 0) {
    if (freq_div < v2_2::GAIN_STM_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("STM frequency division is out of range. Minimum is {}, but you use {}.", v2_2::GAIN_STM_SAMPLING_FREQ_DIV_MIN, freq_div);
      return false;
    }
    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.num_devices(); i++) {
      tx.body(i).gain_stm_initial().set_freq_div(freq_div);
      tx.body(i).gain_stm_initial().set_mode(mode);
    }
  } else {
    auto* p = reinterpret_cast<Phase*>(tx.bodies_raw_ptr());
    for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(drives[sent - 1][i]);
  }

  if (sent + 1 == drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.num_devices();
  return true;
}

bool DriverV2_2::gain_stm_normal_duty(const std::vector<std::vector<Drive>>& drives, const size_t sent, const uint32_t freq_div,
                                      const GainSTMMode mode, std::optional<uint16_t>, std::optional<uint16_t>, TxDatagram& tx) const {
  if (drives.size() > v2_2::GAIN_STM_BUF_SIZE_MAX) {
    spdlog::error("GainSTM out of buffer");
    return false;
  }

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26813)
#endif
  if (mode == GainSTMMode::PhaseHalf) {
    spdlog::error("PhaseHalf is not supported in normal mode");
    return false;
  }
#ifdef _MSC_VER
#pragma warning(pop)
#endif

  tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

  if (sent == 0) {
    if (freq_div < v2_2::GAIN_STM_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("STM frequency division is out of range. Minimum is {}, but you use {}.", v2_2::GAIN_STM_SAMPLING_FREQ_DIV_MIN, freq_div);
      return false;
    }
    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.num_devices(); i++) {
      tx.body(i).gain_stm_initial().set_freq_div(freq_div);
      tx.body(i).gain_stm_initial().set_mode(mode);
    }
  } else {
    auto* p = reinterpret_cast<Duty*>(tx.bodies_raw_ptr());
    for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(drives[sent - 1][i]);
  }

  if (sent + 1 == drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.num_devices();

  return true;
}

void DriverV2_2::force_fan(TxDatagram& tx, const bool value) const noexcept {
  if (value)
    tx.header().fpga_flag.set(FPGAControlFlags::FORCE_FAN);
  else
    tx.header().fpga_flag.remove(FPGAControlFlags::FORCE_FAN);
}

void DriverV2_2::reads_fpga_info(TxDatagram& tx, const bool value) const noexcept {
  if (value)
    tx.header().fpga_flag.set(FPGAControlFlags::READS_FPGA_INFO);
  else
    tx.header().fpga_flag.remove(FPGAControlFlags::READS_FPGA_INFO);
}

void DriverV2_2::cpu_version(TxDatagram& tx) const noexcept {
  tx.header().msg_id = MSG_RD_CPU_VERSION;
  tx.header().cpu_flag = static_cast<CPUControlFlags::VALUE>(MSG_RD_CPU_VERSION);  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

void DriverV2_2::fpga_version(TxDatagram& tx) const noexcept {
  tx.header().msg_id = MSG_RD_FPGA_VERSION;
  tx.header().cpu_flag = static_cast<CPUControlFlags::VALUE>(MSG_RD_FPGA_VERSION);  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

void DriverV2_2::fpga_functions(TxDatagram& tx) const noexcept {
  tx.header().msg_id = MSG_RD_FPGA_FUNCTION;
  tx.header().cpu_flag = static_cast<CPUControlFlags::VALUE>(MSG_RD_FPGA_FUNCTION);  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

}  // namespace autd3::driver
