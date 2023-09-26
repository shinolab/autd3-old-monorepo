// File: evp.hpp
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

#include "autd3/gain/holo/backend.hpp"
#include "autd3/gain/holo/constraint.hpp"
#include "autd3/gain/holo/utils.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

#if __cplusplus >= 202002L
#include <ranges>
#endif

namespace autd3::gain::holo {

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

  AUTD3_DEF_PARAM(EVP, double, gamma)
  AUTD3_DEF_PARAM(EVP, AmplitudeConstraint, constraint)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = _backend->evp(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
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

}  // namespace autd3::gain::holo
