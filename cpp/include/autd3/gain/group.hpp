// File: group.hpp
// Project: gain
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <iterator>
#include <memory>
#include <unordered_map>
#include <vector>

#include "autd3/gain/cache.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/internal/gain.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain {

template <class F>
class Group final : public internal::Gain, public IntoCache<Group<F>>, public IntoTransform<Group<F>> {
 public:
  using key_type = typename std::invoke_result_t<F, const internal::Device&, const internal::Transducer&>::value_type;

  explicit Group(const F& f) : _f(f) {}

  /**
   * @brief Set gain
   *
   * @tparam G Gain
   * @param key Key
   * @param gain Gain
   */
  template <class G>
  void set(const key_type key, G&& gain) & {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _map[key] = std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain));
  }

  /**
   * @brief Set gain
   *
   * @tparam G Gain
   * @param key Key
   * @param gain Gain
   */
  template <class G>
  Group&& set(const key_type key, G&& gain) && {
    static_assert(std::is_base_of_v<Gain, std::remove_reference_t<G>>, "This is not Gain");
    _map[key] = std::make_shared<std::remove_reference_t<G>>(std::forward<G>(gain));
    return std::move(*this);
  }

  [[nodiscard]] internal::native_methods::GainPtr gain_ptr(const internal::Geometry& geometry) const override {
    std::unordered_map<key_type, int32_t> keymap;

    auto view = geometry.devices() | std::views::transform([](const internal::Device& dev) { return static_cast<uint32_t>(dev.idx()); });
    std::vector<uint32_t> device_indices(view.begin(), view.end());

    auto map = internal::native_methods::AUTDGainGroupCreateMap(device_indices.data(), static_cast<uint32_t>(device_indices.size()));
    int32_t k = 0;
    for (const auto& dev : geometry.devices()) {
      std::vector<int32_t> m;
      m.reserve(dev.num_transducers());
      std::for_each(dev.cbegin(), dev.cend(), [this, &dev, &m, &keymap, &k](const auto& tr) {
        if (auto key = this->_f(dev, tr); key.has_value()) {
          if (!keymap.contains(key.value())) {
            keymap[key.value()] = k++;
          }
          m.emplace_back(keymap[key.value()]);
        } else {
          m.emplace_back(-1);
        }
      });
      map = AUTDGainGroupMapSet(map, static_cast<uint32_t>(dev.idx()), m.data());
    }
    std::vector<int32_t> keys;
    std::vector<internal::native_methods::GainPtr> values;
    for (auto& kv : _map) {
      if (!keymap.contains(kv.first)) throw internal::AUTDException("Unknown group key");
      keys.emplace_back(keymap[kv.first]);
      values.emplace_back(kv.second->gain_ptr(geometry));
    }

    return AUTDGainGroup(map, keys.data(), values.data(), static_cast<uint32_t>(keys.size()));
  }

 private:
  const F& _f;
  std::unordered_map<key_type, std::shared_ptr<Gain>> _map;
};

}  // namespace autd3::gain
