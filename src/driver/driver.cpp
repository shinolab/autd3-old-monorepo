// File: driver.cpp
// Project: v2_7
// Created Date: 14/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 05/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/driver/driver.hpp"

#include "../spdlog.hpp"
#include "autd3/driver/defined.hpp"

namespace autd3::driver {

template <>
bool Sync<Legacy>::pack(TxDatagram& tx) {
  if (std::any_of(_cycles, _cycles + _size, [](const uint16_t cycle) { return cycle != 4096; })) {
    spdlog::error("Cannot change frequency in LegacyMode.");
    return false;
  }

  tx.header().cpu_flag.remove(CPUControlFlags::Mod);
  tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
  tx.header().cpu_flag.set(CPUControlFlags::ConfigSync);
  tx.num_bodies = tx.num_devices();

  std::memcpy(tx.bodies_raw_ptr(), _cycles, tx.bodies_size());

  return true;
}

template <>
bool Sync<Normal>::pack(TxDatagram& tx) {
  tx.header().cpu_flag.remove(CPUControlFlags::Mod);
  tx.header().cpu_flag.remove(CPUControlFlags::ConfigSilencer);
  tx.header().cpu_flag.set(CPUControlFlags::ConfigSync);
  tx.num_bodies = tx.num_devices();

  std::memcpy(tx.bodies_raw_ptr(), _cycles, tx.bodies_size());

  return true;
}

bool Modulation::pack(TxDatagram& tx) {
  if (_size > MOD_BUF_SIZE_MAX) {
    spdlog::error("Modulation buffer overflow");
    return false;
  }

  const auto is_first_frame = *_sent == 0;
  const auto max_size = is_first_frame ? MOD_HEADER_INITIAL_DATA_SIZE : MOD_HEADER_SUBSEQUENT_DATA_SIZE;
  const auto mod_size = (std::min)(_size - *_sent, max_size);
  const auto is_last_frame = *_sent + mod_size == _size;
  const auto* buf = _mod_data + *_sent;

  tx.header().msg_id = _msg_id;
  tx.header().cpu_flag.set(CPUControlFlags::Mod);
  tx.header().cpu_flag.remove(CPUControlFlags::ModBegin);
  tx.header().cpu_flag.remove(CPUControlFlags::ModEnd);
  tx.header().size = static_cast<uint8_t>(mod_size);

  if (mod_size == 0) {
    tx.header().cpu_flag.remove(CPUControlFlags::Mod);
    return true;
  }

  if (is_first_frame) {
    if (_freq_div < MOD_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("Modulation frequency division is out of range. Minimum is {}, but you use {}.", MOD_SAMPLING_FREQ_DIV_MIN, _freq_div);
      return false;
    }

    tx.header().cpu_flag.set(CPUControlFlags::ModBegin);
    tx.header().mod_initial().freq_div = _freq_div;
    std::memcpy(&tx.header().mod_initial().data[0], buf, mod_size);
  } else {
    std::memcpy(&tx.header().mod_subsequent().data[0], buf, mod_size);
  }

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::ModEnd);

  *_sent += mod_size;
  return true;
}

bool ConfigSilencer::pack(TxDatagram& tx) {
  if (_cycle < SILENCER_CYCLE_MIN) {
    spdlog::error("Silencer cycle is out of range. Minimum is {}, but you use {}.", SILENCER_CYCLE_MIN, _cycle);
    return false;
  }

  tx.header().msg_id = _msg_id;
  tx.header().cpu_flag.remove(CPUControlFlags::Mod);
  tx.header().cpu_flag.remove(CPUControlFlags::ConfigSync);
  tx.header().cpu_flag.set(CPUControlFlags::ConfigSilencer);

  tx.header().silencer().cycle = _cycle;
  tx.header().silencer().step = _step;
  return true;
}

bool FocusSTMBody::pack(TxDatagram& tx) {
  if (_total_size > FOCUS_STM_BUF_SIZE_MAX) {
    spdlog::error("FocusSTM out of buffer");
    return false;
  }

  if (_start_idx) tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
  if (_finish_idx) tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);

  if (_points == nullptr || _points[0].empty()) return true;

  if (*_sent == 0) {
    if (_freq_div < FOCUS_STM_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("STM frequency division is out of range. Minimum is {}, but you use {}.", FOCUS_STM_SAMPLING_FREQ_DIV_MIN, _freq_div);
      return false;
    }
    tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
#ifdef AUTD3_USE_METER
    const auto sound_speed_internal = static_cast<uint32_t>(std::round(_sound_speed * 1024));
#else
    const auto sound_speed_internal = static_cast<uint32_t>(std::round(_sound_speed / 1000 * 1024));
#endif
    for (size_t i = 0; i < tx.num_devices(); i++) {
      auto& d = tx.body(i);
      const auto& s = _points[i];
      d.focus_stm_initial().set_size(static_cast<uint16_t>(s.size()));
      d.focus_stm_initial().set_freq_div(_freq_div);
      d.focus_stm_initial().set_sound_speed(sound_speed_internal);
      if (_start_idx) {
        if (static_cast<size_t>(_start_idx.value()) >= _total_size) {
          spdlog::error("STM start index out of range");
          return false;
        }
        d.focus_stm_initial().set_stm_start_idx(_start_idx.value());
      }
      if (_finish_idx) {
        if (static_cast<size_t>(_finish_idx.value()) >= _total_size) {
          spdlog::error("STM finish index out of range");
          return false;
        }
        d.focus_stm_initial().set_stm_finish_idx(_finish_idx.value());
      }
      d.focus_stm_initial().set_point(s);
    }
  } else {
    for (size_t i = 0; i < tx.num_devices(); i++) {
      auto& d = tx.body(i);
      const auto& s = _points[i];
      d.focus_stm_subsequent().set_size(static_cast<uint16_t>(s.size()));
      d.focus_stm_subsequent().set_point(s);
    }
  }

  tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

  const auto send_size = _points[0].size();
  if (*_sent + send_size == _total_size) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

  tx.num_bodies = tx.num_devices();

  *_sent += send_size;

  return true;
}

template <>
bool GainSTMBody<Legacy>::pack(TxDatagram& tx) {
  if (_size > GAIN_STM_LEGACY_BUF_SIZE_MAX) {
    spdlog::error("GainSTM out of buffer");
    return false;
  }

  if (_start_idx) tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
  if (_finish_idx) tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);

  bool is_last_frame = false;
  if (*_sent == 0) {
    if (_freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("STM frequency division is out of range. Minimum is {}, but you use {}.", GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN, _freq_div);
      return false;
    }

    tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
    for (size_t i = 0; i < tx.num_devices(); i++) {
      tx.body(i).gain_stm_initial().set_freq_div(_freq_div);
      tx.body(i).gain_stm_initial().set_mode(_mode);
      tx.body(i).gain_stm_initial().set_cycle(_size);
      if (_start_idx) {
        if (static_cast<size_t>(_start_idx.value()) >= _size) {
          spdlog::error("STM start index out of range");
          return false;
        }
        tx.body(i).gain_stm_initial().set_stm_start_idx(_start_idx.value());
      }
      if (_finish_idx) {
        if (static_cast<size_t>(_finish_idx.value()) >= _size) {
          spdlog::error("STM finish index out of range");
          return false;
        }
        tx.body(i).gain_stm_initial().set_stm_finish_idx(_finish_idx.value());
      }
    }
    (*_sent)++;
  } else {
    switch (_mode) {
      case GainSTMMode::PhaseDutyFull:
        is_last_frame = *_sent + 1 >= _size + 1;
        {
          auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(_drives[*_sent - 1][i]);
        }
        (*_sent)++;
        break;
      case GainSTMMode::PhaseFull:
        is_last_frame = *_sent + 2 >= _size + 1;
        {
          auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(0, _drives[*_sent - 1][i]);
        }
        (*_sent)++;
        if (*_sent - 1 < _size) {
          auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(1, _drives[*_sent - 1][i]);
          (*_sent)++;
        }
        break;
      case GainSTMMode::PhaseHalf:
        is_last_frame = *_sent + 4 >= _size + 1;
        {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(0, _drives[*_sent - 1][i]);
        }
        (*_sent)++;
        if (*_sent - 1 < _size) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(1, _drives[*_sent - 1][i]);
          (*_sent)++;
        }
        if (*_sent - 1 < _size) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(2, _drives[*_sent - 1][i]);
          (*_sent)++;
        }
        if (*_sent - 1 < _size) {
          auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
          for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(3, _drives[*_sent - 1][i]);
          (*_sent)++;
        }
        break;
    }
  }

  tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

  if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

  tx.num_bodies = tx.num_devices();

  return true;
}

template <>
bool GainSTMBody<NormalPhase>::pack(TxDatagram& tx) {
  if (_size > GAIN_STM_BUF_SIZE_MAX) {
    spdlog::error("GainSTM out of buffer");
    return false;
  }

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26813)
#endif
  if (_mode == GainSTMMode::PhaseHalf) {
    spdlog::error("PhaseHalf is not supported in normal mode");
    return false;
  }
#ifdef _MSC_VER
#pragma warning(pop)
#endif

  if (_start_idx) tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
  if (_finish_idx) tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
  tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

  if (*_sent == 0) {
    if (_freq_div < GAIN_STM_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("STM frequency division is out of range. Minimum is {}, but you use {}.", GAIN_STM_SAMPLING_FREQ_DIV_MIN, _freq_div);
      return false;
    }
    tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
    for (size_t i = 0; i < tx.num_devices(); i++) {
      tx.body(i).gain_stm_initial().set_freq_div(_freq_div);
      tx.body(i).gain_stm_initial().set_mode(_mode);
      tx.body(i).gain_stm_initial().set_cycle(_size);
      if (_start_idx) {
        if (static_cast<size_t>(_start_idx.value()) >= _size) {
          spdlog::error("STM start index out of range");
          return false;
        }
        tx.body(i).gain_stm_initial().set_stm_start_idx(_start_idx.value());
      }
      if (_finish_idx) {
        if (static_cast<size_t>(_finish_idx.value()) >= _size) {
          spdlog::error("STM finish index out of range");
          return false;
        }
        tx.body(i).gain_stm_initial().set_stm_finish_idx(_finish_idx.value());
      }
    }
  } else {
    auto* p = reinterpret_cast<Phase*>(tx.bodies_raw_ptr());
    for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(_drives[*_sent - 1][i], _cycles[i]);
  }

  if (*_sent + 1 == _size + 1) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

  tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

  tx.num_bodies = tx.num_devices();
  return true;
}

template <>
bool GainSTMBody<NormalDuty>::pack(TxDatagram& tx) {
  if (_size > GAIN_STM_BUF_SIZE_MAX) {
    spdlog::error("GainSTM out of buffer");
    return false;
  }

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26813)
#endif
  if (_mode == GainSTMMode::PhaseHalf) {
    spdlog::error("PhaseHalf is not supported in normal mode");
    return false;
  }
#ifdef _MSC_VER
#pragma warning(pop)
#endif

  if (_start_idx) tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
  if (_finish_idx) tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
  tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

  if (*_sent == 0) {
    if (_freq_div < GAIN_STM_SAMPLING_FREQ_DIV_MIN) {
      spdlog::error("STM frequency division is out of range. Minimum is {}, but you use {}.", GAIN_STM_SAMPLING_FREQ_DIV_MIN, _freq_div);
      return false;
    }
    tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
    for (size_t i = 0; i < tx.num_devices(); i++) {
      tx.body(i).gain_stm_initial().set_freq_div(_freq_div);
      tx.body(i).gain_stm_initial().set_mode(_mode);
      tx.body(i).gain_stm_initial().set_cycle(_size);
      if (_start_idx) {
        if (static_cast<size_t>(_start_idx.value()) >= _size) {
          spdlog::error("STM start index out of range");
          return false;
        }
        tx.body(i).gain_stm_initial().set_stm_start_idx(_start_idx.value());
      }
      if (_finish_idx) {
        if (static_cast<size_t>(_finish_idx.value()) >= _size) {
          spdlog::error("STM finish index out of range");
          return false;
        }
        tx.body(i).gain_stm_initial().set_stm_finish_idx(_finish_idx.value());
      }
    }
  } else {
    auto* p = reinterpret_cast<Duty*>(tx.bodies_raw_ptr());
    for (size_t i = 0; i < _drives[*_sent - 1].size(); i++) p[i].set(_drives[*_sent - 1][i], _cycles[i]);
  }

  if (*_sent + 1 == _size + 1) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

  tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

  tx.num_bodies = tx.num_devices();

  return true;
}

}  // namespace autd3::driver
