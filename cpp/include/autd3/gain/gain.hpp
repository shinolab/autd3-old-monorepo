// File: gain.hpp
// Project: gain
// Created Date: 29/08/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
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

#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

class Gain : public internal::Gain {
 public:
  Gain() = default;

  [[nodiscard]] virtual std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> calc(const internal::Geometry& geometry) const = 0;

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
    const auto drives = calc(geometry);
    return std::accumulate(
        drives.begin(), drives.end(), internal::native_methods::AUTDGainCustom(),
        [](const internal::native_methods::GainPtr acc, const std::pair<size_t, std::vector<internal::native_methods::Drive>>& kv) {
          return AUTDGainCustomSet(acc, static_cast<uint32_t>(kv.first), kv.second.data(), static_cast<uint32_t>(kv.second.size()));
        });
  }

  template <class Fn>
  [[nodiscard]] static std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> transform(const internal::Geometry& geometry,
                                                                                                          Fn func) {
    std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> drives_map;
    std::for_each(geometry.cbegin(), geometry.cend(), [&drives_map, &func](const internal::Device& dev) {
      std::vector<internal::native_methods::Drive> drives;
      drives.reserve(dev.num_transducers());
      std::transform(dev.cbegin(), dev.cend(), std::back_inserter(drives),
                     [&dev, &drives_map, &func](const internal::Transducer& tr) { return func(dev, tr); });
      drives_map[dev.idx()] = std::move(drives);
    });
    return drives_map;
  }
};

}  // namespace autd3::gain
