// File: operation.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <sstream>

#include "datagram.hpp"

namespace autd3::driver {

inline void clear(TxDatagram& tx) noexcept {
  tx.header().msg_id = MSG_CLEAR;
  tx.num_bodies = 0;
}

inline void sync(const uint8_t msg_id, const uint16_t sync_cycle_ticks, const gsl::span<uint16_t> cycles, TxDatagram& tx) noexcept {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.set(CPUControlFlags::DO_SYNC);
  tx.header().sync_header().ecat_sync_cycle_ticks = sync_cycle_ticks;

  for (size_t i = 0; i < tx.bodies().size(); i++) {
    auto& dst = tx.bodies()[i];
    const auto src = cycles.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
    std::memcpy(&dst.data[0], src.data(), src.size_bytes());
  }

  tx.num_bodies = tx.bodies().size();
}

inline void modulation(const uint8_t msg_id, const gsl::span<uint8_t> mod_data, const bool is_first_frame, const uint32_t freq_div,
                       const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::DO_SYNC);
  tx.header().cpu_flag.remove(CPUControlFlags::CONFIG_SILENCER);

  if (is_first_frame) {
    if (freq_div < MOD_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "Modulation frequency division is oud of range. Minimum is " << MOD_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }

    tx.header().cpu_flag.set(CPUControlFlags::MOD_BEGIN);
    tx.header().mod_head().freq_div = freq_div;
    std::memcpy(&tx.header().mod_head().data[0], mod_data.data(), mod_data.size_bytes());
  } else {
    std::memcpy(&tx.header().mod_body().data[0], mod_data.data(), mod_data.size_bytes());
  }
  tx.header().size = gsl::narrow_cast<uint8_t>(mod_data.size());

  if (is_last_frame) {
    tx.header().cpu_flag.set(CPUControlFlags::MOD_END);
  }
}

inline void config_silencer(const uint8_t msg_id, const uint16_t cycle, const uint16_t step, TxDatagram& tx) {
  if (cycle < SILENCER_CYCLE_MIN) {
    std::stringstream ss;
    ss << "Silencer cycle is oud of range. Minimum is " << SILENCER_CYCLE_MIN << ", but you use " << cycle;
    throw std::runtime_error(ss.str());
  }

  tx.header().msg_id = msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::DO_SYNC);
  tx.header().cpu_flag.set(CPUControlFlags::CONFIG_SILENCER);

  tx.header().silencer_header().cycle = cycle;
  tx.header().silencer_header().step = step;
}

inline void normal_legacy(const uint8_t msg_id, const gsl::span<LegacyDrive> drives, TxDatagram& tx) noexcept {
  tx.header().msg_id = msg_id;

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  for (size_t i = 0; i < tx.bodies().size(); i++) {
    auto& dst = tx.bodies()[i];
    const auto src = drives.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
    std::memcpy(&dst.data[0], src.data(), src.size_bytes());
  }

  tx.num_bodies = tx.bodies().size();
}

inline void normal_duty(const uint8_t msg_id, const gsl::span<Duty> drives, TxDatagram& tx) noexcept {
  tx.header().msg_id = msg_id;

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

  for (size_t i = 0; i < tx.bodies().size(); i++) {
    auto& dst = tx.bodies()[i];
    const auto src = drives.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
    std::memcpy(&dst.data[0], src.data(), src.size_bytes());
  }

  tx.num_bodies = tx.bodies().size();
}

inline void normal_phase(const uint8_t msg_id, const gsl::span<Phase> drives, TxDatagram& tx) noexcept {
  tx.header().msg_id = msg_id;

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_MODE);

  tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

  for (size_t i = 0; i < tx.bodies().size(); i++) {
    auto& dst = tx.bodies()[i];
    const auto src = drives.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
    std::memcpy(&dst.data[0], src.data(), src.size_bytes());
  }

  tx.num_bodies = tx.bodies().size();
}

inline void point_stm(const uint8_t msg_id, const gsl::span<gsl::span<STMFocus>> points, const bool is_first_frame, const uint32_t freq_div,
                      const double sound_speed, const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().msg_id = msg_id;

  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.remove(FPGAControlFlags::STM_GAIN_MODE);

  if (is_first_frame) {
    if (freq_div < STM_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "STM frequency division is oud of range. Minimum is " << STM_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    const auto sound_speed_internal = gsl::narrow_cast<uint32_t>(std::round(sound_speed * 1024.0));

    for (size_t i = 0; i < tx.bodies().size(); i++) {
      auto& d = tx.bodies()[i];
      const gsl::span<STMFocus> s = points[i];
      d.point_stm_head().set_size(gsl::narrow_cast<uint16_t>(s.size()));
      d.point_stm_head().set_freq_div(freq_div);
      d.point_stm_head().set_sound_speed(sound_speed_internal);
      d.point_stm_head().set_point(s);
    }
  } else {
    for (size_t i = 0; i < tx.bodies().size(); i++) {
      auto& d = tx.bodies()[i];
      const gsl::span<STMFocus> s = points[i];
      d.point_stm_body().set_size(gsl::narrow_cast<uint16_t>(s.size()));
      d.point_stm_body().set_point(s);
    }
  }

  if (is_last_frame) {
    tx.header().cpu_flag.set(CPUControlFlags::STM_END);
  }

  tx.num_bodies = tx.bodies().size();
}

inline void gain_stm_legacy(const uint8_t msg_id, const gsl::span<LegacyDrive> drives, const bool is_first_frame, const uint32_t freq_div,
                            const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().msg_id = msg_id;

  tx.header().fpga_flag.set(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  if (is_first_frame) {
    if (freq_div < STM_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "STM frequency division is oud of range. Minimum is " << STM_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.bodies().size(); i++) {
      auto& dst = tx.bodies()[i];
      const auto src = drives.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
      std::memcpy(&dst.data[0], src.data(), src.size_bytes());
    }
  } else {
    for (size_t i = 0; i < tx.bodies().size(); i++) {
      auto& dst = tx.bodies()[i];
      const auto src = drives.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
      std::memcpy(&dst.data[0], src.data(), src.size_bytes());
    }
  }

  if (is_last_frame) {
    tx.header().cpu_flag.set(CPUControlFlags::STM_END);
  }

  tx.num_bodies = tx.bodies().size();
}

inline void gain_stm_phase(const uint8_t msg_id, const gsl::span<Phase> drives, const bool is_first_frame, const uint32_t freq_div,
                           TxDatagram& tx) noexcept(false) {
  tx.header().msg_id = msg_id;

  tx.header().cpu_flag.remove(CPUControlFlags::IS_DUTY);

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  if (is_first_frame) {
    if (freq_div < STM_SAMPLING_FREQ_DIV_MIN) {
      std::stringstream ss;
      ss << "STM frequency division is oud of range. Minimum is " << STM_SAMPLING_FREQ_DIV_MIN << ", but you use " << freq_div;
      throw std::runtime_error(ss.str());
    }

    tx.header().cpu_flag.set(CPUControlFlags::STM_BEGIN);
    for (size_t i = 0; i < tx.bodies().size(); i++) {
      auto& dst = tx.bodies()[i];
      const auto src = drives.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
      std::memcpy(&dst.data[0], src.data(), src.size_bytes());
    }
  } else {
    for (size_t i = 0; i < tx.bodies().size(); i++) {
      auto& dst = tx.bodies()[i];
      const auto src = drives.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
      std::memcpy(&dst.data[0], src.data(), src.size_bytes());
    }
  }

  tx.num_bodies = tx.bodies().size();
}

inline void gain_stm_duty(const uint8_t msg_id, const gsl::span<Duty> drives, const bool is_last_frame, TxDatagram& tx) noexcept(false) {
  tx.header().msg_id = msg_id;

  tx.header().cpu_flag.set(CPUControlFlags::IS_DUTY);

  tx.header().fpga_flag.remove(FPGAControlFlags::LEGACY_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_MODE);
  tx.header().fpga_flag.set(FPGAControlFlags::STM_GAIN_MODE);

  for (size_t i = 0; i < tx.bodies().size(); i++) {
    auto& dst = tx.bodies()[i];
    const auto src = drives.subspan(i * NUM_TRANS_IN_UNIT, NUM_TRANS_IN_UNIT);
    std::memcpy(&dst.data[0], src.data(), src.size_bytes());
  }

  if (is_last_frame) {
    tx.header().cpu_flag.set(CPUControlFlags::STM_END);
  }

  tx.num_bodies = tx.bodies().size();
}

inline void force_fan(TxDatagram& tx, const bool value) noexcept {
  if (value) {
    tx.header().fpga_flag.set(FPGAControlFlags::FORCE_FAN);
  } else {
    tx.header().fpga_flag.remove(FPGAControlFlags::FORCE_FAN);
  }
}

inline void reads_fpga_info(TxDatagram& tx, const bool value) noexcept {
  if (value) {
    tx.header().cpu_flag.set(CPUControlFlags::READS_FPGA_INFO);
  } else {
    tx.header().cpu_flag.remove(CPUControlFlags::READS_FPGA_INFO);
  }
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
