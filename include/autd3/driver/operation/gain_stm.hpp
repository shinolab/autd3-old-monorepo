// File: gain_stm.hpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <spdlog/spdlog.h>

#include <optional>

#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct GainSTMBase : Operation {
  [[nodiscard]] bool is_finished() const override { return _sent >= drives.size() + 1; }

  std::vector<std::vector<Drive>> drives{};
  uint32_t freq_div{4096};
  GainSTMMode mode{GainSTMMode::PhaseDutyFull};
  std::optional<uint16_t> start_idx{std::nullopt};
  std::optional<uint16_t> finish_idx{std::nullopt};

 protected:
  size_t _sent{0};
};

template <typename T>
struct GainSTM;

template <>
struct GainSTM<Legacy> final : GainSTMBase {
  void init() override { _sent = 0; }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().cpu_flag.remove(CPUControlFlags::STMBegin);
    tx.header().cpu_flag.remove(CPUControlFlags::STMEnd);

    tx.header().fpga_flag.set(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.set(FPGAControlFlags::STMMode);
    tx.header().fpga_flag.set(FPGAControlFlags::STMGainMode);

    tx.num_bodies = 0;

    if (is_finished()) return;

    if (drives.size() > GAIN_STM_LEGACY_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    if (start_idx) {
      if (static_cast<size_t>(start_idx.value()) >= drives.size()) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
    }
    if (finish_idx) {
      if (static_cast<size_t>(finish_idx.value()) >= drives.size()) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);
    }

    bool is_last_frame = false;
    if (_sent == 0) {
      if (freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
        throw std::runtime_error("STM frequency division is out of range. Minimum is" + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                                 " but you use " + std::to_string(freq_div));

      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
      for (size_t i = 0; i < tx.num_devices(); i++) {
        tx.body(i).gain_stm_initial().set_freq_div(freq_div);
        tx.body(i).gain_stm_initial().set_mode(mode);
        tx.body(i).gain_stm_initial().set_cycle(drives.size());
        tx.body(i).gain_stm_initial().set_stm_start_idx(start_idx.value_or(0));
        tx.body(i).gain_stm_initial().set_stm_finish_idx(finish_idx.value_or(0));
      }
      _sent++;
    } else {
      switch (mode) {
        case GainSTMMode::PhaseDutyFull:
          is_last_frame = _sent + 1 >= drives.size() + 1;
          {
            auto* p = reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr());
            for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(drives[_sent - 1][i]);
          }
          _sent++;
          break;
        case GainSTMMode::PhaseFull:
          is_last_frame = _sent + 2 >= drives.size() + 1;
          {
            auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies_raw_ptr());
            for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(0, drives[_sent - 1][i]);
          }
          _sent++;
          if (_sent - 1 < drives.size()) {
            auto* p = reinterpret_cast<LegacyPhaseFull*>(tx.bodies_raw_ptr());
            for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(1, drives[_sent - 1][i]);
            _sent++;
          }
          break;
        case GainSTMMode::PhaseHalf:
          is_last_frame = _sent + 4 >= drives.size() + 1;
          {
            auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
            for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(0, drives[_sent - 1][i]);
          }
          _sent++;
          if (_sent - 1 < drives.size()) {
            auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
            for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(1, drives[_sent - 1][i]);
            _sent++;
          }
          if (_sent - 1 < drives.size()) {
            auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
            for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(2, drives[_sent - 1][i]);
            _sent++;
          }
          if (_sent - 1 < drives.size()) {
            auto* p = reinterpret_cast<LegacyPhaseHalf*>(tx.bodies_raw_ptr());
            for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(3, drives[_sent - 1][i]);
            _sent++;
          }
          break;
      }
    }

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.num_bodies = tx.num_devices();
  }
};

template <>
struct GainSTM<Normal> final : GainSTMBase {
  void init() override {
    _sent = 0;
    _next_duty = false;
  }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().cpu_flag.remove(CPUControlFlags::STMBegin);
    tx.header().cpu_flag.remove(CPUControlFlags::STMEnd);

    tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.set(FPGAControlFlags::STMMode);
    tx.header().fpga_flag.set(FPGAControlFlags::STMGainMode);

    tx.num_bodies = 0;

    if (is_finished()) return;

    if (_sent == 0) {
      pack_phase(tx);
      _sent++;
      return;
    }

    switch (mode) {
      case GainSTMMode::PhaseDutyFull:
        if (_next_duty) {
          pack_duty(tx);
          _sent++;
        } else {
          pack_phase(tx);
        }
        _next_duty = !_next_duty;
        break;
      case GainSTMMode::PhaseFull:
        pack_phase(tx);
        _sent++;
        break;
      case GainSTMMode::PhaseHalf:
        throw std::runtime_error("PhaseHalf is not supported in normal mode");
    }
  }

  std::vector<uint16_t> cycles{};

 private:
  bool _next_duty{false};

  void pack_phase(TxDatagram& tx) const {
    if (drives.size() > GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    if (start_idx) {
      if (static_cast<size_t>(start_idx.value()) >= drives.size()) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
    }
    if (finish_idx) {
      if (static_cast<size_t>(finish_idx.value()) >= drives.size()) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);
    }

    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    if (_sent == 0) {
      if (freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
        throw std::runtime_error("STM frequency division is out of range. Minimum is" + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                                 " but you use " + std::to_string(freq_div));

      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
      for (size_t i = 0; i < tx.num_devices(); i++) {
        tx.body(i).gain_stm_initial().set_freq_div(freq_div);
        tx.body(i).gain_stm_initial().set_mode(mode);
        tx.body(i).gain_stm_initial().set_cycle(drives.size());
        tx.body(i).gain_stm_initial().set_stm_start_idx(start_idx.value_or(0));
        tx.body(i).gain_stm_initial().set_stm_finish_idx(finish_idx.value_or(0));
      }
    } else {
      auto* p = reinterpret_cast<Phase*>(tx.bodies_raw_ptr());
      for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(drives[_sent - 1][i], cycles[i]);
    }

    if (_sent + 1 == drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    tx.num_bodies = tx.num_devices();
  }

  void pack_duty(TxDatagram& tx) const {
    if (drives.size() > GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    if (start_idx) {
      if (static_cast<size_t>(start_idx.value()) >= drives.size()) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    }
    if (finish_idx) {
      if (static_cast<size_t>(finish_idx.value()) >= drives.size()) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    }

    tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

    if (_sent == 0) {
      if (freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
        throw std::runtime_error("STM frequency division is out of range. Minimum is" + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                                 " but you use " + std::to_string(freq_div));

      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
      for (size_t i = 0; i < tx.num_devices(); i++) {
        tx.body(i).gain_stm_initial().set_freq_div(freq_div);
        tx.body(i).gain_stm_initial().set_mode(mode);
        tx.body(i).gain_stm_initial().set_cycle(drives.size());
        tx.body(i).gain_stm_initial().set_stm_start_idx(start_idx.value_or(0));
        tx.body(i).gain_stm_initial().set_stm_finish_idx(finish_idx.value_or(0));
      }
    } else {
      auto* p = reinterpret_cast<Duty*>(tx.bodies_raw_ptr());
      for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(drives[_sent - 1][i], cycles[i]);
    }

    if (_sent + 1 == drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    tx.num_bodies = tx.num_devices();
  }
};

template <>
struct GainSTM<NormalPhase> final : GainSTMBase {
  void init() override { _sent = 0; }

  void pack(TxDatagram& tx) override {
    tx.header().cpu_flag.remove(CPUControlFlags::WriteBody);
    tx.header().cpu_flag.remove(CPUControlFlags::ModDelay);
    tx.header().cpu_flag.remove(CPUControlFlags::STMBegin);
    tx.header().cpu_flag.remove(CPUControlFlags::STMEnd);

    tx.header().fpga_flag.remove(FPGAControlFlags::LegacyMode);
    tx.header().fpga_flag.set(FPGAControlFlags::STMMode);
    tx.header().fpga_flag.set(FPGAControlFlags::STMGainMode);

    tx.num_bodies = 0;

    if (is_finished()) return;

    pack_phase(tx);
    _sent++;
  }

  std::vector<uint16_t> cycles{};

 private:
  void pack_phase(TxDatagram& tx) const {
    if (drives.size() > GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    if (start_idx) {
      if (static_cast<size_t>(start_idx.value()) >= drives.size()) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
    }
    if (finish_idx) {
      if (static_cast<size_t>(finish_idx.value()) >= drives.size()) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);
    }

    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    if (_sent == 0) {
      if (freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
        throw std::runtime_error("STM frequency division is out of range. Minimum is" + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                                 " but you use " + std::to_string(freq_div));

      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
      for (size_t i = 0; i < tx.num_devices(); i++) {
        tx.body(i).gain_stm_initial().set_freq_div(freq_div);
        tx.body(i).gain_stm_initial().set_mode(GainSTMMode::PhaseFull);
        tx.body(i).gain_stm_initial().set_cycle(drives.size());
        tx.body(i).gain_stm_initial().set_stm_start_idx(start_idx.value_or(0));
        tx.body(i).gain_stm_initial().set_stm_finish_idx(finish_idx.value_or(0));
      }
    } else {
      auto* p = reinterpret_cast<Phase*>(tx.bodies_raw_ptr());
      for (size_t i = 0; i < drives[_sent - 1].size(); i++) p[i].set(drives[_sent - 1][i], cycles[i]);
    }

    if (_sent + 1 == drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    tx.num_bodies = tx.num_devices();
  }
};

}  // namespace autd3::driver
