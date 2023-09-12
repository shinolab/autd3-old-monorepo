// File: holo.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
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
  Backend() noexcept : _ptr(internal::native_methods::BackendPtr{nullptr}) {}
  explicit Backend(const internal::native_methods::BackendPtr ptr) noexcept : _ptr(ptr) {}
  Backend(const Backend& obj) = default;
  Backend& operator=(const Backend& obj) = default;
  Backend(Backend&& obj) = default;
  Backend& operator=(Backend&& obj) = default;
  virtual ~Backend() = default;

  [[nodiscard]] internal::native_methods::BackendPtr ptr() const { return _ptr; }

  [[nodiscard]] virtual internal::native_methods::GainPtr sdp(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_alpha(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_lambda(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr sdp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr evp(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr evp_with_gamma(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr evp_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr gs(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gs_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gs_with_constraint(internal::native_methods::GainPtr ptr, AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr gspat(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gspat_with_repeat(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr gspat_with_constraint(internal::native_methods::GainPtr ptr,
                                                                                AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr naive(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr naive_with_constraint(internal::native_methods::GainPtr ptr,
                                                                                AmplitudeConstraint v) const = 0;

  [[nodiscard]] virtual internal::native_methods::GainPtr lm(const double* foci, const double* amps, uint64_t size) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_eps1(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_eps2(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_tau(internal::native_methods::GainPtr ptr, double v) const = 0;
  [[nodiscard]] virtual internal::native_methods::GainPtr lm_with_k_max(internal::native_methods::GainPtr ptr, uint32_t v) const = 0;
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
      AUTDDeleteNalgebraBackend(_ptr);
      _ptr._0 = nullptr;
    }
  }
  NalgebraBackend(const NalgebraBackend& v) noexcept = delete;
  NalgebraBackend& operator=(const NalgebraBackend& obj) = delete;
  NalgebraBackend(NalgebraBackend&& obj) = default;
  NalgebraBackend& operator=(NalgebraBackend&& obj) = default;

  internal::native_methods::GainPtr sdp(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloSDP(this->_ptr, foci, amps, size);
  }
  internal::native_methods::GainPtr sdp_with_alpha(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloSDPWithAlpha(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloSDPWithRepeat(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_lambda(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloSDPWithLambda(ptr, v);
  }
  internal::native_methods::GainPtr sdp_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloSDPWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr evp(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloEVP(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr evp_with_gamma(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloEVPWithGamma(ptr, v);
  }

  internal::native_methods::GainPtr evp_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloEVPWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gs(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloGS(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr gs_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloGSWithRepeat(ptr, v);
  }

  internal::native_methods::GainPtr gs_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloGSWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr gspat(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloGS(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr gspat_with_repeat(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloGSWithRepeat(ptr, v);
  }

  internal::native_methods::GainPtr gspat_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloGSWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr naive(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloNaive(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr naive_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloNaiveWithConstraint(ptr, v.ptr());
  }

  internal::native_methods::GainPtr lm(const double* foci, const double* amps, const uint64_t size) const override {
    return AUTDGainHoloLM(this->_ptr, foci, amps, size);
  }

  internal::native_methods::GainPtr lm_with_eps1(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithEps1(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_eps2(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithEps2(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_tau(const internal::native_methods::GainPtr ptr, const double v) const override {
    return AUTDGainHoloLMWithTau(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_k_max(const internal::native_methods::GainPtr ptr, const uint32_t v) const override {
    return AUTDGainHoloLMWithKMax(ptr, v);
  }

  internal::native_methods::GainPtr lm_with_initial(const internal::native_methods::GainPtr ptr, const double* v,
                                                    const uint64_t size) const override {
    return AUTDGainHoloLMWithInitial(ptr, v, size);
  }

  internal::native_methods::GainPtr lm_with_constraint(const internal::native_methods::GainPtr ptr, const AmplitudeConstraint v) const override {
    return AUTDGainHoloLMWithConstraint(ptr, v.ptr());
  }
};

#define AUTD3_HOLO_ADD_FOCUS(HOLO_T)                              \
  void add_focus(Vector3 focus, double amp)& {                    \
    _foci.emplace_back(std::move(focus));                         \
    _amps.emplace_back(amp);                                      \
  }                                                               \
  [[nodiscard]] HOLO_T&& add_focus(Vector3 focus, double amp)&& { \
    _foci.emplace_back(std::move(focus));                         \
    _amps.emplace_back(amp);                                      \
    return std::move(*this);                                      \
  }
#define AUTD3_HOLO_ADD_FOCI(HOLO_T)                                                                                                          \
  template <std::ranges::viewable_range R>                                                                                                   \
  auto add_foci_from_iter(R&& iter)->std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, std::pair<Vector3, double>>>& {            \
    for (auto [focus, amp] : iter) {                                                                                                         \
      _foci.emplace_back(std::move(focus));                                                                                                  \
      _amps.emplace_back(amp);                                                                                                               \
    }                                                                                                                                        \
  }                                                                                                                                          \
  template <std::ranges::viewable_range R>                                                                                                   \
  auto add_foci_from_iter(R&& iter)->std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, std::pair<Vector3, double>>, HOLO_T&&>&& { \
    for (auto [focus, amp] : iter) {                                                                                                         \
      _foci.emplace_back(std::move(focus));                                                                                                  \
      _amps.emplace_back(amp);                                                                                                               \
    }                                                                                                                                        \
    return std::move(*this);                                                                                                                 \
  }

#define AUTD3_HOLO_PARAM(HOLO_T, PARAM_T, PARAM_NAME)                     \
  void with_##PARAM_NAME(const PARAM_T value)& { _##PARAM_NAME = value; } \
  [[nodiscard]] HOLO_T&& with_##PARAM_NAME(const PARAM_T value)&& {       \
    _##PARAM_NAME = value;                                                \
    return std::move(*this);                                              \
  }

/**
 * @brief Gain to produce multiple foci by solving Semi-Definite Programming
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

  AUTD3_HOLO_ADD_FOCUS(SDP)
#if __cplusplus >= 202002L
  AUTD3_HOLO_ADD_FOCI(SDP)
#endif

  AUTD3_HOLO_PARAM(SDP, double, alpha)
  AUTD3_HOLO_PARAM(SDP, uint32_t, repeat)
  AUTD3_HOLO_PARAM(SDP, double, lambda)
  AUTD3_HOLO_PARAM(SDP, AmplitudeConstraint, constraint)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const Geometry&) const override {
    auto ptr = _backend->sdp(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
    if (_alpha.has_value()) ptr = _backend->sdp_with_alpha(ptr, _alpha.value());
    if (_repeat.has_value()) ptr = _backend->sdp_with_repeat(ptr, _repeat.value());
    if (_lambda.has_value()) ptr = _backend->sdp_with_lambda(ptr, _lambda.value());
    if (_constraint.has_value()) ptr = _backend->sdp_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<Vector3> _foci;
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

  AUTD3_HOLO_ADD_FOCUS(EVP)
#if __cplusplus >= 202002L
  AUTD3_HOLO_ADD_FOCI(EVP)
#endif

  AUTD3_HOLO_PARAM(EVP, double, gamma)
  AUTD3_HOLO_PARAM(EVP, AmplitudeConstraint, constraint)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const Geometry&) const override {
    auto ptr = _backend->evp(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
    if (_gamma.has_value()) ptr = _backend->evp_with_gamma(ptr, _gamma.value());
    if (_constraint.has_value()) ptr = _backend->evp_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<Vector3> _foci;
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

  AUTD3_HOLO_ADD_FOCUS(GS)
#if __cplusplus >= 202002L
  AUTD3_HOLO_ADD_FOCI(GS)
#endif

  AUTD3_HOLO_PARAM(GS, uint32_t, repeat)
  AUTD3_HOLO_PARAM(GS, AmplitudeConstraint, constraint)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const Geometry&) const override {
    auto ptr = _backend->gs(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
    if (_repeat.has_value()) ptr = _backend->gs_with_repeat(ptr, _repeat.value());
    if (_constraint.has_value()) ptr = _backend->gs_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<Vector3> _foci;
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

  AUTD3_HOLO_ADD_FOCUS(GSPAT)
#if __cplusplus >= 202002L
  AUTD3_HOLO_ADD_FOCI(GSPAT)
#endif

  AUTD3_HOLO_PARAM(GSPAT, uint32_t, repeat)
  AUTD3_HOLO_PARAM(GSPAT, AmplitudeConstraint, constraint)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const Geometry&) const override {
    auto ptr = _backend->gspat(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
    if (_repeat.has_value()) ptr = _backend->gspat_with_repeat(ptr, _repeat.value());
    if (_constraint.has_value()) ptr = _backend->gspat_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<Vector3> _foci;
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

  AUTD3_HOLO_ADD_FOCUS(Naive)
#if __cplusplus >= 202002L
  AUTD3_HOLO_ADD_FOCI(Naive)
#endif

  AUTD3_HOLO_PARAM(Naive, AmplitudeConstraint, constraint)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const Geometry&) const override {
    auto ptr = _backend->naive(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
    if (_constraint.has_value()) ptr = _backend->naive_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<Vector3> _foci;
  std::vector<double> _amps;
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
template <class B>
class LM final : public internal::Gain {
 public:
  explicit LM(std::shared_ptr<B> backend) : _backend(std::move(backend)) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
  }

  AUTD3_HOLO_ADD_FOCUS(LM)
#if __cplusplus >= 202002L
  AUTD3_HOLO_ADD_FOCI(LM)
#endif

  AUTD3_HOLO_PARAM(LM, double, eps1)
  AUTD3_HOLO_PARAM(LM, double, eps2)
  AUTD3_HOLO_PARAM(LM, double, tau)
  AUTD3_HOLO_PARAM(LM, double, k_max)

  AUTD3_HOLO_PARAM(LM, AmplitudeConstraint, constraint)

  void with_initial(std::vector<double> value) & { _initial = std::move(value); }

  [[nodiscard]] LM&& with_initial(std::vector<double> value) && {
    _initial = std::move(value);
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const Geometry&) const override {
    auto ptr = _backend->lm(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
    if (_eps1.has_value()) ptr = _backend->lm_with_eps1(ptr, _eps1.value());
    if (_eps2.has_value()) ptr = _backend->lm_with_eps2(ptr, _eps2.value());
    if (_tau.has_value()) ptr = _backend->lm_with_tau(ptr, _tau.value());
    if (_k_max.has_value()) ptr = _backend->lm_with_k_max(ptr, _k_max.value());
    if (!_initial.empty()) ptr = _backend->lm_with_initial(ptr, _initial.data(), _initial.size());
    if (_constraint.has_value()) ptr = _backend->lm_with_constraint(ptr, _constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::vector<Vector3> _foci;
  std::vector<double> _amps;
  std::optional<double> _eps1;
  std::optional<double> _eps2;
  std::optional<double> _tau;
  std::optional<uint32_t> _k_max;
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

  AUTD3_HOLO_ADD_FOCUS(Greedy)
#if __cplusplus >= 202002L
  AUTD3_HOLO_ADD_FOCI(Greedy)
#endif

  AUTD3_HOLO_PARAM(Greedy, uint32_t, phase_div)

  AUTD3_HOLO_PARAM(Greedy, AmplitudeConstraint, constraint)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainHoloGreedy(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
    if (_phase_div.has_value()) ptr = AUTDGainHoloGreedyWithPhaseDiv(ptr, _phase_div.value());
    if (_constraint.has_value()) ptr = AUTDGainHoloLMWithConstraint(ptr, _constraint.value().ptr());
    return ptr;
  }

 private:
  std::vector<Vector3> _foci;
  std::vector<double> _amps;
  std::optional<uint32_t> _phase_div;
  std::optional<AmplitudeConstraint> _constraint;
};

}  // namespace autd3::gain::holo
