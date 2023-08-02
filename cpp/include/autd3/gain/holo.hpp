// File: holo.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <variant>
#include <vector>

#include "autd3/internal/gain.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain::holo {

/**
 * @brief Calculation backend
 */
class Backend {
 public:
  Backend() noexcept : _ptr(internal::native_methods::BackendPtr{nullptr}){};
  explicit Backend(internal::native_methods::BackendPtr ptr) noexcept : _ptr(ptr){};
  Backend(const Backend& obj) = default;
  Backend& operator=(const Backend& obj) = default;
  Backend(Backend&& obj) = default;
  Backend& operator=(Backend&& obj) = default;
  virtual ~Backend() { internal::native_methods::AUTDDeleteBackend(_ptr); }

  [[nodiscard]] internal::native_methods::BackendPtr ptr() { return _ptr; }

 protected:
  internal::native_methods::BackendPtr _ptr;
};

/**
 * @brief Backend using [Nalgebra](https://nalgebra.org/)
 */
class DefaultBackend final : public Backend {
 public:
  DefaultBackend() : Backend(internal::native_methods::AUTDDefaultBackend()) {}
  ~DefaultBackend() override = default;
};

/**
 * @brief Amplitude constraint
 */
class AmplitudeConstraint {
 public:
  /**
   * @brief Do nothing (this is equivalent to `Clamp(0, 1)`)
   */
  static AmplitudeConstraint dont_care() { return AmplitudeConstraint{internal::native_methods::AUTDGainHoloDotCareConstraint()}; }

  /**
   * @brief Normalize the value by dividing the maximum value
   */
  static AmplitudeConstraint normalize() { return AmplitudeConstraint{internal::native_methods::AUTDGainHoloNormalizeConstraint()}; }

  /**
   * @brief Set all amplitudes to the specified value
   * @param value amplitude
   */
  static AmplitudeConstraint uniform(const double value) {
    return AmplitudeConstraint{internal::native_methods::AUTDGainHoloUniformConstraint(value)};
  }

  /**
   * @brief Clamp all amplitudes to the specified range
   *
   * @param min_v minimum amplitude
   * @param max_v maximum amplitude
   */
  static AmplitudeConstraint clamp(const double min_v, const double max_v) {
    return AmplitudeConstraint{internal::native_methods::AUTDGainHoloClampConstraint(min_v, max_v)};
  }

  [[nodiscard]] internal::native_methods::ConstraintPtr ptr() const { return _ptr; }

 private:
  explicit AmplitudeConstraint(const internal::native_methods::ConstraintPtr ptr) : _ptr(ptr) {}

  internal::native_methods::ConstraintPtr _ptr;
};

/**
 * @brief Multi-focus gain
 */
class Holo : public internal::Gain {
 public:
  Holo() : _backend(std::make_shared<DefaultBackend>()) {}
  explicit Holo(std::shared_ptr<Backend> backend) : _backend(std::move(backend)) {}

  Holo(const Holo& obj) = default;
  Holo& operator=(const Holo& obj) = default;
  Holo(Holo&& obj) = default;
  Holo& operator=(Holo&& obj) = default;
  ~Holo() override = default;

  void add_focus(internal::Vector3 focus, double amp) {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
  }

 protected:
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
  std::shared_ptr<Backend> _backend;
};

/**
 * @brief Gain to produce multiple foci by solving Semi-Denfinite Programming
 *
 * @details Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE
 * World Haptics Conference (WHC). IEEE, 2015.
 */
class SDP final : public Holo {
 public:
  SDP() = default;
  explicit SDP(std::shared_ptr<Backend> backend) : Holo(std::move(backend)) {}

  /**
   * @brief Parameter. See the paper for details.
   */
  SDP with_alpha(const double value) {
    _alpha = value;
    return std::move(*this);
  }

  /**
   * @brief Parameter. See the paper for details.
   */
  SDP with_repeat(const uint32_t value) {
    _repeat = value;
    return std::move(*this);
  }

  /**
   * @brief Parameter. See the paper for details.
   */
  SDP with_lambda(const double value) {
    _lambda = value;
    return std::move(*this);
  }

  /**
   * @brief Set amplitude constraint
   */
  SDP with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = AUTDGainHoloSDP(_backend->ptr(), reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_alpha.has_value()) ptr = AUTDGainHoloSDPWithAlpha(ptr, _alpha.value());
    if (_repeat.has_value()) ptr = AUTDGainHoloSDPWithRepeat(ptr, _repeat.value());
    if (_lambda.has_value()) ptr = AUTDGainHoloSDPWithLambda(ptr, _lambda.value());
    if (_constraint.has_value()) ptr = AUTDGainHoloSDPWithConstraint(ptr, _constraint.value().ptr());
    return ptr;
  }

 private:
  std::optional<double> _alpha;
  std::optional<uint32_t> _repeat;
  std::optional<double> _lambda;
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci by solving Eigen Value Problem
 *
 * @details Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014):
 * 1-10.
 */
class EVP final : public Holo {
 public:
  EVP() = default;
  explicit EVP(std::shared_ptr<Backend> backend) : Holo(std::move(backend)) {}

  /**
   * @brief Parameter. See the paper for details.
   */
  EVP with_gamma(const double value) {
    _gamma = value;
    return std::move(*this);
  }

  /**
   * @brief Set amplitude constraint
   */
  EVP with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = AUTDGainHoloEVP(_backend->ptr(), reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_gamma.has_value()) ptr = AUTDGainHoloEVPWithGamma(ptr, _gamma.value());
    if (_constraint.has_value()) ptr = AUTDGainHoloEVPWithConstraint(ptr, _constraint.value().ptr());
    return ptr;
  }

 private:
  std::optional<double> _gamma;
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci with GS algorithm
 *
 * @details Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
 */
class GS final : public Holo {
 public:
  GS() = default;
  explicit GS(std::shared_ptr<Backend> backend) : Holo(std::move(backend)) {}

  /**
   * @brief Parameter. See the paper for details.
   */
  GS with_repeat(const uint32_t value) {
    _repeat = value;
    return std::move(*this);
  }

  /**
   * @brief Set amplitude constraint
   */
  GS with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = AUTDGainHoloGS(_backend->ptr(), reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_repeat.has_value()) ptr = AUTDGainHoloGSWithRepeat(ptr, _repeat.value());
    if (_constraint.has_value()) ptr = AUTDGainHoloGSWithConstraint(ptr, _constraint.value().ptr());
    return ptr;
  }

 private:
  std::optional<uint32_t> _repeat;
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci with GS-PAT algorithm
 *
 * @details Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on
 * Graphics (TOG), 39(4):138–1, 2020.
 */
class GSPAT final : public Holo {
 public:
  GSPAT() = default;
  explicit GSPAT(std::shared_ptr<Backend> backend) : Holo(std::move(backend)) {}

  /**
   * @brief Parameter. See the paper for details.
   */
  GSPAT with_repeat(const uint32_t value) {
    _repeat = value;
    return std::move(*this);
  }

  /**
   * @brief Set amplitude constraint
   */
  GSPAT with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = AUTDGainHoloGSPAT(_backend->ptr(), reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_repeat.has_value()) ptr = AUTDGainHoloGSPATWithRepeat(ptr, _repeat.value());
    if (_constraint.has_value()) ptr = AUTDGainHoloGSPATWithConstraint(ptr, _constraint.value().ptr());
    return ptr;
  }

 private:
  std::optional<uint32_t> _repeat;
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci with naive linear synthesis
 */
class Naive final : public Holo {
 public:
  Naive() = default;
  explicit Naive(std::shared_ptr<Backend> backend) : Holo(std::move(backend)) {}

  /**
   * @brief Set amplitude constraint
   */
  Naive with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = AUTDGainHoloNaive(_backend->ptr(), reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_constraint.has_value()) ptr = AUTDGainHoloNaiveWithConstraint(ptr, _constraint.value().ptr());
    return ptr;
  }

 private:
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci with Levenberg-Marquardt algorithm
 *
 * @details
 * * K.Levenberg, “A method for the solution of certain non-linear problems in least squares,” Quarterly of applied mathematics, vol.2,
 * no.2, pp.164–168, 1944.
 * * D.W.Marquardt, “An algorithm for least-squares estimation of non-linear parameters,” Journal of the society for Industrial and
 * AppliedMathematics, vol.11, no.2, pp.431–441, 1963.
 * * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
 */
class LM final : public Holo {
 public:
  LM() = default;
  explicit LM(std::shared_ptr<Backend> backend) : Holo(std::move(backend)) {}

  /**
   * @brief Parameter. See the papers for details.
   */
  LM with_eps1(const double value) {
    _eps1 = value;
    return std::move(*this);
  }

  /**
   * @brief Parameter. See the papers for details.
   */
  LM with_eps2(const double value) {
    _eps2 = value;
    return std::move(*this);
  }

  /**
   * @brief Parameter. See the papers for details.
   */
  LM with_tau(const double value) {
    _tau = value;
    return std::move(*this);
  }

  /**
   * @brief Parameter. See the papers for details.
   */
  LM with_kmax(const uint32_t value) {
    _kmax = value;
    return std::move(*this);
  }

  /**
   * @brief Parameter. See the papers for details.
   */
  LM with_initial(std::vector<double> value) {
    _initial = std::move(value);
    return std::move(*this);
  }

  /**
   * @brief Set amplitude constraint
   */
  LM with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = AUTDGainHoloLM(_backend->ptr(), reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_eps1.has_value()) ptr = AUTDGainHoloLMWithEps1(ptr, _eps1.value());
    if (_eps2.has_value()) ptr = AUTDGainHoloLMWithEps2(ptr, _eps2.value());
    if (_tau.has_value()) ptr = AUTDGainHoloLMWithTau(ptr, _tau.value());
    if (_kmax.has_value()) ptr = AUTDGainHoloLMWithKMax(ptr, _kmax.value());
    if (!_initial.empty()) ptr = AUTDGainHoloLMWithInitial(ptr, _initial.data(), static_cast<uint64_t>(_initial.size()));
    if (_constraint.has_value()) ptr = AUTDGainHoloLMWithConstraint(ptr, _constraint.value().ptr());
    return ptr;
  }

 private:
  std::optional<double> _eps1;
  std::optional<double> _eps2;
  std::optional<double> _tau;
  std::optional<uint32_t> _kmax;
  std::vector<double> _initial;
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci with greedy algorithm
 *
 * @details Shun Suzuki, Masahiro Fujiwara, Yasutoshi Makino, and Hiroyuki Shinoda, “Radiation Pressure Field Reconstruction for Ultrasound Midair
 * Haptics by Greedy Algorithm with Brute-Force Search,” in IEEE Transactions on Haptics, doi: 10.1109/TOH.2021.3076489
 */
class Greedy final : public Holo {
 public:
  Greedy() = default;

  /**
   * @brief Parameter. See the paper for details.
   */
  Greedy with_phase_div(const uint32_t value) {
    _div = value;
    return std::move(*this);
  }

  /**
   * @brief Set amplitude constraint
   */
  Greedy with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainHoloGreedy(reinterpret_cast<const double*>(_foci.data()), _amps.data(),
                                                            static_cast<uint64_t>(_amps.size()));
    if (_div.has_value()) ptr = AUTDGainHoloGreedyWithPhaseDiv(ptr, _div.value());
    if (_constraint.has_value()) ptr = AUTDGainHoloLMWithConstraint(ptr, _constraint.value().ptr());
    return ptr;
  }

 private:
  std::optional<uint32_t> _div;
  std::optional<AmplitudeConstraint> _constraint;
};

}  // namespace autd3::gain::holo
