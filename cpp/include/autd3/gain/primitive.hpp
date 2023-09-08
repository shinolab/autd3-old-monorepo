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
#include <memory>
#include <numeric>
#include <optional>
#include <unordered_map>
#include <vector>

#include "autd3/internal/def.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/device.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

/**
 * @brief Gain to cache the result of calculation
 */
template <class G>
class Cache : public internal::Gain {
 public:
  Cache(G g) : _g(std::move(g)) { static_assert(std::is_base_of_v<Gain, G>, "This is not Gain"); }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const std::vector<const internal::Device*>& devices) const override {
    std::vector<uint32_t> device_indices;
    device_indices.reserve(devices.size());
    std::transform(devices.begin(), devices.end(), std::back_inserter(device_indices),
                   [](const internal::Device* dev) { return static_cast<uint32_t>(dev->idx()); });

    if (_cache.size() != devices.size() ||
        std::any_of(device_indices.begin(), device_indices.end(), [this](uint32_t idx) { return _cache.find(idx) == _cache.end(); })) {
      std::vector<internal::native_methods::DevicePtr> device_ptrs;
      device_ptrs.reserve(devices.size());
      std::transform(devices.begin(), devices.end(), std::back_inserter(device_ptrs), [](const internal::Device* dev) { return dev->ptr(); });
      std::vector<std::vector<internal::native_methods::Drive>> drives;
      drives.reserve(devices.size());
      std::transform(devices.begin(), devices.end(), std::back_inserter(drives), [](const internal::Device* dev) {
        std::vector<internal::native_methods::Drive> d;
        d.resize(dev->num_transducers());
        return std::move(d);
      });

      std::vector<internal::native_methods::Drive*> drives_ptrs;
      drives_ptrs.reserve(drives.size());
      std::transform(drives.begin(), drives.end(), std::back_inserter(drives_ptrs),
                     [](std::vector<internal::native_methods::Drive>& d) { return d.data(); });

      if (char err[256]{};
          internal::native_methods::AUTDGainCalc(_g.gain_ptr(devices), device_ptrs.data(), drives_ptrs.data(),
                                                 static_cast<uint32_t>(device_ptrs.size()), err) == internal::native_methods::AUTD3_ERR)
        throw internal::AUTDException(err);
      for (size_t i = 0; i < devices.size(); i++) _cache.emplace(devices[i]->idx(), std::move(drives[i]));
    }

    return std::accumulate(devices.begin(), devices.end(), internal::native_methods::AUTDGainCustom(),
                           [this](internal::native_methods::GainPtr acc, const internal::Device* dev) {
                             return internal::native_methods::AUTDGainCustomSet(acc, static_cast<uint32_t>(dev->idx()), _cache[dev->idx()].data(),
                                                                                static_cast<uint32_t>(_cache[dev->idx()].size()));
                           });
  }

 private:
  G _g;
  mutable std::unordered_map<size_t, std::vector<internal::native_methods::Drive>> _cache;
};

#define AUTD3_IMPL_WITH_CACHE_GAIN(T) \
  [[nodiscard]] Cache<T> with_cache()&& { return Cache(std::move(*this)); }

/**
 * @brief Gain to output nothing
 */
class Null final : public internal::Gain {
 public:
  Null() = default;

  AUTD3_IMPL_WITH_CACHE_GAIN(Null)

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

  AUTD3_IMPL_WITH_CACHE_GAIN(Focus)

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

  AUTD3_IMPL_WITH_CACHE_GAIN(Bessel)

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

  AUTD3_IMPL_WITH_CACHE_GAIN(Plane)

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

template <typename K, class F>
class Group : public internal::Gain {
 public:
  Group(const F& f) : _f(f) {}

  AUTD3_IMPL_WITH_CACHE_GAIN(Group)

  /**
   * @brief Set gain
   *
   * @tparam K Key
   * @tparam G Gain
   * @param key Key
   * @param gain Gain
   */
  template <class G>
  void set(const K key, G&& gain) & {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _map[key] = std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain));
  }

  /**
   * @brief Set gain
   *
   * @tparam K Key
   * @tparam G Gain
   * @param key Key
   * @param gain Gain
   */
  template <class G>
  Group set(const K key, G&& gain) && {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _map[key] = std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain));
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const std::vector<const internal::Device*>& devices) const override {
    std::unordered_map<K, int32_t> keymap;

    std::vector<uint32_t> device_indices;
    device_indices.reserve(devices.size());
    std::transform(devices.begin(), devices.end(), std::back_inserter(device_indices),
                   [](const internal::Device* dev) { return static_cast<uint32_t>(dev->idx()); });

    std::vector<std::vector<int32_t>> map;
    map.reserve(devices.size());
    int32_t k = 0;
    for (const auto* dev : devices) {
      std::vector<int32_t> m;
      m.reserve(dev->num_transducers());
      std::for_each(dev->cbegin(), dev->cend(), [this, dev, &m, &keymap, &k](const auto& tr) {
        auto key = _f(*dev, tr);
        if (key.has_value()) {
          if (keymap.find(key.value()) == keymap.end()) {
            keymap[key.value()] = k++;
          }
          m.emplace_back(keymap[key.value()]);
        } else {
          m.emplace_back(-1);
        }
      });
      map.emplace_back(std::move(m));
    }
    std::vector<int32_t> keys;
    std::vector<internal::native_methods::GainPtr> values;
    for (auto& kv : _map) {
      keys.emplace_back(keymap[kv.first]);
      values.emplace_back(kv.second->gain_ptr(devices));
    }

    std::vector<const int32_t*> map_ptrs;
    map_ptrs.reserve(map.size());
    std::transform(map.begin(), map.end(), std::back_inserter(map_ptrs), [](const std::vector<int32_t>& m) { return m.data(); });

    return internal::native_methods::AUTDGainGroup(device_indices.data(), map_ptrs.data(), static_cast<uint64_t>(map_ptrs.size()), keys.data(),
                                                   values.data(), static_cast<uint64_t>(keys.size()));
  }

 private:
  const F& _f;
  std::unordered_map<K, std::shared_ptr<Gain>> _map;
};

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
