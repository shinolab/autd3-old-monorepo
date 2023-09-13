// File: greedy.hpp
// Project: holo
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/gain/holo/constraint.hpp"
#include "autd3/gain/holo/utils.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::gain::holo {

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

  AUTD3_DEF_PARAM(Greedy, uint32_t, phase_div)

  AUTD3_DEF_PARAM(Greedy, AmplitudeConstraint, constraint)

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
