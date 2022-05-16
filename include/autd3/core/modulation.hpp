// File: modulation.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <algorithm>
#include <stdexcept>
#include <vector>

#include "autd3/driver/cpu/operation.hpp"
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
  virtual void calc() = 0;

  /**
   * @brief Build modulation data
   */
  void build() {
    if (_props.built) return;
    calc();
    if (buffer().size() > driver::MOD_BUF_SIZE_MAX) throw std::runtime_error("Modulation buffer overflow");
    _props.built = true;
  }

  /**
   * \brief Re-build modulation data
   */
  void rebuild() {
    _props.built = false;
    build();
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

  void init() override {
    build();
    _props.sent = 0;
  }

  void pack(const uint8_t msg_id, driver::TxDatagram& tx) override {
    const auto is_first_frame = _props.sent == 0;
    const auto max_size = is_first_frame ? driver::MOD_HEAD_DATA_SIZE : driver::MOD_BODY_DATA_SIZE;
    const auto mod_size = std::min(buffer().size() - _props.sent, max_size);
    const auto is_last_frame = _props.sent + mod_size == buffer().size();
    const auto* buf = _props.buffer.data() + _props.sent;
    modulation(msg_id, buf, mod_size, is_first_frame, _props.freq_div, is_last_frame, tx);

    _props.sent += mod_size;
  }

  [[nodiscard]] bool is_finished() const noexcept override { return _props.sent == buffer().size(); }

 protected:
  ModProps _props;
};

}  // namespace autd3::core
