// File: geometry.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <optional>
#include <vector>

#include "autd3/internal/def.hpp"
#include "autd3/internal/native_methods.hpp"
#include "autd3/internal/transducer.hpp"

namespace autd3::internal {

class AUTD3 {
 public:
  /**
   * @brief Number of transducer in an AUTD3 device
   */
  static constexpr size_t NUM_TRANS_IN_UNIT = static_cast<size_t>(native_methods::NUM_TRANS_IN_UNIT);
  /**
   * @brief Number of transducer in x-axis of AUTD3 device
   */
  static constexpr size_t NUM_TRANS_IN_X = static_cast<size_t>(native_methods::NUM_TRANS_IN_X);
  /**
   * @brief Number of transducer in y-axis of AUTD3 device
   */
  static constexpr size_t NUM_TRANS_IN_Y = static_cast<size_t>(native_methods::NUM_TRANS_IN_Y);
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
  Geometry(const native_methods::GeometryPtr ptr, const native_methods::TransMode mode) : _mode(mode), _ptr(ptr) {}

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
  [[nodiscard]] size_t num_devices() const { return static_cast<size_t>(AUTDNumDevices(_ptr)); }

  /**
   * @brief Get the number of transducers
   */
  [[nodiscard]] size_t num_transducers() const { return static_cast<size_t>(AUTDNumTransducers(_ptr)); }

  /**
   * @brief Get center position of all transducers
   */
  [[nodiscard]] Vector3 center() const {
    Vector3 v;
    AUTDGeometryCenter(_ptr, v.data());
    return v;
  }

  /**
   * @brief Get center position of transducers in the specified device
   */
  [[nodiscard]] Vector3 center_of(const size_t idx) const {
    Vector3 v;
    AUTDGeometryCenterOf(_ptr, static_cast<uint32_t>(idx), v.data());
    return v;
  }

  /**
   * @brief Speed of sound
   */
  [[nodiscard]] double sound_speed() const { return AUTDGetSoundSpeed(_ptr); }

  /**
   * @brief Set speed of sound
   */
  void set_sound_speed(const double value) const { AUTDSetSoundSpeed(_ptr, value); }

  /**
   * @brief Set the sound speed from temperature
   *
   * @param temp Temperature in celsius
   * @param k Ratio of specific heat
   * @param r Gas constant
   * @param m Molar mass
   */
  void set_sound_speed_from_temp(const double temp, const double k = 1.4, const double r = 8.31446261815324, const double m = 28.9647e-3) const {
    AUTDSetSoundSpeedFromTemp(_ptr, temp, k, r, m);
  }

  /**
   * @brief Attenuation coefficientq
   */
  [[nodiscard]] double attenuation() const { return AUTDGetAttenuation(_ptr); }
  /**
   * @brief Set attenuation coefficient
   */
  void set_attenuation(const double value) const { AUTDSetAttenuation(_ptr, value); }

  [[nodiscard]] std::vector<Transducer>::const_iterator begin() const noexcept { return _transducers.begin(); }
  [[nodiscard]] std::vector<Transducer>::const_iterator end() const noexcept { return _transducers.end(); }
  [[nodiscard]] std::vector<Transducer>::iterator begin() noexcept { return _transducers.begin(); }
  [[nodiscard]] std::vector<Transducer>::iterator end() noexcept { return _transducers.end(); }

  [[nodiscard]] const Transducer& operator[](const size_t i) const { return _transducers[i]; }
  [[nodiscard]] Transducer& operator[](const size_t i) { return _transducers[i]; }

  /**
   * @brief Only for internal use
   */
  void configure_transducers() {
    const auto size = num_transducers();
    _transducers.clear();
    _transducers.reserve(size);
    for (uint32_t i = 0; i < size; i++) _transducers.emplace_back(i, _ptr);
  }

  [[nodiscard]] native_methods::GeometryPtr ptr() const noexcept { return _ptr; }

 private:
  native_methods::TransMode _mode;
  native_methods::GeometryPtr _ptr;
  std::vector<Transducer> _transducers{};
};

}  // namespace autd3::internal
