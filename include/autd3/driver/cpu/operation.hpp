// File: operation.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <sstream>
#include <vector>

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
  tx.num_bodies = 0;
}

inline void sync(const uint8_t msg_id, const uint16_t sync_cycle_ticks, const uint16_t* const cycles, TxDatagram& tx) noexcept {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);
  tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SYNC);
  tx.header().sync_header().ecat_sync_cycle_ticks = sync_cycle_ticks;

  std::memcpy(tx.bodies(), cycles, sizeof(Body) * tx.size());

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
    if (freq_div < MOD_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "Modulation frequency division is oud of range. Minimum is " << MOD_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }
    tx.header().cpu_flag.set(CPUControlFlags::MOD_BEGIN);
    tx.header().mod_head().freq_div = freq_div;
    std::memcpy(&tx.header().mod_head().data[0], mod_data, mod_size);
  } else {
    std::memcpy(&tx.header().mod_body().data[0], mod_data, mod_size);
  }

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::MOD_END);
}

inline void config_silencer(const uint8_t msg_id, const uint16_t cycle, const uint16_t step, TxDatagram& tx) {
  if (cycle < SILENCER_CYCLE_MIN) {
    std::stringstream ss;
    ss << "Silencer cycle is oud of range. Minimum is " << SILENCER_CYCLE_MIN << ", but you use " << cycle;
    throw std::runtime_error(ss.str());
  }

  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::MOD);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SYNC);
  tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SILENCER);

  tx.header().silencer_header().cycle = cycle;
  tx.header().silencer_header().step = step;
}

inline void normal_legacy_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.num_bodies = 0;
}

inline void normal_legacy_body(const LegacyDrive* const drives, TxDatagram& tx) noexcept {
  std::memcpy(tx.bodies(), drives, sizeof(Body) * tx.size());

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void normal_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.num_bodies = 0;
}

inline void normal_duty_body(const Duty* drives, TxDatagram& tx) noexcept {
  tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

  std::memcpy(tx.bodies(), drives, sizeof(Body) * tx.size());
  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void normal_phase_body(const Phase* drives, TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

  std::memcpy(tx.bodies(), drives, sizeof(Body) * tx.size());
  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void point_stm_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
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
    if (freq_div < STM_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "STM frequency division is oud of range. Minimum is " << STM_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    const auto sound_speed_internal = static_cast<uint32_t>(std::round(sound_speed * 1024.0));

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
  tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  tx.num_bodies = 0;
}

inline void gain_stm_legacy_body(const LegacyDrive* const drives, const bool is_first_frame, const uint32_t freq_div, const bool is_last_frame,
                                 TxDatagram& tx) noexcept(false) {
  if (is_first_frame) {
    if (freq_div < STM_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "STM frequency division is oud of range. Minimum is " << STM_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.size(); i++) tx.bodies()[i].gain_stm_head().set_freq_div(freq_div);
  } else {
    std::memcpy(tx.bodies(), drives, sizeof(Body) * tx.size());
  }

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STM_END);

  tx.num_bodies = tx.size();
}

inline void gain_stm_normal_header(TxDatagram& tx) noexcept {
  tx.header().cpu_flag.remove(CPUControlFlags::WRITE_BODY);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_BEGIN);
  tx.header().cpu_flag.remove(CPUControlFlags::STM_END);

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  tx.num_bodies = 0;
}

inline void gain_stm_normal_phase(const Phase* const drives, const bool is_first_frame, const uint32_t freq_div, TxDatagram& tx) noexcept(false) {
  tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

  if (is_first_frame) {
    if (freq_div < STM_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "STM frequency division is oud of range. Minimum is " << STM_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.size(); i++) tx.bodies()[i].gain_stm_head().set_freq_div(freq_div);
  } else {
    std::memcpy(tx.bodies(), drives, sizeof(Body) * tx.size());
  }

  tx.header().cpu_flag.set(CPUControlFlags::WRITE_BODY);

  tx.num_bodies = tx.size();
}

inline void gain_stm_normal_duty(const Duty* const drives, const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

  std::memcpy(tx.bodies(), drives, sizeof(Body) * tx.size());
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
    tx.header().cpu_flag.set(CPUControlFlags::READS_FPGA_INFO);
  else
    tx.header().cpu_flag.remove(CPUControlFlags::READS_FPGA_INFO);
}

inline void cpu_version(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_RD_CPU_VERSION;
  tx.header().cpu_flag = CPUControlFlags::MOD_END;  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

inline void fpga_version(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_RD_FPGA_VERSION;
  tx.header().cpu_flag = CPUControlFlags::STM_BEGIN;  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

inline void fpga_functions(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_RD_FPGA_FUNCTION;
  tx.header().cpu_flag =
      static_cast<CPUControlFlags::VALUE>(CPUControlFlags::STM_BEGIN | CPUControlFlags::MOD_BEGIN);  // For backward compatibility before 1.9
  tx.num_bodies = 0;
}

}  // namespace autd3::driver
