// File: gain.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <memory>
#include <utility>
#include <vector>

#include "autd3/core/gain.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/core/stm/stm.hpp"
#include "autd3/driver/operation/gain_stm.hpp"

namespace autd3::core {

/**
 * @brief GainSTM provides a function to display Gain sequentially and periodically.
 * @details GainSTM uses a timer on the FPGA to ensure that Gain is precisely timed.
 */
struct GainSTM final : public STM {
  explicit GainSTM(driver::GainSTMMode mode = driver::GainSTMMode::PhaseDutyFull) : STM(), _mode(mode) {}

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  template <typename G>
  void add(G&& gain) {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.emplace_back(std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain)));
  }

  /**
   * @brief Add gain
   * @param[in] gain gain
   */
  void add(std::shared_ptr<Gain> gain) { _gains.emplace_back(std::move(gain)); }

  [[nodiscard]] size_t size() const override { return _gains.size(); }

  std::unique_ptr<driver::Operation> operation(const Geometry& geometry) override {
    std::vector<std::vector<driver::Drive>> drives;
    drives.reserve(_gains.size());
    std::transform(_gains.begin(), _gains.end(), std::back_inserter(drives), [geometry](const auto& gain) { return gain->calc(geometry); });
    const driver::GainSTMProps props{sampling_frequency_division, _mode, start_idx, finish_idx};
    switch (geometry.mode) {
      case Mode::Legacy:
        return std::make_unique<driver::GainSTM<driver::Legacy>>(std::move(drives), props);
      case Mode::Advanced:
        return std::make_unique<driver::GainSTM<driver::Advanced>>(std::move(drives), geometry.cycles(), props);
      case Mode::AdvancedPhase:
        return std::make_unique<driver::GainSTM<driver::AdvancedPhase>>(std::move(drives), geometry.cycles(), props);
    }
    throw std::runtime_error("Unreachable!");
  }

 private:
  driver::GainSTMMode _mode;
  std::vector<std::shared_ptr<Gain>> _gains;
};

}  // namespace autd3::core
