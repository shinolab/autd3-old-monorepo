// File: primitive.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 25/07/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iterator>
#include <memory>
#include <vector>

#include "autd3/internal/def.hpp"
#include "autd3/internal/exception.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

class Null final : public internal::Gain {
 public:
  Null() = default;

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    return internal::native_methods::AUTDGainNull();
  }
};

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
  [[deprecated("please use add() instead")]] void add_gain(const size_t device_idx, G&& gain) {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.emplace_back(std::make_pair(std::vector{device_idx}, std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain))));
  }

  template <class G>
  [[deprecated("please use add() instead")]] void add(const size_t device_idx, G&& gain) {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _gains.emplace_back(std::make_pair(std::vector{device_idx}, std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain))));
  }

  template <class G>
  void add_by_group(const std::initializer_list<size_t> ids, G&& gain) {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    std::vector<size_t> ids_vec(ids.begin(), ids.end());
    _gains.emplace_back(std::make_pair(ids_vec, std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain))));
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
    auto ptr = internal::native_methods::AUTDGainGrouped();
    for (auto& [ids, gain] : _gains) {
      std::vector<uint32_t> ids_u32;
      std::transform(ids.begin(), ids.end(), std::back_inserter(ids_u32), [](const size_t i) { return static_cast<uint32_t>(i); });
      ptr = AUTDGainGroupedAddByGroup(ptr, ids_u32.data(), static_cast<uint64_t>(ids_u32.size()), gain->gain_ptr(geometry));
    }
    return ptr;
  }

 private:
  std::vector<std::pair<std::vector<size_t>, std::shared_ptr<Gain>>> _gains;
};

class Gain : public internal::Gain {
 public:
  Gain() = default;

  [[nodiscard]] virtual std::vector<internal::native_methods::Drive> calc(const internal::Geometry& geometry) const = 0;

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
    const auto drives = calc(geometry);
    return internal::native_methods::AUTDGainCustom(drives.data(), static_cast<uint64_t>(drives.size()));
  }

  template <class Fn>
  static std::vector<internal::native_methods::Drive> transform(const internal::Geometry& geometry, Fn func) {
    std::vector<internal::native_methods::Drive> drives;
    drives.reserve(geometry.num_transducers());
    std::transform(geometry.begin(), geometry.end(), std::back_inserter(drives), func);
    return drives;
  }
};

class Cache : public internal::Gain {
 public:
  template <class G>
  Cache(G&& g, const internal::Geometry& geometry) {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _drives.resize(geometry.num_transducers());
    if (char err[256]{};
        internal::native_methods::AUTDGainCalc(g.gain_ptr(geometry), geometry.ptr(), _drives.data(), err) == internal::native_methods::AUTD3_ERR)
      throw internal::AUTDException(err);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
    return internal::native_methods::AUTDGainCustom(_drives.data(), static_cast<uint64_t>(_drives.size()));
  }

  [[nodiscard]] const std::vector<internal::native_methods::Drive>& drives() const { return _drives; }
  std::vector<internal::native_methods::Drive>& drives() { return _drives; }

  [[nodiscard]] std::vector<internal::native_methods::Drive>::const_iterator begin() const noexcept { return _drives.begin(); }
  [[nodiscard]] std::vector<internal::native_methods::Drive>::const_iterator end() const noexcept { return _drives.end(); }
  [[nodiscard]] std::vector<internal::native_methods::Drive>::iterator begin() noexcept { return _drives.begin(); }
  [[nodiscard]] std::vector<internal::native_methods::Drive>::iterator end() noexcept { return _drives.end(); }
  [[nodiscard]] const internal::native_methods::Drive& operator[](const size_t i) const { return _drives[i]; }
  [[nodiscard]] internal::native_methods::Drive& operator[](const size_t i) { return _drives[i]; }

 private:
  std::vector<internal::native_methods::Drive> _drives;
};

}  // namespace autd3::gain
