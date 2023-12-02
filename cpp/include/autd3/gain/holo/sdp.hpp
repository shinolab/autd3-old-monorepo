// File: sdp.hpp
// Project: holo
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/holo/backend.hpp"
#include "autd3/gain/holo/holo.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::gain::holo {

/**
 * @brief Gain to produce multiple foci by solving Semi-Definite Programming
 *
 * @details Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram." 2015 IEEE
 * World Haptics Conference (WHC). IEEE, 2015.
 */
template <backend B>
class SDP final : public Holo<SDP<B>>, public IntoCache<SDP<B>>, public IntoTransform<SDP<B>> {
 public:
  explicit SDP(std::shared_ptr<B> backend) : Holo<SDP>(), _backend(std::move(backend)) {}

  AUTD3_DEF_PARAM(SDP, double, alpha)
  AUTD3_DEF_PARAM(SDP, uint32_t, repeat)
  AUTD3_DEF_PARAM(SDP, double, lambda)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::geometry::Geometry&) const override {
    auto ptr = this->_backend->sdp(reinterpret_cast<const double*>(this->_foci.data()), reinterpret_cast<const double*>(this->_amps.data()),
                                   this->_amps.size());
    if (_alpha.has_value()) ptr = this->_backend->sdp_with_alpha(ptr, _alpha.value());
    if (_repeat.has_value()) ptr = this->_backend->sdp_with_repeat(ptr, _repeat.value());
    if (_lambda.has_value()) ptr = this->_backend->sdp_with_lambda(ptr, _lambda.value());
    if (this->_constraint.has_value()) ptr = this->_backend->sdp_with_constraint(ptr, this->_constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::optional<double> _alpha;
  std::optional<uint32_t> _repeat;
  std::optional<double> _lambda;
};

}  // namespace autd3::gain::holo
