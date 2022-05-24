// File: holo.hpp
// Project: gain
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <memory>
#include <utility>
#include <variant>
#include <vector>

#include "autd3/core/gain.hpp"
#include "backend.hpp"

namespace autd3::gain::holo {

/**
 * @brief AmplitudeConstraint to do nothing
 */
struct DontCare final {
  static double convert(const double raw, const double) { return raw; }
};

/**
 * @brief AmplitudeConstraint to normalize to the largest amplitude
 */
struct Normalize final {
  static double convert(const double raw, const double max) { return raw / max; }
};

/**
 * @brief AmplitudeConstraint to give the same amplitude to all transducers
 */
struct Uniform final {
  explicit Uniform(const double value) : _value(value) {}

  [[nodiscard]] double convert(const double, const double) const { return _value; }

 private:
  double _value;
};

/**
 * @brief AmplitudeConstraint to clamp amplitude in [0, 1]
 */
struct Clamp final {
  [[nodiscard]] double convert(const double raw, const double) const { return std::clamp(raw, 0.0, 1.0); }
};

/**
 * @brief Amplitude constraint
 */
using AmplitudeConstraint = std::variant<DontCare, Normalize, Uniform, Clamp>;

/**
 * @brief Gain to produce multiple focal points
 */
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Holo : public core::Gain<T> {
 public:
  explicit Holo(BackendPtr backend, const AmplitudeConstraint constraint = Normalize()) : constraint(constraint), _backend(std::move(backend)) {}
  ~Holo() override = default;
  Holo(const Holo& v) noexcept = delete;
  Holo& operator=(const Holo& obj) = delete;
  Holo(Holo&& obj) = default;
  Holo& operator=(Holo&& obj) = default;

  /**
   * @brief Add focus position and amplitude of focus
   */
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
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class SDP final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit SDP(BackendPtr backend) : Holo<T>(std::move(backend), Normalize()), alpha(1e-3), lambda(0.9), repeat(100) {}

  void calc(const core::Geometry<T>& geometry) override;

  double alpha;
  double lambda;
  size_t repeat;
};

/**
 * @brief Gain to produce multiple focal points with EVD method.
 * Refer to Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air
 * using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.
 */
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class EVD final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit EVD(BackendPtr backend) : Holo<T>(std::move(backend), Uniform(1.0)), gamma(1.0) {}

  void calc(const core::Geometry<T>& geometry) override;

  double gamma;
};

/**
 * @brief Gain to produce multiple focal points with naive method.
 */
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Naive final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit Naive(BackendPtr backend) : Holo<T>(std::move(backend), Normalize()) {}

  void calc(const core::Geometry<T>& geometry) override;
};

/**
 * @brief Gain to produce multiple focal points with GS method.
 * Refer to Asier Marzo and Bruce W Drinkwater, "Holographic acoustic
 * tweezers," Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
 */
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class GS final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GS(BackendPtr backend) : Holo<T>(std::move(backend), Normalize()), repeat(100) {}

  void calc(const core::Geometry<T>& geometry) override;

  size_t repeat;
};

/**
 * @brief Gain to produce multiple focal points with GS-PAT method (not yet been implemented with GPU).
 * Refer to Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point
 * sound-fields for phased arrays of transducers," ACMTrans-actions on
 * Graphics (TOG), 39(4):138–1, 2020.
 */
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class GSPAT final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GSPAT(BackendPtr backend) : Holo<T>(std::move(backend), Normalize()), repeat(100) {}

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
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class LM final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit LM(BackendPtr backend) : Holo<T>(std::move(backend)), eps_1(1e-8), eps_2(1e-8), tau(1e-3), k_max(5) {}

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
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class GaussNewton final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GaussNewton(BackendPtr backend) : Holo<T>(std::move(backend)), eps_1(1e-6), eps_2(1e-6), k_max(500) {}

  void calc(const core::Geometry<T>& geometry) override;

  double eps_1;
  double eps_2;
  size_t k_max;
  std::vector<double> initial;
};

/**
 * @brief Gain to produce multiple focal points with Gauss-Newton method.
 */
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class GradientDescent final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit GradientDescent(BackendPtr backend) : Holo<T>(std::move(backend)), eps(1e-6), step(0.5), k_max(2000) {}

  void calc(const core::Geometry<T>& geometry) override;

  double eps;
  double step;
  size_t k_max;
  std::vector<double> initial;
};

/**
 * @brief Gain to produce multiple focal points with Greedy algorithm.
 * Refer to Shun suzuki, et al. “Radiation Pressure Field Reconstruction for Ultrasound Midair Haptics by Greedy Algorithm with Brute-Force Search,”
 * in IEEE Transactions on Haptics, doi: 10.1109/TOH.STM.3076489
 * @details This method is computed on the CPU.
 */
template <typename T = core::DynamicTransducer, std::enable_if_t<std::is_base_of_v<core::Transducer<typename T::D>, T>, nullptr_t> = nullptr>
class Greedy final : public Holo<T> {
 public:
  /**
   * @param[in] backend pointer to Backend
   */
  explicit Greedy(BackendPtr backend)
      : Holo<T>(std::move(backend)),
        phase_div(16),
        objective([](const VectorXd& target, const VectorXc& p) { return (target - p.cwiseAbs()).cwiseAbs().sum(); }) {}

  void calc(const core::Geometry<T>& geometry) override;

  size_t phase_div;
  std::function<double(const VectorXd&, const VectorXc&)> objective;
};

}  // namespace autd3::gain::holo
