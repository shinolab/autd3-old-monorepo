// File: special_data.hpp
// Project: autd3
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/11/2022
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

class SpecialData {
 public:
  virtual bool check_trials_override() const = 0;
  virtual size_t check_trials() const = 0;
  virtual ~SpecialData() = default;

  std::unique_ptr<core::DatagramHeader> header() { return std::move(_h); }
  std::unique_ptr<core::DatagramBody> body() { return std::move(_b); }

 protected:
  explicit SpecialData(std::unique_ptr<core::DatagramHeader> h, std::unique_ptr<core::DatagramBody> b) : _h(std::move(h)), _b(std::move(b)) {}

  std::unique_ptr<core::DatagramHeader> _h;
  std::unique_ptr<core::DatagramBody> _b;
};

class Stop : public SpecialData {
 public:
  Stop() : SpecialData(std::make_unique<core::SilencerConfig>(), std::make_unique<core::Amplitudes>(0.0)) {}

  [[nodiscard]] bool check_trials_override() const override { return false; }
  [[nodiscard]] size_t check_trials() const override { return 0; }
};

class UpdateFlag : public SpecialData {
 public:
  UpdateFlag() : SpecialData(std::make_unique<core::NullHeader>(), std::make_unique<core::NullBody>()) {}

  [[nodiscard]] bool check_trials_override() const override { return false; }
  [[nodiscard]] size_t check_trials() const override { return 0; }
};

class Clear : public SpecialData {
 public:
  Clear() : SpecialData(std::make_unique<core::Clear>(), std::make_unique<core::NullBody>()) {}

  [[nodiscard]] bool check_trials_override() const override { return true; }
  [[nodiscard]] size_t check_trials() const override { return 200; }
};

class Synchronize : public SpecialData {
 public:
  Synchronize() : SpecialData(std::make_unique<core::NullHeader>(), std::make_unique<core::Synchronize>()) {}

  [[nodiscard]] bool check_trials_override() const override { return true; }
  [[nodiscard]] size_t check_trials() const override { return 200; }
};

class ModDelayConfig : public SpecialData {
 public:
  ModDelayConfig() : SpecialData(std::make_unique<core::NullHeader>(), std::make_unique<core::ModDelayConfig>()) {}

  [[nodiscard]] bool check_trials_override() const override { return false; }
  [[nodiscard]] size_t check_trials() const override { return 0; }
};

inline autd3::Stop stop() { return autd3::Stop{}; }

inline autd3::UpdateFlag update_flag() { return autd3::UpdateFlag{}; }

inline autd3::Clear clear() { return autd3::Clear{}; }

inline autd3::Synchronize synchronize() { return autd3::Synchronize{}; }

inline autd3::ModDelayConfig mod_delay_config() { return autd3::ModDelayConfig{}; }

}  // namespace autd3
