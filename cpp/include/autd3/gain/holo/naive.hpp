// File: naive.hpp
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

namespace autd3::gain::holo {

/**
 * @brief Gain to produce multiple foci with naive linear synthesis
 */
template <backend B>
class Naive final : public Holo<Naive<B>>, public IntoCache<Naive<B>>, public IntoTransform<Naive<B>> {
 public:
  explicit Naive(std::shared_ptr<B> backend) : Holo<Naive>(), _backend(std::move(backend)) {}

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::geometry::Geometry&) const override {
    auto ptr = this->_backend->naive(reinterpret_cast<const double*>(this->_foci.data()), reinterpret_cast<const double*>(this->_amps.data()),
                                     this->_amps.size());
    if (this->_constraint.has_value()) ptr = this->_backend->naive_with_constraint(ptr, this->_constraint.value());
    return ptr;
  }

 private:
  std::shared_ptr<B> _backend;
};

}  // namespace autd3::gain::holo
