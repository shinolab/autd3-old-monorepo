// File: holo.hpp
// Project: holo
// Created Date: 10/10/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <ranges>
#include <vector>

#include "autd3/gain/holo/amplitude.hpp"
#include "autd3/gain/holo/constraint.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"

namespace autd3::gain::holo {

template <class R>
concept holo_foci_range = std::ranges::viewable_range<R> && std::same_as<std::ranges::range_value_t<R>, std::pair<internal::Vector3, Amplitude>>;

template <class H>
class Holo : public internal::Gain {
 public:
  Holo() = default;

  void add_focus(internal::Vector3 focus, Amplitude amp) & {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
  }

  [[nodiscard]] H add_focus(internal::Vector3 focus, Amplitude amp) && {
    this->_foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*static_cast<H*>(this));
  }

  template <holo_foci_range R>
  void add_foci_from_iter(R&& iter) & {
    for (auto [focus, amp] : iter) {
      _foci.emplace_back(std::move(focus));
      _amps.emplace_back(amp);
    }
  }

  template <holo_foci_range R>
  H add_foci_from_iter(R&& iter) && {
    for (auto [focus, amp] : iter) {
      _foci.emplace_back(std::move(focus));
      _amps.emplace_back(amp);
    }
    return std::move(*static_cast<H*>(this));
  }

  void with_constraint(const EmissionConstraint value) & { _constraint = value; }

  [[nodiscard]] H with_constraint(const EmissionConstraint value) && {
    _constraint = value;
    return std::move(*static_cast<H*>(this));
  }

 protected:
  std::vector<internal::Vector3> _foci;
  std::vector<Amplitude> _amps;
  std::optional<EmissionConstraint> _constraint;
};

}  // namespace autd3::gain::holo