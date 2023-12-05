// File: gain.hpp
// Project: gain
// Created Date: 29/08/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iterator>
#include <numeric>
#include <unordered_map>
#include <vector>

#include "autd3/internal/drive.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

template <class F>
concept gain_transform = requires(F f, const internal::geometry::Device& dev, const internal::geometry::Transducer& tr) {
  { f(dev, tr) } -> std::same_as<internal::Drive>;
};

class Gain : public internal::Gain {
 public:
  Gain() = default;

  [[nodiscard]] virtual std::unordered_map<size_t, std::vector<internal::Drive>> calc(const internal::geometry::Geometry& geometry) const = 0;

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::geometry::Geometry& geometry) const override {
    const auto drives = calc(geometry);
    return std::accumulate(drives.begin(), drives.end(), internal::native_methods::AUTDGainCustom(),
                           [](const internal::native_methods::GainPtr acc, const std::pair<size_t, std::vector<internal::Drive>>& kv) {
                             return AUTDGainCustomSet(acc, static_cast<uint32_t>(kv.first),
                                                      reinterpret_cast<const internal::native_methods::Drive*>(kv.second.data()),
                                                      static_cast<uint32_t>(kv.second.size()));
                           });
  }

  template <gain_transform Fn>
  [[nodiscard]] static std::unordered_map<size_t, std::vector<internal::Drive>> transform(const internal::geometry::Geometry& geometry, Fn func) {
    std::unordered_map<size_t, std::vector<internal::Drive>> drives_map;
    std::for_each(geometry.devices().begin(), geometry.devices().end(), [&drives_map, &func](const internal::geometry::Device& dev) {
      std::vector<internal::Drive> drives;
      drives.reserve(dev.num_transducers());
      std::transform(dev.cbegin(), dev.cend(), std::back_inserter(drives),
                     [&dev, &drives_map, &func](const internal::geometry::Transducer& tr) { return func(dev, tr); });
      drives_map[dev.idx()] = std::move(drives);
    });
    return drives_map;
  }  // LCOV_EXCL_LINE
};

}  // namespace autd3::gain
