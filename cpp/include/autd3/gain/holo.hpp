// File: holo.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/06/2023
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

class Backend {
 public:
  Backend() noexcept = default;
  Backend(const Backend& obj) = default;
  Backend& operator=(const Backend& obj) = default;
  Backend(Backend&& obj) = default;
  Backend& operator=(Backend&& obj) = default;
  virtual ~Backend() = default;

  [[nodiscard]] virtual internal::native_methods::BackendPtr ptr() const = 0;
};

class DefaultBackend final : public Backend {
 public:
  DefaultBackend() = default;

  [[nodiscard]] internal::native_methods::BackendPtr ptr() const override { return internal::native_methods::AUTDDefaultBackend(); }
};

class AmplitudeConstraint {
 public:
  static AmplitudeConstraint dont_care() { return AmplitudeConstraint{internal::native_methods::AUTDGainHoloDotCareConstraint()}; }

  static AmplitudeConstraint normalize() { return AmplitudeConstraint{internal::native_methods::AUTDGainHoloNormalizeConstraint()}; }

  static AmplitudeConstraint uniform(const double value) {
    return AmplitudeConstraint{internal::native_methods::AUTDGainHoloUniformConstraint(value)};
  }

  static AmplitudeConstraint clamp(const double min_v, const double max_v) {
    return AmplitudeConstraint{internal::native_methods::AUTDGainHoloClampConstraint(min_v, max_v)};
  }

  [[nodiscard]] internal::native_methods::ConstraintPtr ptr() const { return _ptr; }

 private:
  explicit AmplitudeConstraint(const internal::native_methods::ConstraintPtr ptr) : _ptr(ptr) {}

  internal::native_methods::ConstraintPtr _ptr;
};

class Holo : public internal::Gain {
 public:
  explicit Holo() : _backend(std::make_shared<DefaultBackend>()) {}

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

class SDP final : public Holo {
 public:
  SDP() = default;

  SDP with_alpha(const double value) {
    _alpha = value;
    return std::move(*this);
  }

  SDP with_repeat(const uint32_t value) {
    _repeat = value;
    return std::move(*this);
  }
  SDP with_lambda(const double value) {
    _lambda = value;
    return std::move(*this);
  }

  SDP with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  template <class B>
  SDP with_backend(B&& backend) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
    _backend = std::make_shared<std::remove_reference_t<B>>(std::forward<B>(backend));
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

class EVP final : public Holo {
 public:
  EVP() = default;

  EVP with_gamma(const double value) {
    _gamma = value;
    return std::move(*this);
  }

  EVP with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  template <class B>
  EVP with_backend(B&& backend) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
    _backend = std::make_shared<std::remove_reference_t<B>>(std::forward<B>(backend));
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

class GS final : public Holo {
 public:
  GS() = default;

  GS with_repeat(const uint32_t value) {
    _repeat = value;
    return std::move(*this);
  }

  GS with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  template <class B>
  GS with_backend(B&& backend) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
    _backend = std::make_shared<std::remove_reference_t<B>>(std::forward<B>(backend));
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

class GSPAT final : public Holo {
 public:
  GSPAT() = default;

  GSPAT with_repeat(const uint32_t value) {
    _repeat = value;
    return std::move(*this);
  }

  GSPAT with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  template <class B>
  GSPAT with_backend(B&& backend) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
    _backend = std::make_shared<std::remove_reference_t<B>>(std::forward<B>(backend));
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

class Naive final : public Holo {
 public:
  Naive() = default;

  Naive with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  template <class B>
  Naive with_backend(B&& backend) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
    _backend = std::make_shared<std::remove_reference_t<B>>(std::forward<B>(backend));
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

class LM final : public Holo {
 public:
  LM() = default;

  LM with_eps1(const double value) {
    _eps1 = value;
    return std::move(*this);
  }

  LM with_eps2(const double value) {
    _eps2 = value;
    return std::move(*this);
  }

  LM with_tau(const double value) {
    _tau = value;
    return std::move(*this);
  }

  LM with_kmax(const uint32_t value) {
    _kmax = value;
    return std::move(*this);
  }

  LM with_initial(std::vector<double> value) {
    _initial = std::move(value);
    return std::move(*this);
  }

  LM with_constraint(const AmplitudeConstraint constraint) {
    _constraint = constraint;
    return std::move(*this);
  }

  template <class B>
  LM with_backend(B&& backend) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
    _backend = std::make_shared<std::remove_reference_t<B>>(std::forward<B>(backend));
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

class Greedy final : public Holo {
 public:
  Greedy() = default;

  Greedy with_phase_div(const uint32_t value) {
    _div = value;
    return std::move(*this);
  }

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
