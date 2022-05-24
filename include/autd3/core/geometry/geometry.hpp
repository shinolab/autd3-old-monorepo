// File: geometry.hpp
// Project: geometry
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <numeric>
#include <vector>

#include "device.hpp"
#include "legacy_transducer.hpp"
#include "transducer.hpp"

namespace autd3::core {

/**
 * @brief Geometry of all devices
 */
template <typename T = DynamicTransducer, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct Geometry {
  Geometry() : attenuation(0.0), sound_speed(340.0), _devices() {}

  /**
   * @brief Attenuation coefficient in Np/mm.
   */
  double attenuation;

  /**
   * @brief Speed of sound in m/s.
   */
  double sound_speed;

  /**
   * @brief Number of devices
   */
  [[nodiscard]] size_t num_devices() const noexcept { return _devices.size(); }

  /**
   * @brief Number of transducers
   */
  [[nodiscard]] size_t num_transducers() const noexcept { return _devices.size() * driver::NUM_TRANS_IN_UNIT; }

  /**
   * @brief Center position of all connected devices
   */
  [[nodiscard]] Vector3 center() const {
    Vector3 sum = Vector3::Zero();
    return std::accumulate(begin(), end(), sum,
                           [](const Vector3& acc, const Device<T>& dev) {
                             Vector3 res = acc + dev.center();
                             return res;
                           }) /
           _devices.size();
  }

  /**
   * @brief  Add new device with position and rotation. Note that the transform is done with order: Translate -> Rotate
   * @param position Position of transducer #0, which is the one at the lower-left corner.
   * @param euler_angles ZYZ convention euler angle of the device
   * @return an id of added device
   */
  size_t add_device(const Vector3& position, const Vector3& euler_angles) {
    return add_device(position, Eigen::AngleAxis(euler_angles.x(), Vector3::UnitZ()) * Eigen::AngleAxis(euler_angles.y(), Vector3::UnitY()) *
                                    Eigen::AngleAxis(euler_angles.z(), Vector3::UnitZ()));
  }

  /**
   * @brief Same as add_device(const Vector3&, const Vector3&), but using quaternion rather than zyz euler angles.
   * @param position Position of transducer #0, which is the one at the lower-left corner.
   * @param quaternion rotation quaternion of the device.
   * @return an id of added device
   */
  size_t add_device(const Vector3& position, const Quaternion& quaternion) {
    const auto device_id = _devices.size();
    _devices.emplace_back(device_id, position, quaternion);
    return device_id;
  }

  [[nodiscard]] typename std::vector<Device<T>>::const_iterator begin() const noexcept { return _devices.begin(); }
  [[nodiscard]] typename std::vector<Device<T>>::const_iterator end() const noexcept { return _devices.end(); }
  [[nodiscard]] typename std::vector<Device<T>>::iterator begin() noexcept { return _devices.begin(); }
  [[nodiscard]] typename std::vector<Device<T>>::iterator end() noexcept { return _devices.end(); }

  Device<T>& operator[](const size_t i) { return _devices[i]; }
  const Device<T>& operator[](const size_t i) const { return _devices[i]; }

 private:
  std::vector<Device<T>> _devices;
};

}  // namespace autd3::core
