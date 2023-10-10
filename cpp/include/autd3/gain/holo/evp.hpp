// File: evp.hpp
// Project: holo
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/holo/holo.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::gain::holo {

/**
 * @brief Gain to produce multiple foci by solving Eigen Value Problem
 *
 * @details Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound." ACM Transactions on Graphics (TOG) 33.6 (2014):
 * 1-10.
 */
template <class B>
class EVP final : public Holo<EVP<B>, B>, public IntoCache<EVP<B>>, public IntoTransform<EVP<B>> {
 public:
  explicit EVP(std::shared_ptr<B> backend) : Holo<EVP, B>(std::move(backend)) {}

  AUTD3_DEF_PARAM(EVP, double, gamma)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = this->_backend->evp(reinterpret_cast<const double*>(this->_foci.data()), this->_amps.data(), this->_amps.size());
    if (_gamma.has_value()) ptr = this->_backend->evp_with_gamma(ptr, _gamma.value());
    if (this->_constraint.has_value()) ptr = this->_backend->evp_with_constraint(ptr, this->_constraint.value());
    return ptr;
  }

 private:
  std::optional<double> _gamma;
};

}  // namespace autd3::gain::holo
