// File: lm.hpp
// Project: holo
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <vector>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/holo/backend.hpp"
#include "autd3/gain/holo/constraint.hpp"
#include "autd3/gain/holo/utils.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

#if __cplusplus >= 202002L
#include <ranges>
#endif

namespace autd3::gain::holo {

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

  AUTD3_IMPL_WITH_CACHE_GAIN(LM)
  AUTD3_IMPL_WITH_TRANSFORM_GAIN(LM)

  AUTD3_DEF_PARAM(LM, double, eps1)
  AUTD3_DEF_PARAM(LM, double, eps2)
  AUTD3_DEF_PARAM(LM, double, tau)
  AUTD3_DEF_PARAM(LM, uint32_t, k_max)

  AUTD3_DEF_PARAM(LM, AmplitudeConstraint, constraint)

  void with_initial(std::vector<double> value) & { _initial = std::move(value); }

  [[nodiscard]] LM&& with_initial(std::vector<double> value) && {
    _initial = std::move(value);
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
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
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
  std::optional<double> _eps1;
  std::optional<double> _eps2;
  std::optional<double> _tau;
  std::optional<uint32_t> _k_max;
  std::vector<double> _initial;
  std::optional<AmplitudeConstraint> _constraint;
};

}  // namespace autd3::gain::holo
