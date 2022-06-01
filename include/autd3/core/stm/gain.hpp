// File: gain.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/geometry/dynamic_transducer.hpp"
#include "autd3/core/geometry/legacy_transducer.hpp"
#include "autd3/core/geometry/normal_phase_transducer.hpp"
#include "autd3/core/geometry/normal_transducer.hpp"
#include "autd3/core/geometry/transducer.hpp"
#include "autd3/core/interface.hpp"
#include "autd3/driver/cpu/operation.hpp"
#include "stm.hpp"

namespace autd3::core {

#pragma warning(push)
#pragma warning(disable : 26813)

/**
 * @brief GainSTM provides a function to display Gain sequentially and periodically.
 * @details GainSTM uses a timer on the FPGA to ensure that Gain is precisely timed.
 * GainSTM currently has the following three limitations.
 * 1. The maximum number of gains is driver::GAIN_STM_BUF_SIZE_MAX.
 */
template <typename T, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct GainSTM {};

/**
 * @brief GainSTM for LegacyTransducer
 */
template <>
struct GainSTM<LegacyTransducer> final : public STM<LegacyTransducer> {
  explicit GainSTM(const Geometry<LegacyTransducer>& geometry) : STM(), _geometry(geometry), _sent(0), _mode(driver::Mode::PhaseDutyFull) {}

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  template <typename G, std::enable_if_t<std::is_base_of_v<Gain<LegacyTransducer>, G>, nullptr_t> = nullptr>
  void add(G& gain) {
    if (_gains.size() + 1 > driver::GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    gain.build(_geometry);

    _gains.emplace_back(gain.drives());
  }

  driver::Mode& mode() noexcept { return _mode; }

  size_t size() const override { return _gains.size(); }

  void init() override { _sent = 0; }

  void pack(const Geometry<LegacyTransducer>&, driver::TxDatagram& tx) override {
    gain_stm_legacy_header(tx);

    if (is_finished()) return;

    const auto is_first_frame = _sent == 0;

    if (is_first_frame) {
      gain_stm_legacy_body({}, is_first_frame, _freq_div, false, _mode, tx);
      _sent += 1;
      return;
    }

    bool is_last_frame;
    driver::LegacyDrive *p1, *p2, *p3, *p4;
    switch (_mode) {
      case driver::Mode::PhaseDutyFull:
        is_last_frame = _sent + 1 == _gains.size() + 1;
        gain_stm_legacy_body({_gains.at(_sent - 1).data.data()}, is_first_frame, _freq_div, is_last_frame, _mode, tx);
        _sent += 1;
        break;
      case driver::Mode::PhaseFull:
        is_last_frame = _sent + 2 >= _gains.size() + 1;
        p1 = _gains.at(_sent - 1).data.data();
        p2 = _sent + 1 - 1 < _gains.size() ? _gains.at(_sent + 1 - 1).data.data() : nullptr;
        gain_stm_legacy_body({p1, p2}, is_first_frame, _freq_div, is_last_frame, _mode, tx);
        _sent += 2;
        break;
      case driver::Mode::PhaseHalf:
        is_last_frame = _sent + 4 >= _gains.size() + 1;
        p1 = _gains.at(_sent - 1).data.data();
        p2 = _sent + 1 - 1 < _gains.size() ? _gains.at(_sent + 1 - 1).data.data() : nullptr;
        p3 = _sent + 2 - 1 < _gains.size() ? _gains.at(_sent + 2 - 1).data.data() : nullptr;
        p4 = _sent + 3 - 1 < _gains.size() ? _gains.at(_sent + 3 - 1).data.data() : nullptr;
        gain_stm_legacy_body({p1, p2, p3, p4}, is_first_frame, _freq_div, is_last_frame, _mode, tx);
        _sent += 4;
        break;
    }
  }

  [[nodiscard]] bool is_finished() const override { return _sent >= _gains.size() + 1; }

 private:
  const Geometry<LegacyTransducer>& _geometry;
  std::vector<LegacyTransducer::D> _gains;
  size_t _sent;
  driver::Mode _mode;
};

/**
 * @brief GainSTM for NormalTransducer
 */
template <>
struct GainSTM<NormalTransducer> final : public STM<NormalTransducer> {
  explicit GainSTM(const Geometry<NormalTransducer>& geometry)
      : STM(), _geometry(geometry), _sent(0), _next_duty(false), _mode(driver::Mode::PhaseDutyFull) {}

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  template <typename G, std::enable_if_t<std::is_base_of_v<Gain<NormalTransducer>, G>, nullptr_t> = nullptr>
  void add(G& gain) {
    if (_gains.size() + 1 > driver::GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    gain.build(_geometry);

    _gains.emplace_back(gain.drives());
  }

  size_t size() const override { return _gains.size(); }

  driver::Mode& mode() noexcept { return _mode; }

  void init() override { _sent = 0; }

  void pack(const Geometry<NormalTransducer>&, driver::TxDatagram& tx) override {
    gain_stm_normal_header(tx);

    if (is_finished()) return;

    const auto is_first_frame = _sent == 0;
    const auto is_last_frame = _mode == driver::Mode::PhaseDutyFull ? _sent + 1 == _gains.size() * 2 + 1 : _sent + 1 == _gains.size() + 1;

    if (is_first_frame) {
      gain_stm_normal_phase(nullptr, is_first_frame, _freq_div, _mode, is_last_frame, tx);
      _sent += 1;
      return;
    }

    if (!_next_duty) {
      const auto idx = _mode == driver::Mode::PhaseDutyFull ? (_sent - 1) / 2 : _sent - 1;
      gain_stm_normal_phase(_gains.at(idx).phases.data(), is_first_frame, _freq_div, _mode, is_last_frame, tx);
    } else {
      gain_stm_normal_duty(_gains.at((_sent - 1) / 2).duties.data(), is_last_frame, tx);
    }
    if (_mode == driver::Mode::PhaseDutyFull) _next_duty = !_next_duty;

    _sent += 1;
  }

  [[nodiscard]] bool is_finished() const override {
    return _mode == driver::Mode::PhaseDutyFull ? _sent == _gains.size() * 2 + 1 : _sent == _gains.size() + 1;
  }

 private:
  const Geometry<NormalTransducer>& _geometry;
  std::vector<NormalTransducer::D> _gains;
  size_t _sent;
  bool _next_duty;
  driver::Mode _mode;
};

/**
 * @brief GainSTM for NormalPhaseTransducer
 */
template <>
struct GainSTM<NormalPhaseTransducer> final : public STM<NormalPhaseTransducer> {
  explicit GainSTM(const Geometry<NormalPhaseTransducer>& geometry) : STM(), _geometry(geometry), _sent(0) {}

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  template <typename G, std::enable_if_t<std::is_base_of_v<Gain<NormalPhaseTransducer>, G>, nullptr_t> = nullptr>
  void add(G& gain) {
    if (_gains.size() + 1 > driver::GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    gain.build(_geometry);

    _gains.emplace_back(gain.drives());
  }

  size_t size() const override { return _gains.size(); }

  void init() override { _sent = 0; }

  void pack(const Geometry<NormalPhaseTransducer>&, driver::TxDatagram& tx) override {
    gain_stm_normal_header(tx);

    if (is_finished()) return;

    const auto is_first_frame = _sent == 0;
    const auto is_last_frame = _sent + 1 == _gains.size() + 1;

    if (is_first_frame) {
      gain_stm_normal_phase(nullptr, is_first_frame, _freq_div, driver::Mode::PhaseFull, is_last_frame, tx);
      _sent += 1;
      return;
    }

    gain_stm_normal_phase(_gains.at(_sent - 1).phases.data(), is_first_frame, _freq_div, driver::Mode::PhaseFull, is_last_frame, tx);

    _sent += 1;
  }

  [[nodiscard]] bool is_finished() const override { return _sent == _gains.size() + 1; }

 private:
  const Geometry<NormalPhaseTransducer>& _geometry;
  std::vector<NormalPhaseTransducer::D> _gains;
  size_t _sent;
};

/**
 * @brief GainSTM for DynamicTransducer
 */
template <>
struct GainSTM<DynamicTransducer> final : public STM<DynamicTransducer> {
  explicit GainSTM(const Geometry<DynamicTransducer>& geometry)
      : STM(), _geometry(geometry), _sent(0), _next_duty(false), _mode(driver::Mode::PhaseDutyFull) {}

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  template <typename G, std::enable_if_t<std::is_base_of_v<Gain<DynamicTransducer>, G>, nullptr_t> = nullptr>
  void add(G& gain) {
    if (_gains.size() + 1 > driver::GAIN_STM_BUF_SIZE_MAX) throw std::runtime_error("GainSTM out of buffer");

    gain.build(_geometry);

    _gains.emplace_back(gain.drives());
  }

  driver::Mode& mode() noexcept { return _mode; }

  size_t size() const override { return _gains.size(); }

  void init() override { _sent = 0; }

  void pack(const Geometry<DynamicTransducer>&, driver::TxDatagram& tx) override {
    bool is_first_frame, is_last_frame;
    switch (DynamicTransducer::mode()) {
      case TransducerMode::Legacy:
        gain_stm_legacy_header(tx);

        if (is_finished()) return;

        is_first_frame = _sent == 0;

        if (is_first_frame) {
          gain_stm_legacy_body({}, is_first_frame, _freq_div, false, _mode, tx);
          _sent += 1;
          return;
        }

        driver::LegacyDrive *p1, *p2, *p3, *p4;
        switch (_mode) {
          case driver::Mode::PhaseDutyFull:
            is_last_frame = _sent + 1 == _gains.size() + 1;
            gain_stm_legacy_body({_gains.at(_sent - 1).legacy_drives.data()}, is_first_frame, _freq_div, is_last_frame, _mode, tx);
            _sent += 1;
            break;
          case driver::Mode::PhaseFull:
            is_last_frame = _sent + 2 >= _gains.size() + 1;
            p1 = _gains.at(_sent - 1).legacy_drives.data();
            p2 = _sent + 1 - 1 < _gains.size() ? _gains.at(_sent + 1 - 1).legacy_drives.data() : nullptr;
            gain_stm_legacy_body({p1, p2}, is_first_frame, _freq_div, is_last_frame, _mode, tx);
            _sent += 2;
            break;
          case driver::Mode::PhaseHalf:
            is_last_frame = _sent + 4 >= _gains.size() + 1;
            p1 = _gains.at(_sent - 1).legacy_drives.data();
            p2 = _sent + 1 - 1 < _gains.size() ? _gains.at(_sent + 1 - 1).legacy_drives.data() : nullptr;
            p3 = _sent + 2 - 1 < _gains.size() ? _gains.at(_sent + 2 - 1).legacy_drives.data() : nullptr;
            p4 = _sent + 3 - 1 < _gains.size() ? _gains.at(_sent + 3 - 1).legacy_drives.data() : nullptr;
            gain_stm_legacy_body({p1, p2, p3, p4}, is_first_frame, _freq_div, is_last_frame, _mode, tx);
            _sent += 4;
            break;
        }
        break;
      case TransducerMode::Normal:
        gain_stm_normal_header(tx);

        if (is_finished()) return;

        is_first_frame = _sent == 0;
        is_last_frame = _mode == driver::Mode::PhaseDutyFull ? _sent + 1 == _gains.size() * 2 + 1 : _sent + 1 == _gains.size() + 1;

        if (is_first_frame) {
          gain_stm_normal_phase(nullptr, is_first_frame, _freq_div, _mode, is_last_frame, tx);
          _sent += 1;
          return;
        }

        if (!_next_duty) {
          const auto idx = _mode == driver::Mode::PhaseDutyFull ? (_sent - 1) / 2 : _sent - 1;
          gain_stm_normal_phase(_gains.at(idx).phases.data(), is_first_frame, _freq_div, _mode, is_last_frame, tx);
        } else {
          gain_stm_normal_duty(_gains.at((_sent - 1) / 2).duties.data(), is_last_frame, tx);
        }
        if (_mode == driver::Mode::PhaseDutyFull) _next_duty = !_next_duty;

        _sent += 1;
        break;
      case TransducerMode::NormalPhase:
        gain_stm_normal_header(tx);

        if (is_finished()) return;

        is_first_frame = _sent == 0;
        is_last_frame = _sent + 1 == _gains.size() + 1;

        if (is_first_frame) {
          gain_stm_normal_phase(nullptr, is_first_frame, _freq_div, driver::Mode::PhaseFull, is_last_frame, tx);
          _sent += 1;
          return;
        }

        gain_stm_normal_phase(_gains.at(_sent - 1).phases.data(), is_first_frame, _freq_div, driver::Mode::PhaseFull, is_last_frame, tx);

        _sent += 1;
        break;
    }
  }

  [[nodiscard]] bool is_finished() const override {
    switch (DynamicTransducer::mode()) {
      case TransducerMode::Legacy:
        return _sent >= _gains.size() + 1;
      case TransducerMode::Normal:
        return _mode == driver::Mode::PhaseDutyFull ? _sent == _gains.size() * 2 + 1 : _sent == _gains.size() + 1;
      case TransducerMode::NormalPhase:
        return _sent == _gains.size() + 1;
      default:
        return _sent >= _gains.size() + 1;
    }
  }

 private:
  const Geometry<DynamicTransducer>& _geometry;
  std::vector<DynamicTransducer::D> _gains;
  size_t _sent;
  bool _next_duty;
  driver::Mode _mode;
};

#pragma warning(pop)

}  // namespace autd3::core
