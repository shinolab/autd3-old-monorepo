// File: geometry.hpp
// Project: geometry
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <numeric>
#include <vector>

#include "mode.hpp"
#include "transducer.hpp"

namespace autd3::core {

/**
 * @brief Geometry of all devices
 */
struct Geometry {
  /**
   * \brief Device contains an AUTD device geometry.
   */
  struct Device {
    explicit Device(const size_t id, const Vector3& position, const Quaternion& rotation) : _origin(position) {
      const Eigen::Transform<double, 3, Eigen::Affine> transform_matrix = Eigen::Translation<double, 3>(position) * rotation;
      const Vector3 x_direction = rotation * Vector3(1, 0, 0);
      const Vector3 y_direction = rotation * Vector3(0, 1, 0);
      const Vector3 z_direction = rotation * Vector3(0, 0, 1);

      _transducers.reserve(driver::NUM_TRANS_IN_UNIT);
      size_t i = id * driver::NUM_TRANS_IN_UNIT;
      for (size_t y = 0; y < driver::NUM_TRANS_Y; y++)
        for (size_t x = 0; x < driver::NUM_TRANS_X; x++) {
          if (driver::is_missing_transducer(x, y)) continue;
          const auto local_pos =
              Vector4(static_cast<double>(x) * driver::TRANS_SPACING_MM, static_cast<double>(y) * driver::TRANS_SPACING_MM, 0.0, 1.0);
          const Vector4 global_pos = transform_matrix * local_pos;
          _transducers.emplace_back(i++, global_pos.head<3>(), x_direction, y_direction, z_direction);
        }
      _trans_inv = transform_matrix.inverse();
    }

    Device() = delete;
    ~Device() = default;
    Device(const Device& v) noexcept = default;
    Device& operator=(const Device& obj) = default;
    Device(Device&& obj) = default;
    Device& operator=(Device&& obj) = default;

    /**
     * @brief Center position of this device
     */
    [[nodiscard]] Vector3 center() const {
      Vector3 sum = Vector3::Zero();
      return std::accumulate(begin(), end(), sum,
                             [](const Vector3& acc, const Transducer& tr) {
                               Vector3 res = acc + tr.position();
                               return res;
                             }) /
             _transducers.size();
    }

    /**
     * @brief Convert a global position to a local position
     */
    [[nodiscard]] Vector3 to_local_position(const Vector3& global_position) const {
      const auto homo = Vector4(global_position[0], global_position[1], global_position[2], 1.0);
      const Vector4 local_position = _trans_inv * homo;
      return local_position.head<3>();
    }

    [[nodiscard]] std::vector<Transducer>::const_iterator begin() const noexcept { return _transducers.begin(); }
    [[nodiscard]] std::vector<Transducer>::const_iterator end() const noexcept { return _transducers.end(); }
    [[nodiscard]] std::vector<Transducer>::iterator begin() noexcept { return _transducers.begin(); }
    [[nodiscard]] std::vector<Transducer>::iterator end() noexcept { return _transducers.end(); }

    const Transducer& operator[](const size_t i) const { return _transducers[i]; }
    Transducer& operator[](const size_t i) { return _transducers[i]; }

   private:
    std::vector<Transducer> _transducers;
    const Vector3 _origin;
    Eigen::Transform<double, 3, Eigen::Affine> _trans_inv;
  };

  Geometry() : attenuation(0.0), sound_speed(340.0), _devices(), _mode(std::make_unique<LegacyMode>()) {}

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
    if (_devices.size() == 0) return sum;
    return std::accumulate(begin(), end(), sum,
                           [](const Vector3& acc, const Device& dev) {
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

  [[nodiscard]] std::vector<Device>::const_iterator begin() const noexcept { return _devices.begin(); }
  [[nodiscard]] std::vector<Device>::const_iterator end() const noexcept { return _devices.end(); }
  [[nodiscard]] std::vector<Device>::iterator begin() noexcept { return _devices.begin(); }
  [[nodiscard]] std::vector<Device>::iterator end() noexcept { return _devices.end(); }

  Device& operator[](const size_t i) { return _devices[i]; }
  const Device& operator[](const size_t i) const { return _devices[i]; }

  const std::unique_ptr<Mode>& mode() const { return _mode; }
  std::unique_ptr<Mode>& mode() { return _mode; }

 private:
  std::vector<Device> _devices;
  std::unique_ptr<Mode> _mode;
};

}  // namespace autd3::core
