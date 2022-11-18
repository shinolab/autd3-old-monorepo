// File: driver.hpp
// Project: v2_6
// Created Date: 15/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/common/fpga/defined.hpp"
#include "autd3/driver/driver.hpp"
#include "autd3/driver/v2_6/defined.hpp"
#include "autd3/spdlog.hpp"

namespace autd3::driver {

class DriverV2_6 final : public Driver {
 public:
  ~DriverV2_6() override = default;

  uint8_t version_num() const noexcept override { return 0x86; }

  void clear(TxDatagram& tx) const noexcept override {
    tx.header().msg_id = MSG_CLEAR;
    tx.num_bodies = 0;
  }

  void null_header(const uint8_t msg_id, TxDatagram& tx) const noexcept override {
    tx.header().msg_id = msg_id;
    tx.header().cpu_flag.remove(CPUControlFlags::MOD);
    tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);
    tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SYNC);
    tx.header().size = 0;
  }

  void null_body(TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
    tx.num_bodies = 0;
  }

  void sync(const uint16_t* const cycles, TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.remove(CPUControlFlags::MOD);
    tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);
    tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SYNC);

    std::memcpy(reinterpret_cast<uint16_t*>(tx.bodies()), cycles, sizeof(Body) * tx.size());

    tx.num_bodies = tx.size();
  }

  void mod_delay(const uint16_t* const delays, TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);
    tx.header().cpu_flag.set(CPUControlFlags::MOD_DELAY);

    std::memcpy(reinterpret_cast<uint16_t*>(tx.bodies()), delays, sizeof(Body) * tx.size());

    tx.num_bodies = tx.size();
  }

  bool modulation(const uint8_t msg_id, const std::vector<uint8_t>& mod_data, size_t& sent, const uint32_t freq_div, TxDatagram& tx) const override {
    if (mod_data.size() > v2_6::MOD_BUF_SIZE_MAX) {
      spdlog::error("Modulation buffer overflow");
      return false;
    }

    const auto is_first_frame = sent == 0;
    const auto max_size = is_first_frame ? driver::MOD_HEAD_DATA_SIZE : driver::MOD_BODY_DATA_SIZE;
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
      if (freq_div < v2_6::MOD_SAMPLING_FREQ_DIV_MIN) {
        spdlog::error("Modulation frequency division is oud of range. Minimum is {}, but you use {}.", v2_6::MOD_SAMPLING_FREQ_DIV_MIN, freq_div);
        return false;
      }

      tx.header().cpu_flag.set(CPUControlFlags::MOD_BEGIN);
      tx.header().mod_head().freq_div = freq_div;
      std::memcpy(&tx.header().mod_head().data[0], buf, mod_size);
    } else {
      std::memcpy(&tx.header().mod_body().data[0], buf, mod_size);
    }

    if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::MOD_END);

    sent += mod_size;
    return true;
  }

  bool config_silencer(const uint8_t msg_id, const uint16_t cycle, const uint16_t step, TxDatagram& tx) const override {
    if (cycle < v2_6::SILENCER_CYCLE_MIN) {
      spdlog::error("Silencer cycle is oud of range. Minimum is {}, but you use {}.", v2_6::SILENCER_CYCLE_MIN, cycle);
      return false;
    }

    tx.header().msg_id = msg_id;
    tx.header().cpu_flag.remove(CPUControlFlags::MOD);
    tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SYNC);
    tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SILENCER);

    tx.header().silencer_header().cycle = cycle;
    tx.header().silencer_header().step = step;
    return true;
  }

  void normal_legacy_header(TxDatagram& tx) noexcept {
    tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

    tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
    tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

    tx.num_bodies = 0;
  }

  void normal_legacy_header(TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

    tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
    tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

    tx.num_bodies = 0;
  }

  void normal_legacy_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override {
    auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies());
    for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

    tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

    tx.num_bodies = tx.size();
  }

  void normal_header(TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

    tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
    tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

    tx.num_bodies = 0;
  }

  void normal_duty_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

    auto* p = reinterpret_cast<Duty*>(tx.bodies());
    for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

    tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

    tx.num_bodies = tx.size();
  }

  void normal_phase_body(const std::vector<Drive>& drives, TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

    auto* p = reinterpret_cast<Phase*>(tx.bodies());
    for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

    tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

    tx.num_bodies = tx.size();
  }

  void point_stm_header(TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
    tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
    tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

    tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
    tx.header().fpga_flag.remove(FPGAControlFlags::STM_GAIN_MODE);

    tx.num_bodies = 0;
  }

  size_t point_stm_send_size(const size_t total_size, const size_t sent) const noexcept override {
    const auto max_size = sent == 0 ? driver::POINT_STM_HEAD_DATA_SIZE : driver::POINT_STM_BODY_DATA_SIZE;
    return (std::min)(total_size - sent, max_size);
  }

  bool point_stm_body(const std::vector<std::vector<STMFocus>>& points, size_t& sent, const size_t total_size, const uint32_t freq_div,
                      const double sound_speed, TxDatagram& tx) const override {
    if (total_size > v2_6::POINT_STM_BUF_SIZE_MAX) {
      spdlog::error("PointSTM out of buffer");
      return false;
    }

    if (points.empty() || points[0].empty()) return true;

    if (sent == 0) {
      if (freq_div < v2_6::POINT_STM_SAMPLING_FREQ_DIV_MIN) {
        spdlog::error("STM frequency division is oud of range. Minimum is {}, but you use {}.", v2_6::POINT_STM_SAMPLING_FREQ_DIV_MIN, freq_div);
        return false;
      }
      tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
#ifdef AUTD3_USE_METER
      const auto sound_speed_internal = static_cast<uint32_t>(std::round(sound_speed * 1024.0));
#else
      const auto sound_speed_internal = static_cast<uint32_t>(std::round(sound_speed / 1e3 * 1024.0));
#endif
      for (size_t i = 0; i < tx.size(); i++) {
        auto& d = tx.bodies()[i];
        const auto& s = points.at(i);
        d.point_stm_head().set_size(static_cast<uint16_t>(s.size()));
        d.point_stm_head().set_freq_div(freq_div);
        d.point_stm_head().set_sound_speed(sound_speed_internal);
        d.point_stm_head().set_point(s);
      }
    } else {
      for (size_t i = 0; i < tx.size(); i++) {
        auto& d = tx.bodies()[i];
        const auto& s = points.at(i);
        d.point_stm_body().set_size(static_cast<uint16_t>(s.size()));
        d.point_stm_body().set_point(s);
      }
    }

    tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

    const auto send_size = points[0].size();
    if (sent + send_size == total_size) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

    tx.num_bodies = tx.size();

    sent += send_size;
    return true;
  }

  void gain_stm_legacy_header(TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
    tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
    tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

    tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
    tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
    tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

    tx.num_bodies = 0;
  }

  bool gain_stm_legacy_body(const std::vector<std::vector<driver::Drive>>& drives, size_t& sent, const uint32_t freq_div, const GainSTMMode mode,
                            TxDatagram& tx) const override {
    if (drives.size() > v2_6::GAIN_STM_LEGACY_BUF_SIZE_MAX) {
      spdlog::error("GainSTM out of buffer");
      return false;
    }

    bool is_last_frame = false;
    if (sent == 0) {
      if (freq_div < v2_6::GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) {
        spdlog::error("STM frequency division is oud of range. Minimum is {}, but you use {}.", v2_6::GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN,
                      freq_div);
        return false;
      }

      tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
      for (size_t i = 0; i < tx.size(); i++) {
        tx.bodies()[i].gain_stm_head().set_freq_div(freq_div);
        tx.bodies()[i].gain_stm_head().set_mode(mode);
        tx.bodies()[i].gain_stm_head().set_cycle(drives.size());
      }
      sent++;
    } else {
      switch (mode) {
        case GainSTMMode::PhaseDutyFull:
          is_last_frame = sent + 1 >= drives.size() + 1;
          {
            auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies());
            for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(drives[sent - 1][i]);
          }
          sent++;
          break;
        case GainSTMMode::PhaseFull:
          is_last_frame = sent + 2 >= drives.size() + 1;
          {
            auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies());
            for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(0, drives[sent - 1][i]);
          }
          sent++;
          if (sent - 1 < drives.size()) {
            auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies());
            for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(1, drives[sent - 1][i]);
            sent++;
          }
          break;
        case GainSTMMode::PhaseHalf:
          is_last_frame = sent + 4 >= drives.size() + 1;
          {
            auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies());
            for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(0, drives[sent - 1][i]);
          }
          sent++;
          if (sent - 1 < drives.size()) {
            auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies());
            for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(1, drives[sent - 1][i]);
            sent++;
          }
          if (sent - 1 < drives.size()) {
            auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies());
            for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(2, drives[sent - 1][i]);
            sent++;
          }
          if (sent - 1 < drives.size()) {
            auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies());
            for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(3, drives[sent - 1][i]);
            sent++;
          }
          break;
        default:
          spdlog::error("Unknown Gain STM Mode: {}", static_cast<int>(mode));
          return false;
      }
    }

    tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

    if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

    tx.num_bodies = tx.size();
    return true;
  }

  void gain_stm_normal_header(TxDatagram& tx) const noexcept override {
    tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
    tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
    tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
    tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

    tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
    tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
    tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

    tx.num_bodies = 0;
  }

  bool gain_stm_normal_phase(const std::vector<std::vector<driver::Drive>>& drives, const size_t sent, const uint32_t freq_div,
                             const GainSTMMode mode, TxDatagram& tx) const override {
    if (drives.size() > v2_6::GAIN_STM_BUF_SIZE_MAX) {
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
      if (freq_div < v2_6::GAIN_STM_SAMPLING_FREQ_DIV_MIN) {
        spdlog::error("STM frequency division is oud of range. Minimum is {}, but you use {}.", v2_6::GAIN_STM_SAMPLING_FREQ_DIV_MIN, freq_div);
        return false;
      }
      tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
      for (size_t i = 0; i < tx.size(); i++) {
        tx.bodies()[i].gain_stm_head().set_freq_div(freq_div);
        tx.bodies()[i].gain_stm_head().set_mode(mode);
        tx.bodies()[i].gain_stm_head().set_cycle(drives.size());
      }
    } else {
      auto* p = reinterpret_cast<Phase*>(tx.bodies());
      for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(drives[sent - 1][i]);
    }

    if (sent + 1 == drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

    tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

    tx.num_bodies = tx.size();
    return true;
  }

  bool gain_stm_normal_duty(const std::vector<std::vector<driver::Drive>>& drives, const size_t sent, const uint32_t freq_div, const GainSTMMode mode,
                            TxDatagram& tx) const override {
    if (drives.size() > v2_6::GAIN_STM_BUF_SIZE_MAX) {
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
      if (freq_div < v2_6::GAIN_STM_SAMPLING_FREQ_DIV_MIN) {
        spdlog::error("STM frequency division is oud of range. Minimum is {}, but you use {}.", v2_6::GAIN_STM_SAMPLING_FREQ_DIV_MIN, freq_div);
        return false;
      }
      tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
      for (size_t i = 0; i < tx.size(); i++) {
        tx.bodies()[i].gain_stm_head().set_freq_div(freq_div);
        tx.bodies()[i].gain_stm_head().set_mode(mode);
        tx.bodies()[i].gain_stm_head().set_cycle(drives.size());
      }
    } else {
      auto* p = reinterpret_cast<Duty*>(tx.bodies());
      for (size_t i = 0; i < drives[sent - 1].size(); i++) p[i].set(drives[sent - 1][i]);
    }

    if (sent + 1 == drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

    tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

    tx.num_bodies = tx.size();
    return true;
  }

  void force_fan(TxDatagram& tx, const bool value) const noexcept override {
    if (value)
      tx.header().fpga_flag.set(FPGAControlFlags::FORCE_FAN);
    else
      tx.header().fpga_flag.remove(FPGAControlFlags::FORCE_FAN);
  }

  void reads_fpga_info(TxDatagram& tx, const bool value) const noexcept override {
    if (value)
      tx.header().fpga_flag.set(FPGAControlFlags::READS_FPGA_INFO);
    else
      tx.header().fpga_flag.remove(FPGAControlFlags::READS_FPGA_INFO);
  }

  void cpu_version(TxDatagram& tx) const noexcept override {
    tx.header().msg_id = MSG_RD_CPU_VERSION;
    tx.header().cpu_flag = (CPUControlFlags::VALUE)(MSG_RD_CPU_VERSION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }

  void fpga_version(TxDatagram& tx) const noexcept override {
    tx.header().msg_id = MSG_RD_FPGA_VERSION;
    tx.header().cpu_flag = (CPUControlFlags::VALUE)(MSG_RD_FPGA_VERSION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }

  void fpga_functions(TxDatagram& tx) const noexcept override {
    tx.header().msg_id = MSG_RD_FPGA_FUNCTION;
    tx.header().cpu_flag = (CPUControlFlags::VALUE)(MSG_RD_FPGA_FUNCTION);  // For backward compatibility before 1.9
    tx.num_bodies = 0;
  }
};

}  // namespace autd3::driver
