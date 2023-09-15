// File: geometry.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 15/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <numeric>
#include <optional>
#include <vector>

#include "autd3/internal/def.hpp"
#include "autd3/internal/geometry/device.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class AUTD3 {
 public:
  /**
   * @brief Number of transducer in an AUTD3 device
   */
  static constexpr size_t NUM_TRANS_IN_UNIT = native_methods::NUM_TRANS_IN_UNIT;
  /**
   * @brief Number of transducer in x-axis of AUTD3 device
   */
  static constexpr size_t NUM_TRANS_IN_X = native_methods::NUM_TRANS_IN_X;
  /**
   * @brief Number of transducer in y-axis of AUTD3 device
   */
  static constexpr size_t NUM_TRANS_IN_Y = native_methods::NUM_TRANS_IN_Y;
  /**
   * @brief Spacing between transducers
   */
  static constexpr double TRANS_SPACING = native_methods::TRANS_SPACING_MM;
  /**
   * @brief Device width including substrate
   */
  static constexpr double DEVICE_WIDTH = native_methods::DEVICE_WIDTH_MM;
  /**
   * @brief Device height including substrate
   */
  static constexpr double DEVICE_HEIGHT = native_methods::DEVICE_HEIGHT_MM;

  /**
   * @brief Constructor
   *
   * @param pos Global position
   * @param rot ZYZ euler angles
   */
  AUTD3(Vector3 pos, Vector3 rot) : _pos(std::move(pos)), _rot(rot) {}

  /**
   * @brief Constructor
   *
   * @param pos Global position
   * @param rot Rotation quaternion
   */
  AUTD3(Vector3 pos, Quaternion rot) : _pos(std::move(pos)), _quat(rot) {}

  [[nodiscard]] Vector3 position() const { return _pos; }
  [[nodiscard]] std::optional<Vector3> euler() const { return _rot; }
  [[nodiscard]] std::optional<Quaternion> quaternion() const { return _quat; }

 private:
  Vector3 _pos{};
  std::optional<Vector3> _rot{std::nullopt};
  std::optional<Quaternion> _quat{std::nullopt};
};

class Geometry {
 public:
  Geometry(const native_methods::GeometryPtr ptr, const native_methods::TransMode mode) : _mode(mode), _ptr(ptr) {
    const auto size = AUTDGeometryNumDevices(_ptr);
    _devices.clear();
    _devices.reserve(size);
    for (uint32_t i = 0; i < size; i++) _devices.emplace_back(static_cast<size_t>(i), AUTDGetDevice(_ptr, i));
  }

  ~Geometry() = default;
  Geometry(const Geometry& v) noexcept = default;
  Geometry& operator=(const Geometry& obj) = default;
  Geometry(Geometry&& obj) = default;
  Geometry& operator=(Geometry&& obj) = default;

  /**
   * @brief Only for internal use
   */
  [[nodiscard]] native_methods::TransMode mode() const { return _mode; }

  /**
   * @brief Get the number of devices
   */
  [[nodiscard]] size_t num_devices() const { return _devices.size(); }

  /**
   * @brief Get the number of transducers
   */
  [[nodiscard]] size_t num_transducers() const {
    return std::accumulate(_devices.begin(), _devices.end(), size_t{0}, [](const size_t acc, const Device& d) { return acc + d.num_transducers(); });
  }

  /**
   * @brief Get center position of all transducers
   */
  [[nodiscard]] Vector3 center() const {
    return std::accumulate(_devices.begin(), _devices.end(), Vector3(0, 0, 0),
                           [](const Vector3& acc, const Device& d) -> Vector3 {
                             Vector3 res = acc + d.center();
                             return res;
                           }) /
           static_cast<double>(num_devices());
  }

  [[nodiscard]] std::vector<Device>::const_iterator begin() const noexcept { return _devices.cbegin(); }
  [[nodiscard]] std::vector<Device>::const_iterator end() const noexcept { return _devices.cend(); }
  [[nodiscard]] std::vector<Device>::iterator begin() noexcept { return _devices.begin(); }
  [[nodiscard]] std::vector<Device>::iterator end() noexcept { return _devices.end(); }
  [[nodiscard]] std::vector<Device>::const_iterator cbegin() const noexcept { return _devices.cbegin(); }
  [[nodiscard]] std::vector<Device>::const_iterator cend() const noexcept { return _devices.cend(); }

  [[nodiscard]] const Device& operator[](const size_t i) const { return _devices[i]; }
  [[nodiscard]] Device& operator[](const size_t i) { return _devices[i]; }

  [[nodiscard]] native_methods::GeometryPtr ptr() const noexcept { return _ptr; }

 private:
  native_methods::TransMode _mode;
  native_methods::GeometryPtr _ptr;
  std::vector<Device> _devices{};
};

}  // namespace autd3::internal
