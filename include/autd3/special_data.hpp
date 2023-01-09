// File: special_data.hpp
// Project: autd3
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/core/amplitudes.hpp>
#include <autd3/core/clear.hpp>
#include <autd3/core/delay.hpp>
#include <autd3/core/silencer_config.hpp>
#include <autd3/core/synchronize.hpp>

namespace autd3 {

/**
 * @brief Structure with DatagramHeader and DatagramBody for performing special operations
 */
class SpecialData {
 public:
  [[nodiscard]] virtual bool ack_check_timeout_override() const = 0;
  [[nodiscard]] virtual std::chrono::high_resolution_clock::duration ack_check_timeout() const = 0;
  virtual ~SpecialData() = default;
  SpecialData(const SpecialData& v) noexcept = delete;
  SpecialData& operator=(const SpecialData& obj) = delete;
  SpecialData(SpecialData&& obj) = default;
  SpecialData& operator=(SpecialData&& obj) = default;

  std::unique_ptr<core::DatagramHeader> header() { return std::move(_h); }
  std::unique_ptr<core::DatagramBody> body() { return std::move(_b); }

 protected:
  explicit SpecialData(std::unique_ptr<core::DatagramHeader> h, std::unique_ptr<core::DatagramBody> b) : _h(std::move(h)), _b(std::move(b)) {}

  std::unique_ptr<core::DatagramHeader> _h;
  std::unique_ptr<core::DatagramBody> _b;
};

/**
 * @brief SpecialData to stop ultrasound output
 */
class Stop final : public SpecialData {
 public:
  Stop() : SpecialData(std::make_unique<core::SilencerConfig>(), std::make_unique<core::Amplitudes>(driver::autd3_float_t{0})) {}

  [[nodiscard]] bool ack_check_timeout_override() const override { return false; }
  [[nodiscard]] std::chrono::high_resolution_clock::duration ack_check_timeout() const override {
    return std::chrono::high_resolution_clock::duration::zero();
  }
};

/**
 * @brief SpecialData to update control flags
 */
class UpdateFlag final : public SpecialData {
 public:
  UpdateFlag() : SpecialData(std::make_unique<core::NullHeader>(), std::make_unique<core::NullBody>()) {}

  [[nodiscard]] bool ack_check_timeout_override() const override { return false; }
  [[nodiscard]] std::chrono::high_resolution_clock::duration ack_check_timeout() const override {
    return std::chrono::high_resolution_clock::duration::zero();
  }
};

/**
 * @brief SpecialData for clear
 */
class Clear final : public SpecialData {
 public:
  Clear() : SpecialData(std::make_unique<core::Clear>(), std::make_unique<core::NullBody>()) {}

  [[nodiscard]] bool ack_check_timeout_override() const override { return true; }
  [[nodiscard]] std::chrono::high_resolution_clock::duration ack_check_timeout() const override {
    return std::chrono::nanoseconds(200 * 1000 * 1000);
  }
};

/**
 * @brief SpecialData for synchronization
 */
class Synchronize final : public SpecialData {
 public:
  Synchronize() : SpecialData(std::make_unique<core::NullHeader>(), std::make_unique<core::Synchronize>()) {}

  [[nodiscard]] bool ack_check_timeout_override() const override { return true; }
  [[nodiscard]] std::chrono::high_resolution_clock::duration ack_check_timeout() const override {
    return std::chrono::nanoseconds(200 * 1000 * 1000);
  }
};

/**
 * @brief SpecialData for modulation delay configuration
 */
class ModDelayConfig final : public SpecialData {
 public:
  ModDelayConfig() : SpecialData(std::make_unique<core::NullHeader>(), std::make_unique<core::ModDelayConfig>()) {}

  [[nodiscard]] bool ack_check_timeout_override() const override { return false; }
  [[nodiscard]] std::chrono::high_resolution_clock::duration ack_check_timeout() const override {
    return std::chrono::high_resolution_clock::duration::zero();
  }
};

inline Stop stop() { return Stop{}; }

inline UpdateFlag update_flag() { return UpdateFlag{}; }

inline Clear clear() { return Clear{}; }

inline Synchronize synchronize() { return Synchronize{}; }

inline ModDelayConfig mod_delay_config() { return ModDelayConfig{}; }

}  // namespace autd3
