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

template <typename H, typename B>
class SpecialData {
 public:
  using header_t = H;
  using body_t = B;

  virtual bool check_trials_override() const = 0;
  virtual size_t check_trials() const = 0;

  H header() { return std::move(_h); }
  B body() { return std::move(_b); }

 protected:
  explicit SpecialData(H h, B b) : _h(std::move(h)), _b(std::move(b)) {}

  H _h;
  B _b;
};

class Stop : public SpecialData<core::SilencerConfig, core::Amplitudes> {
 public:
  Stop() : SpecialData<header_t, body_t>(core::SilencerConfig{}, core::Amplitudes(0.0)) {}

  [[nodiscard]] bool check_trials_override() const override { return false; }
  [[nodiscard]] size_t check_trials() const override { return 0; }
};

class UpdateFlag : public SpecialData<core::NullHeader, core::NullBody> {
 public:
  UpdateFlag() : SpecialData<header_t, body_t>(core::NullHeader{}, core::NullBody{}) {}

  [[nodiscard]] bool check_trials_override() const override { return false; }
  [[nodiscard]] size_t check_trials() const override { return 0; }
};

class Clear : public SpecialData<core::Clear, core::NullBody> {
 public:
  Clear() : SpecialData<header_t, body_t>(core::Clear{}, core::NullBody{}) {}

  [[nodiscard]] bool check_trials_override() const override { return true; }
  [[nodiscard]] size_t check_trials() const override { return 200; }
};

class Synchronize : public SpecialData<core::NullHeader, core::Synchronize> {
 public:
  Synchronize() : SpecialData<header_t, body_t>(core::NullHeader{}, core::Synchronize{}) {}

  [[nodiscard]] bool check_trials_override() const override { return true; }
  [[nodiscard]] size_t check_trials() const override { return 200; }
};

class ModDelayConfig : public SpecialData<core::NullHeader, core::ModDelayConfig> {
 public:
  ModDelayConfig() : SpecialData<header_t, body_t>(core::NullHeader{}, core::ModDelayConfig{}) {}

  [[nodiscard]] bool check_trials_override() const override { return false; }
  [[nodiscard]] size_t check_trials() const override { return 0; }
};

inline autd3::Stop stop() { return autd3::Stop{}; }

inline autd3::UpdateFlag update_flag() { return autd3::UpdateFlag{}; }

inline autd3::Clear clear() { return autd3::Clear{}; }

inline autd3::Synchronize synchronize() { return autd3::Synchronize{}; }

inline autd3::ModDelayConfig mod_delay_config() { return autd3::ModDelayConfig{}; }

}  // namespace autd3
