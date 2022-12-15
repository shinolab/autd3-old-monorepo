// File: gain.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/gain.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/driver/driver.hpp"
#include "stm.hpp"

namespace autd3::core {

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26813)
#endif

/**
 * @brief GainSTM provides a function to display Gain sequentially and periodically.
 * @details GainSTM uses a timer on the FPGA to ensure that Gain is precisely timed.
 */
struct GainSTM final : STM {
  explicit GainSTM(const Geometry& geometry) : STM(), _geometry(geometry), _sent(0), _next_duty(false), _mode(driver::GainSTMMode::PhaseDutyFull) {}

  /**
   * @brief Set frequency of the STM
   * @param[in] freq Frequency of the STM
   * @details STM mode has some constraints, which determine the actual frequency of the STM.
   * @return double Actual frequency of STM
   */
  double set_frequency(const double freq) override {
    const auto sample_freq = static_cast<double>(size()) * freq;
    _freq_div = static_cast<uint32_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / sample_freq));
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

  [[nodiscard]] size_t size() const override { return _gains.size(); }

  bool init() override {
    _sent = 0;
    return true;
  }

  bool pack(const std::unique_ptr<const driver::Driver>& driver, const std::unique_ptr<const Mode>& mode, const Geometry&,
            driver::TxDatagram& tx) override {
    mode->pack_stm_gain_header(driver, tx);

    if (is_finished()) return true;

    return mode->pack_stm_gain_body(driver, _sent, _next_duty, _freq_div, _gains, _mode, start_idx, tx);
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
