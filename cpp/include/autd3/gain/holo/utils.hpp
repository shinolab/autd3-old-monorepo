// File: utils.hpp
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

#define AUTD3_HOLO_ADD_FOCUS(HOLO_T)                                        \
  void add_focus(internal::Vector3 focus, double amp)& {                    \
    _foci.emplace_back(std::move(focus));                                   \
    _amps.emplace_back(amp);                                                \
  }                                                                         \
  [[nodiscard]] HOLO_T&& add_focus(internal::Vector3 focus, double amp)&& { \
    _foci.emplace_back(std::move(focus));                                   \
    _amps.emplace_back(amp);                                                \
    return std::move(*this);                                                \
  }
#define AUTD3_HOLO_ADD_FOCI(HOLO_T)                                                                                                         \
  template <std::ranges::viewable_range R>                                                                                                  \
  auto add_foci_from_iter(R&& iter)->std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, std::pair<internal::Vector3, double>>>& { \
    for (auto [focus, amp] : iter) {                                                                                                        \
      _foci.emplace_back(std::move(focus));                                                                                                 \
      _amps.emplace_back(amp);                                                                                                              \
    }                                                                                                                                       \
  }                                                                                                                                         \
  template <std::ranges::viewable_range R>                                                                                                  \
  auto add_foci_from_iter(R&& iter)                                                                                                         \
      ->std::enable_if_t<std::same_as<std::ranges::range_value_t<R>, std::pair<internal::Vector3, double>>, HOLO_T&&>&& {                   \
    for (auto [focus, amp] : iter) {                                                                                                        \
      _foci.emplace_back(std::move(focus));                                                                                                 \
      _amps.emplace_back(amp);                                                                                                              \
    }                                                                                                                                       \
    return std::move(*this);                                                                                                                \
  }
