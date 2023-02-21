// File: geometry.hpp
// Project: geometry
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/02/2023
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
  struct Builder {
    Builder() = default;

    /**
     * @brief Add device to Geometry
     * @tparam T Class inheriting from Device
     * @param device device
     */
    template <typename T>
    auto add_device(T&& device) -> std::enable_if_t<std::is_base_of_v<Device, T>, Builder&> {
      {
        const auto id = _transducers.size();
        const auto transducers = device.get_transducers(id);
        if (transducers.size() > 256) throw std::runtime_error("The maximum number of transducers per device is 256.");
        _transducers.insert(_transducers.end(), transducers.begin(), transducers.end());
        _device_map.emplace_back(transducers.size());
      }
      return *this;
    }

    Builder& attenuation(const driver::autd3_float_t value) {
      _attenuation = value;
      return *this;
    }

    Builder& sound_speed(const driver::autd3_float_t value) {
      _sound_speed = value;
      return *this;
    }

    Builder& mode(const Mode value) {
      _mode = value;
      return *this;
    }

    Builder& legacy_mode() {
      _mode = Mode::Legacy;
      return *this;
    }

    Builder& normal_mode() {
      _mode = Mode::Normal;
      return *this;
    }

    Builder& normal_phase_mode() {
      _mode = Mode::NormalPhase;
      return *this;
    }

#ifdef AUTD3_CAPI
    Geometry* build() { return new Geometry(_mode, _attenuation, _sound_speed, std::move(_transducers), std::move(_device_map)); }
#else
    Geometry build() { return {_mode, _attenuation, _sound_speed, std::move(_transducers), std::move(_device_map)}; }
#endif

   private:
    driver::autd3_float_t _attenuation{0};
    driver::autd3_float_t _sound_speed{
#ifdef AUTD3_USE_METER
        340.0
#else
        340.0e3
#endif
    };
    std::vector<Transducer> _transducers;
    std::vector<size_t> _device_map;
    Mode _mode{Mode::Legacy};
  };

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
    const Vector3 zero = Vector3::Zero();
    return std::accumulate(begin(dev_idx), end(dev_idx), zero,
                           [](const Vector3& acc, const Transducer& tr) {
                             Vector3 res = acc + tr.position();
                             return res;
                           }) /
           _device_map[dev_idx];
  }

  /**
   * @brief Translate all devices
   */
  void translate(const Vector3& t) {
    const Eigen::Translation<driver::autd3_float_t, 3> trans(t);
    affine(trans * Matrix3X3::Identity());
  }

  /**
   * @brief Rotate all devices
   */
  void rotate(const Quaternion& r) {
    const Eigen::Translation<driver::autd3_float_t, 3> trans(0, 0, 0);
    affine(trans * r);
  }

  /**
   * @brief Apply affine transformation to all devices
   */
  void affine(const Vector3& t, const Quaternion& r) {
    const Eigen::Translation<driver::autd3_float_t, 3> trans(t);
    affine(trans * r);
  }

  /**
   * @brief Apply affine transformation to all devices
   */
  void affine(const Affine3& a) {
    std::transform(begin(), end(), begin(), [a](const auto& tr) {
      const auto id = tr.id();
      const Vector3 pos = a * tr.position();
      const Quaternion rot(a.linear() * tr.rotation().toRotationMatrix());
      const auto mod_delay = tr.mod_delay();
      const auto cycle = tr.cycle();
      return Transducer(id, pos, rot, mod_delay, cycle);
    });
  }

  /**
   * @brief Translate the device
   */
  void translate(const size_t dev_idx, const Vector3& t) {
    const Eigen::Translation<driver::autd3_float_t, 3> trans(t);
    affine(dev_idx, trans * Matrix3X3::Identity());
  }

  /**
   * @brief Rotate the device
   */
  void rotate(const size_t dev_idx, const Quaternion& r) {
    const Eigen::Translation<driver::autd3_float_t, 3> trans(0, 0, 0);
    affine(dev_idx, trans * r);
  }

  /**
   * @brief Apply affine transformation to all devices
   */
  void affine(const size_t dev_idx, const Vector3& t, const Quaternion& r) {
    const Eigen::Translation<driver::autd3_float_t, 3> trans(t);
    affine(dev_idx, trans * r);
  }

  /**
   * @brief Apply affine transformation to the device
   */
  void affine(const size_t dev_idx, const Affine3& a) {
    std::transform(begin(dev_idx), end(dev_idx), begin(dev_idx), [a](const auto& tr) {
      const auto id = tr.id();
      const Vector3 pos = a * tr.position();
      const Quaternion rot(a.linear() * tr.rotation().toRotationMatrix());
      const auto mod_delay = tr.mod_delay();
      const auto cycle = tr.cycle();
      return Transducer(id, pos, rot, mod_delay, cycle);
    });
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

  [[nodiscard]] std::vector<Transducer>::const_iterator begin(const size_t dev_idx) const {
    if (dev_idx >= _device_map.size()) throw std::out_of_range("Device index is out of range");
    const auto start_idx =
        std::accumulate(_device_map.begin(), _device_map.begin() + static_cast<decltype(_device_map)::difference_type>(dev_idx), size_t{0});
    return _transducers.begin() + static_cast<decltype(_transducers)::difference_type>(start_idx);
  }
  [[nodiscard]] std::vector<Transducer>::const_iterator end(const size_t dev_idx) const {
    if (dev_idx >= _device_map.size()) throw std::out_of_range("Device index is out of range");
    const auto end_idx =
        std::accumulate(_device_map.begin(), _device_map.begin() + static_cast<decltype(_device_map)::difference_type>(dev_idx), size_t{0}) +
        _device_map[dev_idx];
    return begin() + static_cast<decltype(_transducers)::difference_type>(end_idx);
  }
  [[nodiscard]] std::vector<Transducer>::iterator begin(const size_t dev_idx) {
    if (dev_idx >= _device_map.size()) throw std::out_of_range("Device index is out of range");
    const auto start_idx =
        std::accumulate(_device_map.begin(), _device_map.begin() + static_cast<decltype(_device_map)::difference_type>(dev_idx), size_t{0});
    return _transducers.begin() + static_cast<decltype(_transducers)::difference_type>(start_idx);
  }
  [[nodiscard]] std::vector<Transducer>::iterator end(const size_t dev_idx) {
    if (dev_idx >= _device_map.size()) throw std::out_of_range("Device index is out of range");
    const auto end_idx =
        std::accumulate(_device_map.begin(), _device_map.begin() + static_cast<decltype(_device_map)::difference_type>(dev_idx), size_t{0}) +
        _device_map[dev_idx];
    return begin() + static_cast<decltype(_transducers)::difference_type>(end_idx);
  }

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

  /**
   * Set speed of sound from temperature
   * @param temp temperature in Celsius degree
   * @param k Heat capacity ratio
   * @param r Gas constant [J K^-1 mol^-1]
   * @param m Molar mass [kg mod^-1]
   */
  void set_sound_speed_from_temp(driver::autd3_float_t temp, driver::autd3_float_t k = static_cast<driver::autd3_float_t>(1.4),
                                 driver::autd3_float_t r = static_cast<driver::autd3_float_t>(8.31446261815324),
                                 driver::autd3_float_t m = static_cast<driver::autd3_float_t>(28.9647e-3)) {
#ifdef AUTD3_USE_METER
    sound_speed = std::sqrt(k * r * (static_cast<driver::autd3_float_t>(273.15) + temp) / m);
#else
    sound_speed = std::sqrt(k * r * (static_cast<driver::autd3_float_t>(273.15) + temp) / m) * static_cast<driver::autd3_float_t>(1e3);
#endif
  }

 private:
  Geometry(const Mode mode, const driver::autd3_float_t attenuation, const driver::autd3_float_t sound_speed, std::vector<Transducer> transducers,
           std::vector<size_t> device_map)
      : mode(mode), attenuation(attenuation), sound_speed(sound_speed), _transducers(std::move(transducers)), _device_map(std::move(device_map)) {}

  std::vector<Transducer> _transducers;
  std::vector<size_t> _device_map;
};

}  // namespace autd3::core
