// File: holo_gain.hpp
// Project: include
// Created Date: 16/05/2021
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#pragma once

#include <memory>
#include <utility>
#include <vector>

#include "autd3/core/gain.hpp"
#include "backend.hpp"

namespace autd3::gain::holo {

/**
 * @brief Gain to produce multiple focal points
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Holo : public core::Gain<T> {
 public:
  Holo(BackendPtr backend, std::vector<core::Vector3> foci, const std::vector<double>& amps) : _backend(std::move(backend)), _foci(std::move(foci)) {
    if (this->_foci.size() != amps.size()) throw std::runtime_error("The size of foci and amps are not the same");
    this->_amps.reserve(amps.size());
    for (const auto amp : amps) this->_amps.emplace_back(complex(amp, 0.0));
  }
  ~Holo() override = default;
  Holo(const Holo& v) noexcept = delete;
  Holo& operator=(const Holo& obj) = delete;
  Holo(Holo&& obj) = default;
  Holo& operator=(Holo&& obj) = default;

  [[nodiscard]] const std::vector<core::Vector3>& foci() const { return this->_foci; }
  [[nodiscard]] const std::vector<complex>& amplitudes() const { return this->_amps; }

 protected:
  BackendPtr _backend;
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
   * @param[in] foci focal points
   * @param[in] amps amplitudes of the foci
   * @param[in] alpha parameter
   * @param[in] lambda parameter
   * @param[in] repeat parameter
   * @param[in] normalize parameter
   */
  explicit SDP(BackendPtr backend, const std::vector<core::Vector3>& foci, const std::vector<double>& amps, const double alpha = 1e-3,
               const double lambda = 0.9, const size_t repeat = 100, const bool normalize = true)
      : Holo(std::move(backend), foci, amps), _alpha(alpha), _lambda(lambda), _repeat(repeat), _normalize(normalize) {}

  void calc(const core::Geometry<T>& geometry) override;

 private:
  double _alpha;
  double _lambda;
  size_t _repeat;
  bool _normalize;
};

}  // namespace autd3::gain::holo
