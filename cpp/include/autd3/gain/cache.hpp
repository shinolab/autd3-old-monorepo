// File: cache.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 15/09/2023
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

/**
 * @brief Gain to cache the result of calculation
 */
template <class G>
class Cache final : public internal::Gain {
 public:
  explicit Cache(G g) : _g(std::move(g)) { static_assert(std::is_base_of_v<Gain, G>, "This is not Gain"); }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
    std::vector<uint32_t> device_indices;
    device_indices.reserve(geometry.num_devices());
    std::transform(geometry.cbegin(), geometry.cend(), std::back_inserter(device_indices),
                   [](const internal::Device& dev) { return static_cast<uint32_t>(dev.idx()); });

    if (_cache.size() != device_indices.size() ||
        std::any_of(device_indices.begin(), device_indices.end(), [this](const uint32_t idx) { return _cache.find(idx) == _cache.end(); })) {
      std::vector<std::vector<internal::native_methods::Drive>> drives;
      drives.reserve(device_indices.size());
      std::transform(geometry.cbegin(), geometry.cend(), std::back_inserter(drives), [](const internal::Device& dev) {
        std::vector<internal::native_methods::Drive> d;
        d.resize(dev.num_transducers());
        return std::move(d);
      });

      std::vector<internal::native_methods::Drive*> drives_ptrs;
      drives_ptrs.reserve(drives.size());
      std::transform(drives.begin(), drives.end(), std::back_inserter(drives_ptrs),
                     [](std::vector<internal::native_methods::Drive>& d) { return d.data(); });

      if (char err[256]{}; internal::native_methods::AUTDGainCalc(_g.gain_ptr(geometry), geometry.ptr(), drives_ptrs.data(), err) ==
                           internal::native_methods::AUTD3_ERR)
        throw internal::AUTDException(err);
      for (size_t i = 0; i < device_indices.size(); i++) _cache.emplace(device_indices[i], std::move(drives[i]));
    }

    return std::accumulate(geometry.cbegin(), geometry.cend(), internal::native_methods::AUTDGainCustom(),
                           [this](const internal::native_methods::GainPtr acc, const internal::Device& dev) {
                             return AUTDGainCustomSet(acc, static_cast<uint32_t>(dev.idx()), _cache[dev.idx()].data(),
                                                      static_cast<uint32_t>(_cache[dev.idx()].size()));
                           });
  }

 private:
  G _g;
  mutable std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> _cache;
};

}  // namespace autd3::gain
