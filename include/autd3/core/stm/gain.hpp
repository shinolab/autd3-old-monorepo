// File: gain.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 31/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <limits>
#include <vector>

#include "autd3/core/interface.hpp"
#include "autd3/driver/cpu/operation.hpp"
#include "stm.hpp"

namespace autd3::core {

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26813)
#endif

/**
 * @brief GainSTM provides a function to display Gain sequentially and periodically.
 * @details GainSTM uses a timer on the FPGA to ensure that Gain is precisely timed.
 * GainSTM currently has the following three limitations.
 * 1. The maximum number of gains is driver::GAIN_STM_BUF_SIZE_MAX in normal mode and driver::GAIN_STM_LEGACY_BUF_SIZE_MAX in legacy mode.
 */
struct GainSTM final : public STM {
  explicit GainSTM(const Geometry& geometry) : STM(), _geometry(geometry), _sent(0), _next_duty(false), _mode(driver::GainSTMMode::PhaseDutyFull) {}

  /**
   * @brief Set frequency of the STM
   * @param[in] freq Frequency of the STM
   * @details STM mode has some constraints, which determine the actual frequency of the STM.
   * @return double Actual frequency of STM
   */
  double set_frequency(const double freq) override {
    const auto sample_freq = static_cast<double>(size()) * freq;
    const auto div = std::clamp(static_cast<uint32_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / sample_freq)),
                                _geometry.mode()->gain_stm_div_min(), std::numeric_limits<uint32_t>::max());
    _freq_div = div;
    return frequency();
  }

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  template <typename G, std::enable_if_t<std::is_base_of_v<Gain, G>, nullptr_t> = nullptr>
  void add(G& gain) {
    gain.build(_geometry);
    _gains.emplace_back(gain.drives());
  }

  driver::GainSTMMode& mode() noexcept { return _mode; }

  size_t size() const override { return _gains.size(); }

  void init() override { _sent = 0; }

  void pack(const Geometry& geometry, driver::TxDatagram& tx) override {
    geometry.mode()->pack_stm_gain_header(tx);

    if (is_finished()) return;

    geometry.mode()->pack_stm_gain_body(_sent, _next_duty, _freq_div, _gains, _mode, tx);
  }

  [[nodiscard]] bool is_finished() const override { return _sent >= _gains.size() + 1; }

 private:
  const Geometry& _geometry;
  std::vector<std::vector<driver::Drive>> _gains;
  size_t _sent;
  bool _next_duty;
  driver::GainSTMMode _mode;
};

#ifdef _MSC_VER
#pragma warning(pop)
#endif

}  // namespace autd3::core
