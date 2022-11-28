// File: autd3_device.hpp
// Project: autd3
// Created Date: 24/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <vector>

#include "autd3/core/geometry.hpp"
#include "autd3/core/transducer.hpp"

namespace autd3 {
/**
 * \brief An AUTD3 Device
 */
struct AUTD3 final : core::Device {
  static constexpr size_t NUM_TRANS_IN_UNIT = 249;
  static constexpr size_t NUM_TRANS_X = 18;
  static constexpr size_t NUM_TRANS_Y = 14;
#ifdef AUTD3_USE_METER
  static constexpr double TRANS_SPACING = 10.16e-3;
  static constexpr double DEVICE_WIDTH = 192.0e-3;
  static constexpr double DEVICE_HEIGHT = 151.4e-3;
#else
  static constexpr double TRANS_SPACING = 10.16;
  static constexpr double DEVICE_WIDTH = 192.0;
  static constexpr double DEVICE_HEIGHT = 151.4;
#endif

  static constexpr double TRANS_SPACING_MM = 10.16;

  template <typename T>
  static constexpr auto is_missing_transducer(T x, T y) -> std::enable_if_t<std::is_integral_v<T>, bool> {
    return y == 1 && (x == 1 || x == 2 || x == 16);
  }

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
  explicit AUTD3(core::Vector3 position, core::Quaternion rotation) : Device(), _position(std::move(position)), _rotation(std::move(rotation)) {}

  /**
   * @brief Create new device with position and rotation. Note that the transform is done with order: Translate -> Rotate
   * @param position Position of transducer #0, which is the one at the lower-left corner.
   * @param zyz_euler_angles ZYZ convention euler angle of the device
   */
  explicit AUTD3(core::Vector3 position, const core::Vector3& zyz_euler_angles)
      : Device(),
        _position(std::move(position)),
        _rotation(Eigen::AngleAxis(zyz_euler_angles.x(), core::Vector3::UnitZ()) * Eigen::AngleAxis(zyz_euler_angles.y(), core::Vector3::UnitY()) *
                  Eigen::AngleAxis(zyz_euler_angles.z(), core::Vector3::UnitZ())) {}

  [[nodiscard]] std::vector<core::Transducer> get_transducers(const size_t start_id) const override {
    std::vector<core::Transducer> transducers;
    const Eigen::Transform<double, 3, Eigen::Affine> transform_matrix = Eigen::Translation<double, 3>(_position) * _rotation;
    transducers.reserve(NUM_TRANS_IN_UNIT);
    size_t i = start_id;
    for (size_t y = 0; y < NUM_TRANS_Y; y++)
      for (size_t x = 0; x < NUM_TRANS_X; x++) {
        if (is_missing_transducer(x, y)) continue;
        const auto local_pos = core::Vector4(static_cast<double>(x) * TRANS_SPACING, static_cast<double>(y) * TRANS_SPACING, 0.0, 1.0);
        const core::Vector4 global_pos = transform_matrix * local_pos;
        transducers.emplace_back(i++, global_pos.head<3>(), _rotation);
      }
    return transducers;
  }

 private:
  const core::Vector3 _position;
  const core::Quaternion _rotation;
};

}  // namespace autd3
