// File: gspat.hpp
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

namespace autd3::gain::holo {

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

  AUTD3_IMPL_WITH_CACHE_GAIN(GSPAT)
  AUTD3_IMPL_WITH_TRANSFORM_GAIN(GSPAT)

  AUTD3_DEF_PARAM(GSPAT, uint32_t, repeat)
  AUTD3_DEF_PARAM(GSPAT, AmplitudeConstraint, constraint)

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

}  // namespace autd3::gain::holo
