// File: geometry.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/06/2023
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
  static constexpr size_t NUM_TRANS_IN_UNIT = static_cast<size_t>(native_methods::NUM_TRANS_IN_UNIT);
  static constexpr size_t NUM_TRANS_IN_X = static_cast<size_t>(native_methods::NUM_TRANS_IN_X);
  static constexpr size_t NUM_TRANS_IN_Y = static_cast<size_t>(native_methods::NUM_TRANS_IN_Y);
  static constexpr double TRANS_SPACING = native_methods::TRANS_SPACING_MM;
  static constexpr double DEVICE_WIDTH = native_methods::DEVICE_WIDTH_MM;
  static constexpr double DEVICE_HEIGHT = native_methods::DEVICE_HEIGHT_MM;

  AUTD3(Vector3 pos, Vector3 rot) : _pos(std::move(pos)), _rot(rot) {}
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

  [[nodiscard]] native_methods::TransMode mode() const { return _mode; }

  [[nodiscard]] size_t num_devices() const { return static_cast<size_t>(AUTDNumDevices(_ptr)); }

  [[nodiscard]] size_t num_transducers() const { return static_cast<size_t>(AUTDNumTransducers(_ptr)); }

  [[nodiscard]] Vector3 center() const {
    double x, y, z;
    AUTDGeometryCenter(_ptr, &x, &y, &z);
    return {x, y, z};
  }

  [[nodiscard]] Vector3 center_of(const size_t idx) const {
    double x, y, z;
    AUTDGeometryCenterOf(_ptr, static_cast<uint32_t>(idx), &x, &y, &z);
    return {x, y, z};
  }

  [[nodiscard]] double sound_speed() const { return AUTDGetSoundSpeed(_ptr); }
  void set_sound_speed(const double value) const { AUTDSetSoundSpeed(_ptr, value); }
  void set_sound_speed_from_temp(const double temp, const double k = 1.4, const double r = 8.31446261815324, const double m = 28.9647e-3) const {
    AUTDSetSoundSpeedFromTemp(_ptr, temp, k, r, m);
  }

  [[nodiscard]] double attenuation() const { return AUTDGetAttenuation(_ptr); }
  void set_attenuation(const double value) const { AUTDSetAttenuation(_ptr, value); }

  [[nodiscard]] std::vector<Transducer>::const_iterator begin() const noexcept { return _transducers.begin(); }
  [[nodiscard]] std::vector<Transducer>::const_iterator end() const noexcept { return _transducers.end(); }
  [[nodiscard]] std::vector<Transducer>::iterator begin() noexcept { return _transducers.begin(); }
  [[nodiscard]] std::vector<Transducer>::iterator end() noexcept { return _transducers.end(); }

  [[nodiscard]] const Transducer& operator[](const size_t i) const { return _transducers[i]; }
  [[nodiscard]] Transducer& operator[](const size_t i) { return _transducers[i]; }

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
