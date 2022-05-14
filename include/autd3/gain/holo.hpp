// File: holo_gain.hpp
// Project: include
// Created Date: 16/05/2021
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include <memory>
#include <utility>
#include <variant>
#include <vector>

#include "autd3/core/gain.hpp"
#include "backend.hpp"

namespace autd3::gain::holo {

struct DontCare final {
  static double convert(const double raw, const double) { return raw; }
};

struct Normalize final {
  static double convert(const double raw, const double max) { return raw / max; }
};

struct Uniform final {
  explicit Uniform(const double value) : _value(value) {}

  [[nodiscard]] double convert(const double, const double) const { return _value; }

 private:
  double _value;
};

struct Clamp final {
  [[nodiscard]] double convert(const double raw, const double) const { return std::clamp(raw, 0.0, 1.0); }
};

using AmplitudeConstraint = std::variant<DontCare, Normalize, Uniform, Clamp>;

/**
 * @brief Gain to produce multiple focal points
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Holo : public core::Gain<T> {
 public:
  explicit Holo(BackendPtr<T> backend) : constraint(Normalize()), _backend(std::move(backend)) {}
  ~Holo() override = default;
  Holo(const Holo& v) noexcept = delete;
  Holo& operator=(const Holo& obj) = delete;
  Holo(Holo&& obj) = default;
  Holo& operator=(Holo&& obj) = default;

  void add_focus(const core::Vector3& focus, const double amp) {
    _foci.emplace_back(focus);
    _amps.emplace_back(complex(amp, 0.0));
  }

  [[nodiscard]] const std::vector<core::Vector3>& foci() const { return this->_foci; }
  [[nodiscard]] const std::vector<complex>& amplitudes() const { return this->_amps; }

  AmplitudeConstraint constraint;

 protected:
  BackendPtr<T> _backend;
  std::vector<core::Vector3> _foci;
  std::vector<complex> _amps;
};

/**
 * @brief Gain to produce multiple focal points with SDP method.
 * Refer to Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch
 * perception produced by airborne ultrasonic haptic hologram." 2015 IEEE
 * World Haptics Conference (WHC). IEEE, 2015.
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class SDP final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit SDP(BackendPtr<T> backend) : Holo(std::move(backend)), alpha(1e-3), lambda(0.9), repeat(100) {}

  void calc(const core::Geometry<T>& geometry) override;

  double alpha;
  double lambda;
  size_t repeat;
};

}  // namespace autd3::gain::holo
