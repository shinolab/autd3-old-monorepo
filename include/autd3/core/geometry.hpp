// File: geometry.hpp
// Project: geometry
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <numeric>
#include <vector>

#include "autd3/core/mode.hpp"
#include "autd3/core/transducer.hpp"

namespace autd3::core {

/**
 * \brief Device consists of transducers
 */
struct Device {
  Device() = default;
  virtual ~Device() = default;
  Device(const Device& v) noexcept = default;
  Device& operator=(const Device& obj) = default;
  Device(Device&& obj) = default;
  Device& operator=(Device&& obj) = default;

  [[nodiscard]] virtual std::vector<Transducer> get_transducers(size_t start_id) const = 0;
};

/**
 * @brief Geometry of all transducers
 */
struct Geometry {
  Geometry()
      : attenuation(0.0),
        sound_speed(
#ifdef AUTD3_USE_METER
            340.0)
#else
            340.0e3)
#endif
  {
  }

  /**
   * @brief Number of devices
   */
  [[nodiscard]] size_t num_devices() const noexcept { return _device_map.size(); }

  /**
   * @brief Number of transducers
   */
  [[nodiscard]] size_t num_transducers() const noexcept { return _transducers.size(); }

  /**
   * @brief Center position of all transducers
   */
  [[nodiscard]] Vector3 center() const {
    if (_transducers.empty()) return Vector3::Zero();
    const Vector3 zero = Vector3::Zero();
    return std::accumulate(begin(), end(), zero,
                           [](const Vector3& acc, const Transducer& tr) {
                             Vector3 res = acc + tr.position();
                             return res;
                           }) /
           _transducers.size();
  }

  /**
   * @brief Center position of the device
   */
  [[nodiscard]] Vector3 center_of(const size_t dev_idx) const {
    if (dev_idx >= _device_map.size()) return Vector3::Zero();
    if (_device_map[dev_idx] == 0) return Vector3::Zero();
    const auto start_idx =
        std::accumulate(_device_map.begin(), _device_map.begin() + static_cast<decltype(_device_map)::difference_type>(dev_idx), size_t{0});
    const Vector3 zero = Vector3::Zero();
    return std::accumulate(begin() + static_cast<decltype(_transducers)::difference_type>(start_idx),
                           begin() + static_cast<decltype(_transducers)::difference_type>(start_idx + _device_map[dev_idx]), zero,
                           [](const Vector3& acc, const Transducer& tr) {
                             Vector3 res = acc + tr.position();
                             return res;
                           }) /
           _device_map[dev_idx];
  }

  /**
   * @brief Add device to Geometry
   * @tparam T Class inheriting from Device
   * @param device device
   */
  template <typename T>
  auto add_device(T&& device) -> std::enable_if_t<std::is_base_of_v<Device, T>> {
    const auto id = _transducers.size();
    const auto transducers = device.get_transducers(id);
    if (transducers.size() > 256) throw std::runtime_error("The maximum number of transducers per device is 256.");
    _transducers.insert(_transducers.end(), transducers.begin(), transducers.end());
    _device_map.emplace_back(transducers.size());
  }

  /**
   * @return device_map contains the number of transducers each device has
   */
  [[nodiscard]] const std::vector<size_t>& device_map() const noexcept { return _device_map; }

  [[nodiscard]] std::vector<uint16_t> cycles() const {
    std::vector<uint16_t> cycles;
    cycles.reserve(num_transducers());
    std::transform(begin(), end(), std::back_inserter(cycles), [](const auto& tr) { return tr.cycle(); });
    return cycles;
  }

  [[nodiscard]] std::vector<Transducer>::const_iterator begin() const noexcept { return _transducers.begin(); }
  [[nodiscard]] std::vector<Transducer>::const_iterator end() const noexcept { return _transducers.end(); }
  [[nodiscard]] std::vector<Transducer>::iterator begin() noexcept { return _transducers.begin(); }
  [[nodiscard]] std::vector<Transducer>::iterator end() noexcept { return _transducers.end(); }
  [[nodiscard]] const Transducer& operator[](const size_t i) const { return _transducers[i]; }
  [[nodiscard]] Transducer& operator[](const size_t i) { return _transducers[i]; }

  /**
   * @brief Drive mode
   */
  Mode mode{Mode::Legacy};

  /**
   * @brief Attenuation coefficient.
   */
  driver::autd3_float_t attenuation;

  /**
   * @brief Speed of sound.
   */
  driver::autd3_float_t sound_speed;

 private:
  std::vector<Transducer> _transducers;
  std::vector<size_t> _device_map;
};

}  // namespace autd3::core
