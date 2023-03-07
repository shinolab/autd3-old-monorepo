// File: gain_stm.hpp
// Project: operation
// Created Date: 07/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <optional>
#include <utility>
#include <vector>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct GainSTMProps {
  uint32_t freq_div{4096};
  GainSTMMode mode{GainSTMMode::PhaseDutyFull};
  std::optional<uint16_t> start_idx{std::nullopt};
  std::optional<uint16_t> finish_idx{std::nullopt};
};

template <typename T>
struct GainSTM;

template <>
struct GainSTM<Legacy> final : Operation {
  explicit GainSTM(std::vector<std::vector<Drive>> drives, const GainSTMProps props) : _drives(std::move(drives)), _props(props) {}

  void init() override { _sent = 0; }

  [[nodiscard]] bool is_finished() const override { return _sent >= _drives.size() + 1; }

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

    if (_drives.size() > GAIN_STM_LEGACY_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    if (_props.start_idx) {
      if (static_cast<size_t>(_props.start_idx.value()) >= _drives.size()) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
    }
    if (_props.finish_idx) {
      if (static_cast<size_t>(_props.finish_idx.value()) >= _drives.size()) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);
    }

    bool is_last_frame = false;
    if (_sent == 0) {
      if (_props.freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
        throw std::runtime_error("STM frequency division is out of range. Minimum is " + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                                 ", but you use " + std::to_string(_props.freq_div));

      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
      std::for_each(tx.begin(), tx.end(), [this](const auto& body) {
        const auto& [_, d] = body;
        d.gain_stm_initial().freq_div = _props.freq_div;
        d.gain_stm_initial().mode = _props.mode;
        d.gain_stm_initial().cycle = static_cast<uint16_t>(_drives.size());
        d.gain_stm_initial().stm_start_idx = _props.start_idx.value_or(0);
        d.gain_stm_initial().stm_finish_idx = _props.finish_idx.value_or(0);
      });
      _sent++;
    } else {
      switch (_props.mode) {
        case GainSTMMode::PhaseDutyFull:
          is_last_frame = _sent + 1 >= _drives.size() + 1;
          std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), reinterpret_cast<LegacyDrive*>(tx.bodies_raw_ptr()),
                         [](const auto& d) { return d; });
          _sent++;
          break;
        case GainSTMMode::PhaseFull:
          is_last_frame = _sent + 2 >= _drives.size() + 1;
          std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), reinterpret_cast<LegacyPhaseFull0*>(tx.bodies_raw_ptr()),
                         [](const auto& d) { return d; });
          _sent++;
          if (_sent - 1 < _drives.size()) {
            std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), reinterpret_cast<LegacyPhaseFull1*>(tx.bodies_raw_ptr()),
                           [](const auto& d) { return d; });
            _sent++;
          }
          break;
        case GainSTMMode::PhaseHalf:
          is_last_frame = _sent + 4 >= _drives.size() + 1;
          std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), reinterpret_cast<LegacyPhaseHalf0*>(tx.bodies_raw_ptr()),
                         [](const auto& d) { return d; });
          _sent++;
          if (_sent - 1 < _drives.size()) {
            std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), reinterpret_cast<LegacyPhaseHalf1*>(tx.bodies_raw_ptr()),
                           [](const auto& d) { return d; });
            _sent++;
          }
          if (_sent - 1 < _drives.size()) {
            std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), reinterpret_cast<LegacyPhaseHalf2*>(tx.bodies_raw_ptr()),
                           [](const auto& d) { return d; });
            _sent++;
          }
          if (_sent - 1 < _drives.size()) {
            std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), reinterpret_cast<LegacyPhaseHalf3*>(tx.bodies_raw_ptr()),
                           [](const auto& d) { return d; });
            _sent++;
          }
          break;
      }
    }

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    if (is_last_frame) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.num_bodies = tx.num_devices();
  }

 private:
  std::vector<std::vector<Drive>> _drives{};
  GainSTMProps _props;
  size_t _sent{0};
};

template <>
struct GainSTM<Advanced> final : Operation {
  explicit GainSTM(std::vector<std::vector<Drive>> drives, std::vector<uint16_t> cycles, const GainSTMProps props)
      : _drives(std::move(drives)), _cycles(std::move(cycles)), _props(props) {}

  [[nodiscard]] bool is_finished() const override { return _sent >= _drives.size() + 1; }

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

    switch (_props.mode) {
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
        throw std::runtime_error("PhaseHalf is not supported in advanced mode");
    }
  }

 private:
  std::vector<std::vector<Drive>> _drives{};
  std::vector<uint16_t> _cycles{};
  GainSTMProps _props;
  size_t _sent{0};
  bool _next_duty{false};

  void pack_phase(TxDatagram& tx) const {
    if (_drives.size() > GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    if (_props.start_idx) {
      if (static_cast<size_t>(_props.start_idx.value()) >= _drives.size()) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
    }
    if (_props.finish_idx) {
      if (static_cast<size_t>(_props.finish_idx.value()) >= _drives.size()) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);
    }

    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    if (_sent == 0) {
      if (_props.freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
        throw std::runtime_error("STM frequency division is out of range. Minimum is" + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                                 " but you use " + std::to_string(_props.freq_div));

      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
      std::for_each(tx.begin(), tx.end(), [this](const auto& body) {
        const auto& [_, d] = body;
        d.gain_stm_initial().freq_div = _props.freq_div;
        d.gain_stm_initial().mode = _props.mode;
        d.gain_stm_initial().cycle = static_cast<uint16_t>(_drives.size());
        d.gain_stm_initial().stm_start_idx = _props.start_idx.value_or(0);
        d.gain_stm_initial().stm_finish_idx = _props.finish_idx.value_or(0);
      });
    } else {
      std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), _cycles.begin(),
                     reinterpret_cast<AdvancedDrivePhase*>(tx.bodies_raw_ptr()),
                     [](const auto& d, const auto cycle) { return AdvancedDrivePhase(d, cycle); });
    }

    if (_sent + 1 == _drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    tx.num_bodies = tx.num_devices();
  }

  void pack_duty(TxDatagram& tx) const {
    if (_drives.size() > GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    if (_props.start_idx) {
      if (static_cast<size_t>(_props.start_idx.value()) >= _drives.size()) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    }
    if (_props.finish_idx) {
      if (static_cast<size_t>(_props.finish_idx.value()) >= _drives.size()) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    }

    tx.header().cpu_flag.set(CPUControlFlags::IsDuty);

    if (_sent == 0) {
      if (_props.freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
        throw std::runtime_error("STM frequency division is out of range. Minimum is" + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                                 " but you use " + std::to_string(_props.freq_div));

      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
      std::for_each(tx.begin(), tx.end(), [this](const auto& body) {
        const auto& [_, d] = body;
        d.gain_stm_initial().freq_div = _props.freq_div;
        d.gain_stm_initial().mode = _props.mode;
        d.gain_stm_initial().cycle = static_cast<uint16_t>(_drives.size());
        d.gain_stm_initial().stm_start_idx = _props.start_idx.value_or(0);
        d.gain_stm_initial().stm_finish_idx = _props.finish_idx.value_or(0);
      });
    } else {
      std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), _cycles.begin(), reinterpret_cast<AdvancedDriveDuty*>(tx.bodies_raw_ptr()),
                     [](const auto& d, const auto cycle) { return AdvancedDriveDuty(d, cycle); });
    }

    if (_sent + 1 == _drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    tx.num_bodies = tx.num_devices();
  }
};

template <>
struct GainSTM<AdvancedPhase> final : Operation {
  explicit GainSTM(std::vector<std::vector<Drive>> drives, std::vector<uint16_t> cycles, const GainSTMProps props)
      : _drives(std::move(drives)), _cycles(std::move(cycles)), _props(props) {}

  void init() override { _sent = 0; }

  [[nodiscard]] bool is_finished() const override { return _sent >= _drives.size() + 1; }

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

 private:
  std::vector<std::vector<Drive>> _drives{};
  std::vector<uint16_t> _cycles{};
  GainSTMProps _props;
  size_t _sent{0};

  void pack_phase(TxDatagram& tx) const {
    if (_drives.size() > GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    if (_props.start_idx) {
      if (static_cast<size_t>(_props.start_idx.value()) >= _drives.size()) throw std::runtime_error("STM start index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMStartIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMStartIdx);
    }
    if (_props.finish_idx) {
      if (static_cast<size_t>(_props.finish_idx.value()) >= _drives.size()) throw std::runtime_error("STM finish index out of range");
      tx.header().fpga_flag.set(FPGAControlFlags::UseSTMFinishIdx);
    } else {
      tx.header().fpga_flag.remove(FPGAControlFlags::UseSTMFinishIdx);
    }

    tx.header().cpu_flag.remove(CPUControlFlags::IsDuty);

    if (_sent == 0) {
      if (_props.freq_div < GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN)
        throw std::runtime_error("STM frequency division is out of range. Minimum is" + std::to_string(GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN) +
                                 " but you use " + std::to_string(_props.freq_div));

      tx.header().cpu_flag.set(CPUControlFlags::STMBegin);
      std::for_each(tx.begin(), tx.end(), [this](const auto& body) {
        const auto& [_, d] = body;
        d.gain_stm_initial().freq_div = _props.freq_div;
        d.gain_stm_initial().mode = GainSTMMode::PhaseFull;
        d.gain_stm_initial().cycle = static_cast<uint16_t>(_drives.size());
        d.gain_stm_initial().stm_start_idx = _props.start_idx.value_or(0);
        d.gain_stm_initial().stm_finish_idx = _props.finish_idx.value_or(0);
      });
    } else {
      std::transform(_drives[_sent - 1].begin(), _drives[_sent - 1].end(), _cycles.begin(),
                     reinterpret_cast<AdvancedDrivePhase*>(tx.bodies_raw_ptr()),
                     [](const auto& d, const auto cycle) { return AdvancedDrivePhase(d, cycle); });
    }

    if (_sent + 1 == _drives.size() + 1) tx.header().cpu_flag.set(CPUControlFlags::STMEnd);

    tx.header().cpu_flag.set(CPUControlFlags::WriteBody);

    tx.num_bodies = tx.num_devices();
  }
};

}  // namespace autd3::driver
