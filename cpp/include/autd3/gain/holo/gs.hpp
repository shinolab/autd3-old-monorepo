// File: gs.hpp
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
#include "autd3/gain/holo/holo.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::gain::holo {

/**
 * @brief Gain to produce multiple foci with GS algorithm
 *
 * @details Asier Marzo and Bruce W Drinkwater. Holographic acoustic tweezers.Proceedings of theNational Academy of Sciences, 116(1):84–89, 2019.
 */
template <backend B>
class GS final : public Holo<GS<B>>, public IntoCache<GS<B>>, public IntoTransform<GS<B>> {
 public:
  explicit GS(std::shared_ptr<B> backend) : Holo<GS>(), _backend(std::move(backend)) {}

  AUTD3_DEF_PARAM(GS, uint32_t, repeat)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::geometry::Geometry&) const override {
    auto ptr = this->_backend->gs(reinterpret_cast<const double*>(this->_foci.data()), reinterpret_cast<const double*>(this->_amps.data()),
                                  this->_amps.size());
    if (_repeat.has_value()) ptr = this->_backend->gs_with_repeat(ptr, _repeat.value());
    if (this->_constraint.has_value()) ptr = this->_backend->gs_with_constraint(ptr, this->_constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
  std::optional<uint32_t> _repeat;
};

}  // namespace autd3::gain::holo
