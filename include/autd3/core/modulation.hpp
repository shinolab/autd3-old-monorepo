// File: modulation.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <stdexcept>
#include <vector>

#include "autd3/driver/driver.hpp"
#include "interface.hpp"

namespace autd3::core {

/**
 * @brief Properties of Modulation
 */
struct ModProps {
  ModProps() noexcept : freq_div(40960), built(false), sent(0) {}

  std::vector<uint8_t> buffer;
  uint32_t freq_div;
  bool built;
  size_t sent;
};

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
  bool build() {
    if (_props.built) return true;
    _props.built = true;
    return calc();
  }

  /**
   * \brief Re-build modulation data
   */
  bool rebuild() {
    _props.built = false;
    return build();
  }

  /**
   * \brief modulation data
   */
  [[nodiscard]] const std::vector<uint8_t>& buffer() const noexcept { return _props.buffer; }

  /**
   * \brief sampling frequency division ratio
   * \details sampling frequency will be driver::FPGA_CLK_FREQ /(sampling frequency division ratio).
   * The value must be larger than driver::MOD_SAMPLING_FREQ_DIV_MIN.
   */
  uint32_t& sampling_frequency_division() noexcept { return _props.freq_div; }

  /**
   * \brief sampling frequency division ratio
   * \details sampling frequency will be driver::FPGA_CLK_FREQ /(sampling frequency division ratio).
   */
  [[nodiscard]] uint32_t sampling_frequency_division() const noexcept { return _props.freq_div; }

  /**
   * \brief modulation sampling frequency
   */
  [[nodiscard]] double sampling_frequency() const noexcept {
    return static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(_props.freq_div);
  }

  bool init() override {
    _props.sent = 0;
    return build();
  }

  bool pack(const std::unique_ptr<const driver::Driver>& driver, const uint8_t msg_id, driver::TxDatagram& tx) override {
    return driver->modulation(msg_id, buffer(), _props.sent, _props.freq_div, tx);
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _props.sent == buffer().size(); }

 protected:
  ModProps _props;
};

}  // namespace autd3::core
