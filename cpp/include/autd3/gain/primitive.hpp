// File: primitive.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iterator>
#include <vector>
#include <memory>

#include "autd3/internal/def.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

class Focus final : public internal::Gain {
 public:
  explicit Focus(internal::Vector3 p) : _p(std::move(p)) {}

  Focus with_amp(const double amp) {
    _amp = amp;
    return *this;
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainFocus(_p.x(), _p.y(), _p.z());
    if (_amp.has_value()) ptr = AUTDGainFocusWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  internal::Vector3 _p;
  std::optional<double> _amp;
};

class Bessel final : public internal::Gain {
 public:
  explicit Bessel(internal::Vector3 p, internal::Vector3 d, const double theta) : _p(std::move(p)), _d(std::move(d)), _theta(theta) {}

  Bessel with_amp(const double amp) {
    _amp = amp;
    return *this;
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainBessel(_p.x(), _p.y(), _p.z(), _d.x(), _d.y(), _d.z(), _theta);
    if (_amp.has_value()) ptr = AUTDGainBesselWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  internal::Vector3 _p;
  internal::Vector3 _d;
  double _theta;
  std::optional<double> _amp;
};

class Plane final : public internal::Gain {
 public:
  explicit Plane(internal::Vector3 d) : _d(std::move(d)) {}

  Plane with_amp(const double amp) {
    _amp = amp;
    return *this;
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    auto ptr = internal::native_methods::AUTDGainPlane(_d.x(), _d.y(), _d.z());
    if (_amp.has_value()) ptr = AUTDGainPlaneWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  internal::Vector3 _d;
  std::optional<double> _amp;
};

class Grouped final : public internal::Gain {
 public:
  Grouped() = default;

  template <class G>
  void add_gain(const size_t device_idx, G&& gain) {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.emplace_back(std::make_pair(device_idx, std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain))));
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
    auto ptr = internal::native_methods::AUTDGainGrouped();
    for (auto& [idx, gain] : _gains) ptr = AUTDGainGroupedAdd(ptr, static_cast<uint32_t>(idx), gain->gain_ptr(geometry));
    return ptr;
  }

 private:
  std::vector<std::pair<size_t, std::shared_ptr<Gain>>> _gains;
};

struct Drive {
  double phase;
  double amp;
};

class Gain : public internal::Gain {
 public:
  Gain() = default;

  [[nodiscard]] virtual std::vector<Drive> calc(const internal::Geometry& geometry) const = 0;

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
    const auto drives = calc(geometry);
    const auto size = static_cast<uint64_t>(drives.size());
    std::vector<double> amps;
    amps.reserve(drives.size());
    std::transform(drives.begin(), drives.end(), std::back_inserter(amps), [](const auto& d) { return d.amp; });
    std::vector<double> phases;
    phases.reserve(drives.size());
    std::transform(drives.begin(), drives.end(), std::back_inserter(phases), [](const auto& d) { return d.phase; });

    return internal::native_methods::AUTDGainCustom(amps.data(), phases.data(), size);
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
