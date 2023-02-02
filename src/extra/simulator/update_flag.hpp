// File: update_flag.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::extra::simulator {

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26812)
#endif

class UpdateFlags final {
 public:
  enum Value : uint32_t {
    None = 0,
    UpdateSourceDrive = 1 << 0,
    UpdateColorMap = 1 << 1,
    UpdateCameraPos = 1 << 2,
    UpdateSlicePos = 1 << 3,
    UpdateSliceSize = 1 << 4,
    UpdateSourceAlpha = 1 << 5,
    UpdateSourceFlag = 1 << 6,
    SaveImage = 1 << 7,
    UpdateDeviceInfo = 1 << 8,
  };

  UpdateFlags() = default;
  explicit UpdateFlags(const Value value) noexcept : _value(value) {}

  ~UpdateFlags() = default;
  UpdateFlags(const UpdateFlags& v) noexcept = default;
  UpdateFlags& operator=(const UpdateFlags& obj) = default;
  UpdateFlags& operator=(const Value v) noexcept {
    _value = v;
    return *this;
  }
  UpdateFlags(UpdateFlags&& obj) = default;
  UpdateFlags& operator=(UpdateFlags&& obj) = default;

  constexpr bool operator==(const UpdateFlags a) const { return _value == a._value; }
  constexpr bool operator!=(const UpdateFlags a) const { return _value != a._value; }
  constexpr bool operator==(const Value a) const { return _value == a; }
  constexpr bool operator!=(const Value a) const { return _value != a; }

  void set(const Value v) noexcept { _value = static_cast<Value>(_value | v); }

  void remove(const Value v) noexcept { _value = static_cast<Value>(_value & ~v); }

  [[nodiscard]] bool contains(const Value v) const noexcept { return (_value & v) == v; }

  [[nodiscard]] Value value() const noexcept { return _value; }

  static UpdateFlags all() { return UpdateFlags{static_cast<Value>(0xFFFFFFFF)}; }

 private:
  Value _value;
};

#ifdef _MSC_VER
#pragma warning(pop)
#endif

}  // namespace autd3::extra::simulator
