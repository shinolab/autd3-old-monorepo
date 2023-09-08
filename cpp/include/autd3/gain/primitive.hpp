// File: primitive.hpp
// Project: gain
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iterator>
#include <numeric>
#include <optional>
#include <vector>

#include "autd3/internal/def.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/device.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

///**
// * @brief Gain to cache the result of calculation
// */
// class Cache : public internal::Gain {
// public:
//  template <class G>
//  Cache(G&& g) {
//    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
//    _drives.resize(geometry.num_transducers());
//    if (char err[256]{};
//        internal::native_methods::AUTDGainCalc(g.gain_ptr(geometry), geometry.ptr(), _drives.data(), err) == internal::native_methods::AUTD3_ERR)
//      throw internal::AUTDException(err);
//  }
//
//  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry&) const override {
//    return internal::native_methods::AUTDGainCustom(_drives.data(), static_cast<uint64_t>(_drives.size()));
//  }
//
//  [[nodiscard]] const std::vector<internal::native_methods::Drive>& drives() const { return _drives; }
//  std::vector<internal::native_methods::Drive>& drives() { return _drives; }
//
// private:
//  std::vector<internal::native_methods::Drive> _drives;
//};

//#define AUTD3_IMPL_WITH_CACHE_GAIN \
//  Cache with_cache(const internal::Geometry& geometry) { return Cache(std::move(*this), geometry); }
#define AUTD3_IMPL_WITH_CACHE_GAIN

/**
 * @brief Gain to output nothing
 */
class Null final : public internal::Gain {
 public:
  Null() = default;

  AUTD3_IMPL_WITH_CACHE_GAIN

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const std::vector<const internal::Device*>&) const override {
    return internal::native_methods::AUTDGainNull();
  }
};

/**
 * @brief Gain to produce single focal point
 */
class Focus final : public internal::Gain {
 public:
  explicit Focus(internal::Vector3 p) : _p(std::move(p)) {}

  AUTD3_IMPL_WITH_CACHE_GAIN

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (from 0 to 1)
   */
  void with_amp(const double amp) & { _amp = amp; }

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (from 0 to 1)
   */
  [[nodiscard]] Focus&& with_amp(const double amp) && {
    _amp = amp;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const std::vector<const internal::Device*>&) const override {
    auto ptr = internal::native_methods::AUTDGainFocus(_p.x(), _p.y(), _p.z());
    if (_amp.has_value()) ptr = AUTDGainFocusWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  internal::Vector3 _p;
  std::optional<double> _amp;
};

/**
 * @brief Gain to produce a Bessel beam
 */
class Bessel final : public internal::Gain {
 public:
  explicit Bessel(internal::Vector3 p, internal::Vector3 d, const double theta) : _p(std::move(p)), _d(std::move(d)), _theta(theta) {}

  AUTD3_IMPL_WITH_CACHE_GAIN

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (from 0 to 1)
   */
  void with_amp(const double amp) & { _amp = amp; }

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (from 0 to 1)
   */
  [[nodiscard]] Bessel&& with_amp(const double amp) && {
    _amp = amp;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const std::vector<const internal::Device*>&) const override {
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

/**
 * @brief Gain to produce a plane wave
 */
class Plane final : public internal::Gain {
 public:
  explicit Plane(internal::Vector3 d) : _d(std::move(d)) {}

  AUTD3_IMPL_WITH_CACHE_GAIN

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (from 0 to 1)
   * @return Plane
   */
  void with_amp(const double amp) & { _amp = amp; }

  /**
   * @brief set amplitude
   *
   * @param amp normalized amplitude (from 0 to 1)
   * @return Plane
   */
  [[nodiscard]] Plane&& with_amp(const double amp) && {
    _amp = amp;
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const std::vector<const internal::Device*>&) const override {
    auto ptr = internal::native_methods::AUTDGainPlane(_d.x(), _d.y(), _d.z());
    if (_amp.has_value()) ptr = AUTDGainPlaneWithAmp(ptr, _amp.value());
    return ptr;
  }

 private:
  internal::Vector3 _d;
  std::optional<double> _amp;
};

// template <typename K, class F>
// class GroupByTransducer : public internal::Gain {
//  public:
//   GroupByTransducer(const F& f) : _f(f) {}
//
//   AUTD3_IMPL_WITH_CACHE_GAIN
//
//   /**
//    * @brief Set gain
//    *
//    * @tparam K Key
//    * @tparam G Gain
//    * @param key Key
//    * @param gain Gain
//    */
//   template <class G>
//   void set(const K key, G&& gain) & {
//     static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
//     _map[key] = std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain));
//   }
//
//   /**
//    * @brief Set gain
//    *
//    * @tparam K Key
//    * @tparam G Gain
//    * @param key Key
//    * @param gain Gain
//    */
//   template <class G>
//   GroupByTransducer set(const K key, G&& gain) && {
//     static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
//     _map[key] = std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain));
//     return std::move(*this);
//   }
//
//   [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
//     std::unordered_map<K, int32_t> keymap;
//     std::vector<int32_t> map;
//     map.reserve(geometry.num_transducers());
//     int32_t k = 0;
//     for (auto& tr : geometry) {
//       auto key = _f(tr);
//       if (key.has_value()) {
//         if (keymap.find(key.value()) == keymap.end()) {
//           keymap[key.value()] = k++;
//         }
//         map.emplace_back(keymap[key.value()]);
//       } else {
//         map.emplace_back(-1);
//       }
//     }
//     std::vector<int32_t> keys;
//     std::vector<internal::native_methods::GainPtr> values;
//     for (auto& kv : _map) {
//       keys.emplace_back(keymap[kv.first]);
//       values.emplace_back(kv.second->gain_ptr(geometry));
//     }
//     return internal::native_methods::AUTDGainGroupByTransducer(map.data(), static_cast<uint64_t>(map.size()), keys.data(), values.data(),
//                                                                static_cast<uint64_t>(keys.size()));
//   }
//
//  private:
//   const F& _f;
//   std::unordered_map<K, std::shared_ptr<Gain>> _map;
// };

class Gain : public internal::Gain {
 public:
  Gain() = default;

  [[nodiscard]] virtual std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> calc(
      const std::vector<const internal::Device*>& devices) const = 0;

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const std::vector<const internal::Device*>& devices) const override {
    const auto drives = calc(devices);
    return std::accumulate(drives.begin(), drives.end(), internal::native_methods::AUTDGainCustom(),
                           [](internal::native_methods::GainPtr acc, const std::pair<size_t, std::vector<internal::native_methods::Drive>>& kv) {
                             return AUTDGainCustomSet(acc, static_cast<uint32_t>(kv.first), kv.second.data(),
                                                      static_cast<uint32_t>(kv.second.size()));
                           });
  }

  template <class Fn>
  [[nodiscard]] static std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> transform(
      const std::vector<const internal::Device*>& devices, Fn func) {
    std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> drives_map;
    std::for_each(devices.begin(), devices.end(), [&drives_map, &func](const internal::Device* dev) {
      std::vector<internal::native_methods::Drive> drives;
      drives.reserve(dev->num_transducers());
      std::transform(dev->cbegin(), dev->cend(), std::back_inserter(drives),
                     [&dev, &drives_map, &func](const internal::Transducer& tr) { return func(*dev, tr); });
      drives_map[dev->idx()] = std::move(drives);
    });
    return drives_map;
  }
};

}  // namespace autd3::gain
