// File: holo.hpp
// Project: holo
// Created Date: 10/10/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <ranges>
#include <vector>

#include "autd3/gain/holo/backend.hpp"
#include "autd3/gain/holo/constraint.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"

namespace autd3::gain::holo {

template <class H, class B>
class Holo : public internal::Gain {
 public:
  explicit Holo(std::shared_ptr<B> backend) : _backend(std::move(backend)) {
    static_assert(std::is_base_of_v<Backend, std::remove_reference_t<B>>, "This is not Backend");
  }
  Holo() = default;

  void add_focus(internal::Vector3 focus, double amp) & {
    _foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
  }

  [[nodiscard]] H add_focus(internal::Vector3 focus, double amp) && {
    this->_foci.emplace_back(std::move(focus));
    _amps.emplace_back(amp);
    return std::move(*static_cast<H*>(this));
  }

  template <std::ranges::viewable_range R>
  auto add_foci_from_iter(R&& iter) -> std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, std::pair<internal::Vector3, double>>>& {
    for (auto [focus, amp] : iter) {
      _foci.emplace_back(std::move(focus));
      _amps.emplace_back(amp);
    }
  }

  template <std::ranges::viewable_range R>
  auto add_foci_from_iter(R&& iter) -> std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, std::pair<internal::Vector3, double>>, H>&& {
    for (auto [focus, amp] : iter) {
      _foci.emplace_back(std::move(focus));
      _amps.emplace_back(amp);
    }
    return std::move(*static_cast<H*>(this));
  }

  void with_constraint(const AmplitudeConstraint value) & { _constraint = value; }

  [[nodiscard]] H with_constraint(const AmplitudeConstraint value) && {
    _constraint = value;
    return std::move(*static_cast<H*>(this));
  }

 protected:
  std::shared_ptr<B> _backend;
  std::vector<internal::Vector3> _foci;
  std::vector<double> _amps;
  std::optional<AmplitudeConstraint> _constraint;
};

}  // namespace autd3::gain::holo