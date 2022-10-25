// File: operation.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "body.hpp"
#include "datagram.hpp"

namespace autd3::driver {

inline void clear(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_CLEAR;
  tx.num_bodies = 0;
}

inline void null_header(const uint8_t msg_id, TxDatagram& tx) noexcept {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SYNC);
  tx.header().size = 0;
}

inline void null_body(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
  tx.num_bodies = 0;
}

inline void sync(const uint8_t msg_id, const uint16_t* const cycles, TxDatagram& tx) noexcept {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);
  tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SYNC);

  std::memcpy(reinterpret_cast<uint16_t*>(tx.bodies()), cycles, sizeof(Body) * tx.size());

  tx.num_bodies = tx.size();
}

inline void mod_delay(const uint16_t* const delays, TxDatagram& tx) noexcept {
  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.set(CPUControlFlags::MOD_DELAY);

  std::memcpy(reinterpret_cast<uint16_t*>(tx.bodies()), delays, sizeof(Body) * tx.size());

  tx.num_bodies = tx.size();
}

inline void modulation(const uint8_t msg_id, const uint8_t* const mod_data, const size_t mod_size, const bool is_first_frame, const uint32_t freq_div,
                       const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.set(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_END);
  tx.header().size = static_cast<uint8_t>(mod_size);

  if (mod_size == 0) {
    tx.header().cpu_flag.remove(CPUControlFlags::MOD);
    return;
  }

  if (is_first_frame) {
    if (freq_div < MOD_SAMPLING_FREQ_DIV_MIN)
      throw std::runtime_error("Modulation frequency division is oud of range. Minimum is " + std::to_string(MOD_SAMPLING_FREQ_DIV_MIN) +
                               ", but you use " + std::to_string(freq_div));

    tx.header().cpu_flag.set(CPUControlFlags::MOD_BEGIN);
    tx.header().mod_head().freq_div = freq_div;
    std::memcpy(&tx.header().mod_head().data[0], mod_data, mod_size);
  } else {
    std::memcpy(&tx.header().mod_body().data[0], mod_data, mod_size);
  }

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::MOD_END);
}

inline void config_silencer(const uint8_t msg_id, const uint16_t cycle, const uint16_t step, TxDatagram& tx) {
  if (cycle < SILENCER_CYCLE_MIN)
    throw std::runtime_error("Silencer cycle is oud of range. Minimum is " + std::to_string(SILENCER_CYCLE_MIN) + ", but you use " +
                             std::to_string(cycle));

  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SYNC);
  tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SILENCER);

  tx.header().silencer_header().cycle = cycle;
  tx.header().silencer_header().step = step;
}

inline void normal_legacy_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.num_bodies = 0;
}

inline void normal_legacy_body(const std::vector<Drive>& drives, TxDatagram& tx) noexcept {
  auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies());
  for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void normal_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.num_bodies = 0;
}

inline void normal_duty_body(const std::vector<Drive>& drives, TxDatagram& tx) noexcept {
  tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

  auto* p = reinterpret_cast<Duty*>(tx.bodies());
  for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void normal_phase_body(const std::vector<Drive>& drives, TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

  auto* p = reinterpret_cast<Phase*>(tx.bodies());
  for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void point_stm_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_GAIN_MODE);

  tx.num_bodies = 0;
}

inline void point_stm_body(const std::vector<std::vector<STMFocus>>& points, const bool is_first_frame, const uint32_t freq_div,
                           const double sound_speed, const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  if (points.empty() || points[0].empty()) return;

  if (is_first_frame) {
    if (freq_div < POINT_STM_SAMPLING_FREQ_DIV_MIN)
      throw std::runtime_error("STM frequency division is oud of range. Minimum is " + std::to_string(POINT_STM_SAMPLING_FREQ_DIV_MIN) +
                               ", but you use " + std::to_string(freq_div));

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

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.num_bodies = tx.size();
}

inline void gain_stm_legacy_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  tx.num_bodies = 0;
}

inline void gain_stm_legacy_body(const std::vector<const std::vector<Drive>*>& drives, const bool is_first_frame, const uint32_t freq_div,
                                 const bool is_last_frame, const GainSTMMode mode, TxDatagram& tx) noexcept(false) {
  if (is_first_frame) {
    if (freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
      throw std::runtime_error("STM frequency division is oud of range. Minimum is " + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                               ", but you use " + std::to_string(freq_div));

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.size(); i++) {
      tx.bodies()[i].gain_stm_head().set_freq_div(freq_div);
      tx.bodies()[i].gain_stm_head().set_mode(mode);
    }
  } else {
    switch (mode) {
      case GainSTMMode::PhaseDutyFull: {
        auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies());
        for (size_t i = 0; i < drives[0]->size(); i++) p[i].set(drives[0]->at(i));
      } break;
      case GainSTMMode::PhaseFull: {
        auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies());
        for (size_t i = 0; i < drives[0]->size(); i++) p[i].set(0, drives[0]->at(i));
      }
        if (drives[1] != nullptr) {
          auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies());
          for (size_t i = 0; i < drives[1]->size(); i++) p[i].set(1, drives[1]->at(i));
        }
        break;
      case GainSTMMode::PhaseHalf: {
        auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies());
        for (size_t i = 0; i < drives[0]->size(); i++) p[i].set(0, drives[0]->at(i));
      }
        if (drives[1] != nullptr) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies());
          for (size_t i = 0; i < drives[1]->size(); i++) p[i].set(1, drives[1]->at(i));
        }
        if (drives[2] != nullptr) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies());
          for (size_t i = 0; i < drives[2]->size(); i++) p[i].set(2, drives[2]->at(i));
        }
        if (drives[3] != nullptr) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies());
          for (size_t i = 0; i < drives[3]->size(); i++) p[i].set(3, drives[3]->at(i));
        }
        break;
      default: {
        auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies());
        for (size_t i = 0; i < drives[0]->size(); i++) p[i].set(drives[0]->at(i));
      } break;
    }
  }

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.num_bodies = tx.size();
}

inline void gain_stm_normal_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::MOD_DELAY);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  tx.num_bodies = 0;
}

inline void gain_stm_normal_phase(const std::vector<Drive>& drives, const bool is_first_frame, const uint32_t freq_div, const GainSTMMode mode,
                                  const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

#pragma warning(push)
#pragma warning(disable : 26813)
  if (mode == GainSTMMode::PhaseHalf) throw std::runtime_error("PhaseHalf is not supported in normal mode");
#pragma warning(pop)

  if (is_first_frame) {
    if (freq_div < GAIN_STM_SAMPLING_FREQ_DIV_MIN)
      throw std::runtime_error("STM frequency division is oud of range. Minimum is " + std::to_string(GAIN_STM_SAMPLING_FREQ_DIV_MIN) +
                               ", but you use " + std::to_string(freq_div));

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.size(); i++) {
      tx.bodies()[i].gain_stm_head().set_freq_div(freq_div);
      tx.bodies()[i].gain_stm_head().set_mode(mode);
    }
  } else {
    auto* p = reinterpret_cast<Phase*>(tx.bodies());
    for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);
  }

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void gain_stm_normal_duty(const std::vector<Drive>& drives, const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

  auto* p = reinterpret_cast<Duty*>(tx.bodies());
  for (size_t i = 0; i < drives.size(); i++) p[i].set(drives[i]);

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void force_fan(TxDatagram& tx, const bool value) noexcept {
  if (value)
    tx.header().fpga_flag.set(FPGAControlFlags::FORCE_FAN);
  else
    tx.header().fpga_flag.remove(FPGAControlFlags::FORCE_FAN);
}

inline void reads_fpga_info(TxDatagram& tx, const bool value) noexcept {
  if (value)
    tx.header().fpga_flag.set(FPGAControlFlags::READS_FPGA_INFO);
  else
    tx.header().fpga_flag.remove(FPGAControlFlags::READS_FPGA_INFO);
}

inline void cpu_version(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_RD_CPU_VERSION;
  tx.header().cpu_flag = (CPUControlFlags::VALUE)(MSG_RD_CPU_VERSION);  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

inline void fpga_version(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_RD_FPGA_VERSION;
  tx.header().cpu_flag = (CPUControlFlags::VALUE)(MSG_RD_FPGA_VERSION);  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

inline void fpga_functions(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_RD_FPGA_FUNCTION;
  tx.header().cpu_flag = (CPUControlFlags::VALUE)(MSG_RD_FPGA_FUNCTION);  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

}  // namespace autd3::driver
