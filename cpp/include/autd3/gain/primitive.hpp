// File: primitive.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/def.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

class Focus : public internal::Gain {
 public:
  Focus(const internal::Vector3 p, const double amp = 1.0) : internal::Gain(internal::native_methods::AUTDGainFocus(p.x(), p.y(), p.z(), amp)) {}
};

class BesselBeam : public internal::Gain {
 public:
  BesselBeam(const internal::Vector3 p, const internal::Vector3 d, const double theta, const double amp = 1.0)
      : internal::Gain(internal::native_methods::AUTDGainBesselBeam(p.x(), p.y(), p.z(), d.x(), d.y(), d.z(), theta, amp)) {}
};

class PlaneWave : public internal::Gain {
 public:
  PlaneWave(const internal::Vector3 d, const double amp = 1.0)
      : internal::Gain(internal::native_methods::AUTDGainPlaneWave(d.x(), d.y(), d.z(), amp)) {}
};

class Grouped : public internal::Gain {
 public:
  Grouped() : internal::Gain(internal::native_methods::AUTDGainGrouped()) {}

  template <class G>
  void add(const size_t device_idx, G&& gain) {
    static_assert(std::is_base_of_v<internal::Gain, std::remove_reference_t<G>>, "This is not Gain");
    internal::native_methods::AUTDGainGroupedAdd(_ptr, static_cast<uint32_t>(device_idx), gain.ptr());
    gain.set_released();
  }
};

struct Drive {
  double phase;
  double amp;
};

class Gain : public internal::Gain {
 public:
  Gain() : internal::Gain(nullptr) {}

  virtual std::vector<Drive> calc(const internal::Geometry& geometry) = 0;

  void* calc_ptr(const internal::Geometry& geometry) override {
    const auto drives = calc(geometry);
    const auto size = static_cast<uint64_t>(drives.size());
    std::vector<double> amps;
    std::vector<double> phases;
    std::transform(drives.begin(), drives.end(), std::back_inserter(amps), [](const auto& d) { return d.amp; });
    std::transform(drives.begin(), drives.end(), std::back_inserter(phases), [](const auto& d) { return d.phase; });

    _ptr = internal::native_methods::AUTDGainCustom(amps.data(), phases.data(), size);
    return _ptr;
  }

  template <class Fn>
  static std::vector<Drive> transform(const internal::Geometry& geometry, Fn func) {
    std::vector<Drive> drives;
    drives.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(drives), func);
    return drives;
  }
};

}  // namespace autd3::gain
