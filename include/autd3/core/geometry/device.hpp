// File: device.hpp
// Project: geometry
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/driver/hardware.hpp"
#include "transducer.hpp"

namespace autd3::core {
template <typename T>
struct Device {
  Device(const size_t id, const Vector3& position, const Quaternion& rotation) : _origin(position) {
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

  [[nodiscard]] Vector3 center() const {
    Vector3 sum = Vector3::Zero();
    return std::accumulate(begin(), end(), sum,
                           [](const Vector3& acc, const T& tr) {
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

  [[nodiscard]] typename std::vector<T>::const_iterator begin() const noexcept { return _transducers.begin(); }
  [[nodiscard]] typename std::vector<T>::const_iterator end() const noexcept { return _transducers.end(); }
  [[nodiscard]] typename std::vector<T>::iterator begin() noexcept { return _transducers.begin(); }
  [[nodiscard]] typename std::vector<T>::iterator end() noexcept { return _transducers.end(); }

  const T& operator[](const size_t i) const { return _transducers[i]; }
  T& operator[](const size_t i) { return _transducers[i]; }

 private:
  std::vector<T> _transducers;
  Vector3 _origin;
  Eigen::Transform<double, 3, Eigen::Affine> _trans_inv;
};
}  // namespace autd3::core
