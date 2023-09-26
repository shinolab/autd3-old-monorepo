// File: gs.hpp
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

  AUTD3_DEF_PARAM(GS, uint32_t, repeat)
  AUTD3_DEF_PARAM(GS, AmplitudeConstraint, constraint)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = _backend->gs(reinterpret_cast<const double*>(_foci.data()), _amps.data(), _amps.size());
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

}  // namespace autd3::gain::holo
