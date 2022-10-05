// File: update_flag.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 05/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::extra::simulator {

#pragma warning(push)
#pragma warning(disable : 26812)
class UpdateFlags final {
 public:
  enum VALUE : uint32_t {
    NONE = 0,
    UPDATE_SOURCE_DRIVE = 1 << 0,
    UPDATE_COLOR_MAP = 1 << 1,
    UPDATE_CAMERA_POS = 1 << 2,
    UPDATE_SLICE_POS = 1 << 3,
    UPDATE_SLICE_SIZE = 1 << 4,
    UPDATE_SOURCE_ALPHA = 1 << 5,
    UPDATE_SOURCE_FLAG = 1 << 6,
    INIT_SOURCE = 1 << 7,
  };

  UpdateFlags() = default;
  explicit UpdateFlags(const VALUE value) noexcept : _value(value) {}

  ~UpdateFlags() = default;
  UpdateFlags(const UpdateFlags& v) noexcept = default;
  UpdateFlags& operator=(const UpdateFlags& obj) = default;
  UpdateFlags& operator=(const VALUE v) noexcept {
    _value = v;
    return *this;
  }
  UpdateFlags(UpdateFlags&& obj) = default;
  UpdateFlags& operator=(UpdateFlags&& obj) = default;

  constexpr bool operator==(const UpdateFlags a) const { return _value == a._value; }
  constexpr bool operator!=(const UpdateFlags a) const { return _value != a._value; }
  constexpr bool operator==(const VALUE a) const { return _value == a; }
  constexpr bool operator!=(const VALUE a) const { return _value != a; }

  void set(const VALUE v) noexcept { _value = static_cast<VALUE>(_value | v); }

  void remove(const VALUE v) noexcept { _value = static_cast<VALUE>(_value & ~v); }

  [[nodiscard]] bool contains(const VALUE v) const noexcept { return (_value & v) == v; }

  [[nodiscard]] VALUE value() const noexcept { return _value; }

  static UpdateFlags all() { return UpdateFlags{static_cast<VALUE>(0xFFFFFFFF)}; }

 private:
  VALUE _value;
};

#pragma warning(pop)

}  // namespace autd3::extra::simulator
