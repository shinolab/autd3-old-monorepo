// File: transform.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <numeric>
#include <vector>

#include "autd3/gain/transform.hpp"
#include "autd3/internal/drive.hpp"
#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/utils.hpp"

namespace autd3::gain {

template <class F>
concept gain_transform_f = requires(F f, const internal::geometry::Device& dev, const internal::geometry::Transducer& tr, const internal::Drive d) {
  { f(dev, tr, d) } -> std::same_as<internal::Drive>;
};

template <class G, gain_transform_f F>
class Transform final : public internal::Gain, public IntoCache<Transform<G, F>> {
 public:
  Transform(G g, const F& f) : _g(std::move(g)), _f(f) {}

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::geometry::Geometry& geometry) const override {
    std::unordered_map<size_t, std::vector<internal::Drive>> drives;

    const auto res = validate(internal::native_methods::AUTDGainCalc(_g.gain_ptr(geometry), geometry.ptr()));
    std::for_each(geometry.devices().begin(), geometry.devices().end(), [this, &res, &drives](const internal::geometry::Device& dev) {
      std::vector<internal::Drive> d;
      d.resize(dev.num_transducers(), internal::Drive{internal::Phase(0), internal::EmitIntensity::minimum()});
      internal::native_methods::AUTDGainCalcGetResult(res, reinterpret_cast<internal::native_methods::Drive*>(d.data()),
                                                      static_cast<uint32_t>(dev.idx()));
      std::for_each(dev.cbegin(), dev.cend(), [this, &d, &dev](const internal::geometry::Transducer& tr) { d[tr.idx()] = _f(dev, tr, d[tr.idx()]); });
      drives.emplace(dev.idx(), std::move(d));
    });

    internal::native_methods::AUTDGainCalcFreeResult(res);
    return std::accumulate(geometry.devices().begin(), geometry.devices().end(), internal::native_methods::AUTDGainCustom(),
                           [this, &drives](const internal::native_methods::GainPtr acc, const internal::geometry::Device& dev) {
                             return AUTDGainCustomSet(acc, static_cast<uint32_t>(dev.idx()),
                                                      reinterpret_cast<internal::native_methods::Drive*>(drives[dev.idx()].data()),
                                                      static_cast<uint32_t>(drives[dev.idx()].size()));
                           });
  }

 private:
  G _g;
  const F& _f;
};

template <class G>
class IntoTransform {
 public:
  template <gain_transform_f F>
  [[nodiscard]] Transform<G, F> with_transform(const F& f) & {
    return Transform(*static_cast<G*>(this), f);
  }
  template <gain_transform_f F>
  [[nodiscard]] Transform<G, F> with_transform(const F& f) && {
    return Transform(std::move(*static_cast<G*>(this)), f);
  }
};

}  // namespace autd3::gain
