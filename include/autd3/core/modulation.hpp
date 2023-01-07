// File: modulation.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/modulation.hpp"

namespace autd3::core {

/**
 * @brief Modulation controls the amplitude modulation
 */
class Modulation : public DatagramHeader {
 public:
  Modulation() = default;
  ~Modulation() override = default;
  Modulation(const Modulation& v) noexcept = default;
  Modulation& operator=(const Modulation& obj) = default;
  Modulation(Modulation&& obj) = default;
  Modulation& operator=(Modulation&& obj) = default;

  /**
   * \brief Calculate modulation data
   */
  virtual bool calc() = 0;

  /**
   * @brief Build modulation data
   */
  [[nodiscard]] bool build() {
    if (_built) return true;
    _op.mod_data.clear();
    _built = true;
    return calc();
  }

  /**
   * \brief Re-build modulation data
   */
  [[nodiscard]] bool rebuild() {
    _built = false;
    return build();
  }

  /**
   * \brief modulation data
   */
  [[nodiscard]] const std::vector<uint8_t>& buffer() const noexcept { return _op.mod_data; }

  /**
   * @brief [Advanced] modulation data
   * @details Call Modulation::build before using this function to initialize buffer data.
   */
  std::vector<uint8_t>& buffer() noexcept { return _op.mod_data; }

  /**
   * \brief sampling frequency division ratio
   */
  uint32_t& sampling_frequency_division() noexcept { return _op.freq_div; }

  /**
   * \brief sampling frequency division ratio
   */
  [[nodiscard]] uint32_t sampling_frequency_division() const noexcept { return _op.freq_div; }

  /**
   * \brief modulation sampling frequency
   */
  [[nodiscard]] driver::autd3_float_t sampling_frequency() const noexcept {
    return static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / static_cast<driver::autd3_float_t>(_op.freq_div);
  }

  /**
   * \brief Set modulation sampling frequency
   */
  [[nodiscard]] driver::autd3_float_t set_sampling_frequency(const driver::autd3_float_t freq) {
    _op.freq_div = static_cast<uint32_t>(std::round(static_cast<driver::autd3_float_t>(driver::FPGA_CLK_FREQ) / freq));
    return sampling_frequency();
  }

  bool init() override {
    _op.init();
    return build();
  }

  void pack(driver::TxDatagram& tx) override { _op.pack(tx); }

  [[nodiscard]] bool is_finished() const noexcept override { return _op.is_finished(); }

 protected:
  bool _built{false};
  driver::Modulation _op;
};

}  // namespace autd3::core
