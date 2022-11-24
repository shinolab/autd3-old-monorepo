// File: geometry.hpp
// Project: geometry
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <numeric>
#include <vector>

#include "transducer.hpp"

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
 * \brief An AUTD3 Device
 */
struct AUTD3 final : Device {
  ~AUTD3() override = default;
  AUTD3(const AUTD3& v) noexcept = default;
  AUTD3& operator=(const AUTD3& obj) = delete;
  AUTD3(AUTD3&& obj) = default;
  AUTD3& operator=(AUTD3&& obj) = delete;

  /**
   * @brief Same as AUTD3(const Vector3&, const Vector3&), but using quaternion rather than zyz euler angles.
   * @param position Position of transducer #0, which is the one at the lower-left corner.
   * @param rotation rotation quaternion of the device.
   */
  explicit AUTD3(Vector3 position, Quaternion rotation) : Device(), _position(std::move(position)), _rotation(std::move(rotation)) {}

  /**
   * @brief Create new device with position and rotation. Note that the transform is done with order: Translate -> Rotate
   * @param position Position of transducer #0, which is the one at the lower-left corner.
   * @param zyz_euler_angles ZYZ convention euler angle of the device
   */
  explicit AUTD3(Vector3 position, const Vector3& zyz_euler_angles)
      : Device(),
        _position(std::move(position)),
        _rotation(Eigen::AngleAxis(zyz_euler_angles.x(), Vector3::UnitZ()) * Eigen::AngleAxis(zyz_euler_angles.y(), Vector3::UnitY()) *
                  Eigen::AngleAxis(zyz_euler_angles.z(), Vector3::UnitZ())) {}

  [[nodiscard]] std::vector<Transducer> get_transducers(const size_t start_id) const override {
    std::vector<Transducer> transducers;
    const Eigen::Transform<double, 3, Eigen::Affine> transform_matrix = Eigen::Translation<double, 3>(_position) * _rotation;
    transducers.reserve(driver::NUM_TRANS_IN_UNIT);
    size_t i = start_id * driver::NUM_TRANS_IN_UNIT;
    for (size_t y = 0; y < driver::NUM_TRANS_Y; y++)
      for (size_t x = 0; x < driver::NUM_TRANS_X; x++) {
        if (driver::is_missing_transducer(x, y)) continue;
        const auto local_pos = Vector4(static_cast<double>(x) * driver::TRANS_SPACING, static_cast<double>(y) * driver::TRANS_SPACING, 0.0, 1.0);
        const Vector4 global_pos = transform_matrix * local_pos;
        transducers.emplace_back(i++, global_pos.head<3>(), _rotation);
      }
    return transducers;
  }

 private:
  const Vector3 _position;
  const Quaternion _rotation;
};

/**
 * @brief Geometry of all devices
 */
struct Geometry {
  Geometry() = default;

  /**
   * @brief Number of transducers
   */
  [[nodiscard]] size_t num_transducers() const noexcept { return _transducers.size(); }

  /**
   * @brief Center position of all devices
   */
  [[nodiscard]] Vector3 center() const {
    const Vector3 zero = Vector3::Zero();
    return std::accumulate(begin(), end(), zero,
                           [](const Vector3& acc, const Transducer& tr) {
                             Vector3 res = acc + tr.position();
                             return res;
                           }) /
           _transducers.size();
  }

  template <typename T>
  auto add_device(T&& device) -> std::enable_if_t<std::is_base_of_v<Device, T>> {
    const auto id = _transducers.size();
    const auto transducers = device.get_transducers(id);
    _transducers.insert(_transducers.end(), transducers.begin(), transducers.end());
    _device_map.emplace_back(transducers.size());
  }

  [[nodiscard]] std::vector<Transducer>::const_iterator begin() const noexcept { return _transducers.begin(); }
  [[nodiscard]] std::vector<Transducer>::const_iterator end() const noexcept { return _transducers.end(); }
  [[nodiscard]] std::vector<Transducer>::iterator begin() noexcept { return _transducers.begin(); }
  [[nodiscard]] std::vector<Transducer>::iterator end() noexcept { return _transducers.end(); }
  [[nodiscard]] const Transducer& operator[](const size_t i) const { return _transducers[i]; }
  [[nodiscard]] Transducer& operator[](const size_t i) { return _transducers[i]; }

 private:
  std::vector<Transducer> _transducers;
  std::vector<size_t> _device_map;
};

}  // namespace autd3::core
