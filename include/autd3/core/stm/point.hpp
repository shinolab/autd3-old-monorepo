// File: point.hpp
// Project: stm
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <utility>
#include <vector>

#include "autd3/driver/driver.hpp"
#include "stm.hpp"

namespace autd3::core {

/**
 * @brief Control point and duty shift used in PointSTM
 */
struct Point {
  /**
   * @brief Control point
   */
  Vector3 point;
  /**
   * @brief duty shift. The duty ratio will be (50% >> duty_shift).
   */
  uint8_t shift;

  explicit Point(Vector3 point, const uint8_t shift = 0) : point(std::move(point)), shift(shift) {}
  ~Point() = default;
  Point(const Point& v) noexcept = default;
  Point& operator=(const Point& obj) = default;
  Point(Point&& obj) = default;
  Point& operator=(Point&& obj) = default;
};

/**
 * @brief PointSTM provides a function to display the focus sequentially and periodically.
 * @details PointSTM uses a timer on the FPGA to ensure that the focus is precisely timed.
 * PointSTM currently has the following three limitations.
 * 1. The maximum number of control points is driver::POINT_STM_BUF_SIZE_MAX.
 * 2. Only a single focus can be displayed at a certain moment.
 */
struct PointSTM final : STM {
  using value_type = Point;

  explicit PointSTM(const double sound_speed) : STM(), sound_speed(sound_speed), _sent(0) {}

  /**
   * @brief Set frequency of the STM
   * @param[in] freq Frequency of the STM
   * @details STM mode has some constraints, which determine the actual frequency of the STM.
   * @return double Actual frequency of STM
   */
  double set_frequency(const double freq) override {
    const auto sample_freq = static_cast<double>(size()) * freq;
    _freq_div = static_cast<uint32_t>(std::round(static_cast<double>(driver::FPGA_CLK_FREQ) / sample_freq));
    return frequency();
  }

  /**
   * @brief Add control point
   * @param[in] point control point
   * @param[in] duty_shift duty shift. The duty ratio will be (50% >> duty_shift).
   */
  void add(const Vector3& point, uint8_t duty_shift = 0) { _points.emplace_back(point, duty_shift); }

  void push_back(const value_type& v) { _points.emplace_back(v); }

  [[nodiscard]] size_t size() const override { return _points.size(); }

  bool init() override {
    _sent = 0;
    return true;
  }

  bool pack(const std::unique_ptr<const driver::Driver>& driver, const std::unique_ptr<const core::Mode>&, const Geometry& geometry,
            driver::TxDatagram& tx) override {
    driver->point_stm_header(tx);

    if (is_finished()) return true;

    std::vector<std::vector<driver::STMFocus>> points;
    points.reserve(geometry.device_map().size());
    const auto send_size = driver->point_stm_send_size(_points.size(), _sent, geometry.device_map());

    for (size_t i = 0; i < geometry.device_map().size(); i++) {
      std::vector<driver::STMFocus> lp;
      lp.reserve(send_size);
      const auto src = _points.data() + _sent;

      const auto idx = i == 0 ? 0 : geometry.device_map()[i - 1];
      const Vector3 origin = geometry[idx].position();
      const Quaternion rotation = geometry[idx].rotation();
      const Eigen::Transform<double, 3, Eigen::Affine> transform_matrix = Eigen::Translation<double, 3>(origin) * rotation;
      const Eigen::Transform<double, 3, Eigen::Affine> trans_inv = transform_matrix.inverse();

      std::transform(src, src + send_size, std::back_inserter(lp), [&trans_inv](const auto& p) {
        const auto homo = Vector4(p.point[0], p.point[1], p.point[2], 1.0);
        const Vector4 local_position = trans_inv * homo;
        return driver::STMFocus(local_position.x(), local_position.y(), local_position.z(), p.shift);
      });
    }

    return driver->point_stm_body(points, _sent, _points.size(), this->_freq_div, sound_speed, tx);
  }

  [[nodiscard]] bool is_finished() const override { return _sent == _points.size(); }

  /**
   * @brief Speed of sound.
   */
  double sound_speed;

 private:
  std::vector<Point> _points;
  size_t _sent;
};

}  // namespace autd3::core
