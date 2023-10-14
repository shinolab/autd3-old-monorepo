// File: greedy.hpp
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

#include <vector>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/holo/constraint.hpp"
#include "autd3/gain/holo/holo.hpp"
#include "autd3/gain/transform.hpp"
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
class Greedy final : public Holo<Greedy, void>, public IntoCache<Greedy>, public IntoTransform<Greedy> {
 public:
  Greedy() : Holo() {}

  AUTD3_DEF_PARAM(Greedy, uint32_t, phase_div)

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr =
        internal::native_methods::AUTDGainHoloGreedy(reinterpret_cast<const double*>(this->_foci.data()), this->_amps.data(), this->_amps.size());
    if (_phase_div.has_value()) ptr = AUTDGainHoloGreedyWithPhaseDiv(ptr, _phase_div.value());
    if (this->_constraint.has_value()) ptr = AUTDGainHoloGreedyWithConstraint(ptr, this->_constraint.value().ptr());
    return ptr;
  }

 private:
  std::optional<uint32_t> _phase_div;
};

}  // namespace autd3::gain::holo
