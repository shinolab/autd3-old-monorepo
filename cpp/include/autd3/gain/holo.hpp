// File: holo.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/08/2023
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
  virtual ~Backend() = default;

  [[nodiscard]] internal::native_methods::BackendPtr ptr() { return _ptr; }

  [[nodiscard]] virtual internal::native_methods::GainPtr sdp(const double* foci, const double* amps, const uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_alpha(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_lambda(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr evp(const double* foci, const double* amps, const uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr evp_with_gamma(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr evp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr gs(const double* foci, const double* amps, const uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gs_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gs_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr gspat(const double* foci, const double* amps, const uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gspat_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gspat_with_constraint(internal::native_methods::GainPtr ptr,
                                                                                AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr naive(const double* foci, const double* amps, const uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr naive_with_constraint(internal::native_methods::GainPtr ptr,
                                                                                AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr lm(const double* foci, const double* amps, const uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_eps1(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_eps2(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_tau(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_kmax(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_initial(internal::native_methods::GainPtr ptr, const double* v,
                                                                          uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

 protected:
  internal::native_methods::BackendPtr _ptr;
};

/**
 * @brief Backend using [Nalgebra](https://nalgebra.org/)
 */
class NalgebraBackend final : public Backend {
 public:
  NalgebraBackend() : Backend(internal::native_methods::AUTDNalgebraBackend()) {}
  ~NalgebraBackend() override {
    if (_ptr._0 != nullptr) {
      internal::native_methods::AUTDDeleteNalgebraBackend(_ptr);
      _ptr._0 = nullptr;
    }
  }

  internal::native_methods::GainPtr sdp(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloSDP(this->_ptr, foci, amps, size);
  }
  internal::native_methods::GainPtr sdp_with_alpha(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloSDPWithAlpha(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const override {
    return internal::native_methods::AUTDGainHoloSDPWithRepeat(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_lambda(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloSDPWithLambda(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloSDPWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr evp(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloEVP(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr evp_with_gamma(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloEVPWithGamma(ptr, v);
  }

  internal::native_methods::GainPtr evp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloEVPWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gs(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloGS(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr gs_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const override {
    return internal::native_methods::AUTDGainHoloGSWithRepeat(ptr, v);
  }

  internal::native_methods::GainPtr gs_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloGSWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gspat(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloGS(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr gspat_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const override {
    return internal::native_methods::AUTDGainHoloGSWithRepeat(ptr, v);
  }

  internal::native_methods::GainPtr gspat_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloGSWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr naive(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloNaive(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr naive_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloNaiveWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr lm(const double* foci, const double* amps, const uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloLM(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr lm_with_eps1(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloLMWithEps1(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_eps2(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloLMWithEps2(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_tau(internal::native_methods::GainPtr ptr, double v) const override {
    return internal::native_methods::AUTDGainHoloLMWithTau(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_kmax(internal::native_methods::GainPtr ptr, uint32_t v) const override {
    return internal::native_methods::AUTDGainHoloLMWithKMax(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_initial(internal::native_methods::GainPtr ptr, const double* v, uint64_t size) const override {
    return internal::native_methods::AUTDGainHoloLMWithInitial(ptr, v, size);
  }

  internal::native_methods::GainPtr lm_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const override {
    return internal::native_methods::AUTDGainHoloLMWithConstraint(ptr, v.ptr());
  }
};

using DefaultBackend = NalgebraBackend;

/**
 * @brief Gain to produce multiple foci by solving Semi-Denfinite Programming
 *
 * @details Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE
 * World Haptics Conference (WHC). IEEE, 2015.
 */
template <class B>
class SDP final : public internal::Gain {
 public:
  explicit SDP(std::shared_ptr<B> backend) : _backend(std::move(backend)) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
  }

  SDP add_focus(internal::Vector3 focus, double amp) {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*this);
  }

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
    auto ptr = _backend->sdp(reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_alpha.has_value()) ptr = _backend->sdp_with_alpha(ptr, _alpha.value());
    if (_repeat.has_value()) ptr = _backend->sdp_with_repeat(ptr, _repeat.value());
    if (_lambda.has_value()) ptr = _backend->sdp_with_lambda(ptr, _lambda.value());
    if (_constraint.has_value()) ptr = _backend->sdp_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
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
template <class B>
class EVP final : public internal::Gain {
 public:
  explicit EVP(std::shared_ptr<B> backend) : _backend(std::move(backend)) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
  }

  EVP add_focus(internal::Vector3 focus, double amp) {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*this);
  }

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
    auto ptr = _backend->evp(reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_gamma.has_value()) ptr = _backend->evp_with_gamma(ptr, _gamma.value());
    if (_constraint.has_value()) ptr = _backend->evp_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
  std::optional<double> _gamma;
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci with GS algorithm
 *
 * @details Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
 */
template <class B>
class GS final : public internal::Gain {
 public:
  explicit GS(std::shared_ptr<B> backend) : _backend(std::move(backend)) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
  }

  GS add_focus(internal::Vector3 focus, double amp) {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*this);
  }

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
    auto ptr = _backend->gs(reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_repeat.has_value()) ptr = _backend->gs_with_repeat(ptr, _repeat.value());
    if (_constraint.has_value()) ptr = _backend->gs_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
  std::optional<uint32_t> _repeat;
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci with GS-PAT algorithm
 *
 * @details Diego Martinez Plasencia et al. "Gs-pat: high-speed multi-point sound-fields for phased arrays of transducers," ACMTrans-actions on
 * Graphics (TOG), 39(4):138–1, 2020.
 */
template <class B>
class GSPAT final : public internal::Gain {
 public:
  explicit GSPAT(std::shared_ptr<B> backend) : _backend(std::move(backend)) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
  }

  GSPAT add_focus(internal::Vector3 focus, double amp) {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*this);
  }

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
    auto ptr = _backend->gspat(reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_repeat.has_value()) ptr = _backend->gspat_with_repeat(ptr, _repeat.value());
    if (_constraint.has_value()) ptr = _backend->gspat_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
  std::optional<uint32_t> _repeat;
  std::optional<AmplitudeConstraint> _constraint;
};

/**
 * @brief Gain to produce multiple foci with naive linear synthesis
 */
template <class B>
class Naive final : public internal::Gain {
 public:
  explicit Naive(std::shared_ptr<B> backend) : _backend(std::move(backend)) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
  }

  Naive add_focus(internal::Vector3 focus, double amp) {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*this);
  }

  /**
   * @brief Set amplitude constraint
   */
  Naive with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = _backend->naive(reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_constraint.has_value()) ptr = _backend->naive_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
  std::optional<AmplitudeConstraint> _constraint;
};

template <class B>
using LSS = Naive<B>;

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
template <class B>
class LM final : public internal::Gain {
 public:
  explicit LM(std::shared_ptr<B> backend) : _backend(std::move(backend)) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
  }

  LM add_focus(internal::Vector3 focus, double amp) {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*this);
  }

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
    auto ptr = _backend->lm(reinterpret_cast<const double*>(_foci.data()), _amps.data(), static_cast<uint64_t>(_amps.size()));
    if (_eps1.has_value()) ptr = _backend->lm_with_eps1(ptr, _eps1.value());
    if (_eps2.has_value()) ptr = _backend->lm_with_eps2(ptr, _eps2.value());
    if (_tau.has_value()) ptr = _backend->lm_with_tau(ptr, _tau.value());
    if (_kmax.has_value()) ptr = _backend->lm_with_kmax(ptr, _kmax.value());
    if (!_initial.empty()) ptr = _backend->lm_with_initial(ptr, _initial.data(), static_cast<uint64_t>(_initial.size()));
    if (_constraint.has_value()) ptr = _backend->lm_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
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
class Greedy final : public internal::Gain {
 public:
  Greedy() = default;

  Greedy add_focus(internal::Vector3 focus, double amp) {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*this);
  }

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
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
  std::optional<uint32_t> _div;
  std::optional<AmplitudeConstraint> _constraint;
};

}  // namespace autd3::gain::holo
