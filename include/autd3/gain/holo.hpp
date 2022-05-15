// File: holo_gain.hpp
// Project: include
// Created Date: 16/05/2021
// Author: Shun Suzuki
// -----
// Last Modified: 15/05/2022
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
  explicit Holo(BackendPtr backend) : constraint(Normalize()), _backend(std::move(backend)) {}
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
   */
  explicit SDP(BackendPtr backend) : Holo(std::move(backend)), alpha(1e-3), lambda(0.9), repeat(100) {}

  void calc(const core::Geometry<T>& geometry) override;

  double alpha;
  double lambda;
  size_t repeat;
};

/**
 * @brief Gain to produce multiple focal points with naive method.
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Naive final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit Naive(BackendPtr backend) : Holo(std::move(backend)) {}

  void calc(const core::Geometry<T>& geometry) override;
};

/**
 * @brief Gain to produce multiple focal points with GS method.
 * Refer to Asier Marzo and Bruce W Drinkwater, "Holographic acoustic
 * tweezers," Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class GS final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GS(BackendPtr backend) : Holo(std::move(backend)), repeat(100) {}

  void calc(const core::Geometry<T>& geometry) override;

  size_t repeat;
};

/**
 * @brief Gain to produce multiple focal points with GS-PAT method (not yet been implemented with GPU).
 * Refer to Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point
 * sound-fields for phased arrays of transducers," ACMTrans-actions on
 * Graphics (TOG), 39(4):138–1, 2020.
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class GSPAT final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GSPAT(BackendPtr backend) : Holo(std::move(backend)), repeat(100) {}

  void calc(const core::Geometry<T>& geometry) override;

  size_t repeat;
};

/**
 * @brief Gain to produce multiple focal points with Levenberg-Marquardt method.
 * Refer to K.Levenberg, “A method for the solution of certain non-linear problems in
 * least squares,” Quarterly of applied mathematics, vol.2, no.2, pp.164–168, 1944.
 * D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and
 * AppliedMathematics, vol.11, no.2, pp.431–441, 1963.
 * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class LM final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit LM(BackendPtr backend) : Holo(std::move(backend)), eps_1(1e-8), eps_2(1e-8), tau(1e-3), k_max(5) {}

  void calc(const core::Geometry<T>& geometry) override;

  double eps_1;
  double eps_2;
  double tau;
  size_t k_max;
  std::vector<double> initial;
};

/**
 * @brief Gain to produce multiple focal points with Gauss-Newton method.
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class GaussNewton final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GaussNewton(BackendPtr backend) : Holo(std::move(backend)), eps_1(1e-6), eps_2(1e-6), k_max(500) {}

  void calc(const core::Geometry<T>& geometry) override;

  double eps_1;
  double eps_2;
  size_t k_max;
  std::vector<double> initial;
};

/**
 * @brief Gain to produce multiple focal points with Gauss-Newton method.
 */
template <typename T = core::LegacyTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class GradientDescent final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GradientDescent(BackendPtr backend) : Holo(std::move(backend)), eps(1e-6), step(0.5), k_max(2000) {}

  void calc(const core::Geometry<T>& geometry) override;

  double eps;
  double step;
  size_t k_max;
  std::vector<double> initial;
};

}  // namespace autd3::gain::holo
