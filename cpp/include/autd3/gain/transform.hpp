// File: transform.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 20/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iterator>
#include <numeric>
#include <vector>

#include "autd3/gain/transform.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

template <class G, typename F>
class Transform final : public internal::Gain {
 public:
  Transform(G g, F& f) : _g(std::move(g)), _f(f) { static_assert(std::is_base_of_v<Gain, G>, "This is not Gain"); }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
    std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> drives;

    char err[256]{};
    auto res = internal::native_methods::AUTDGainCalc(_g.gain_ptr(geometry), geometry.ptr(), err);
    if (res._0 == nullptr) throw internal::AUTDException(err);
    for (const auto& dev : geometry) {
      std::vector<internal::native_methods::Drive> drives;
      drives.resize(dev.num_transducers());
      internal::native_methods::AUTDGainCalcGetResult(res, drives.data(), static_cast<uint32_t>(dev.idx()));
      _cache.emplace(dev.idx(), std::move(drives));
    }

    std::for_each(geometry.begin(), geometry.end(), [this, &drives](const internal::Device& dev) {
      std::vector<internal::native_methods::Drive> d;
      d.resize(dev.num_transducers());
      internal::native_methods::AUTDGainCalcGetResult(res, d.data(), static_cast<uint32_t>(dev.idx()));
      std::for_each(dev.cbegin(), dev.cend(),
                    [this, &d, &dev](const internal::Transducer& tr) { d[tr.local_idx()] = _f(dev, tr, d[tr.local_idx()]); });
      drives.emplace(dev.idx(), std::move(d));
    });

    internal::native_methods::AUTDGainCalcFreeResult(res);
    return std::accumulate(geometry.begin(), geometry.end(), internal::native_methods::AUTDGainCustom(),
                           [this, &drives](const internal::native_methods::GainPtr acc, const internal::Device& dev) {
                             return AUTDGainCustomSet(acc, static_cast<uint32_t>(dev.idx()), drives[dev.idx()].data(),
                                                      static_cast<uint32_t>(drives[dev.idx()].size()));
                           });
  }

 private:
  G _g;
  const F& _f;
};

}  // namespace autd3::gain
