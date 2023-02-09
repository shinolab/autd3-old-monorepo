// File: utils.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::link {

constexpr uint16_t u16_from_le(const uint16_t v) { return v; }

constexpr uint16_t u16_from_le_bytes(const uint8_t b0, const uint8_t b1) { return static_cast<uint16_t>(b1 << 8 | b0); }

inline int64_t i64_from_le_bytes(const uint8_t b[sizeof(int64_t)]) {
  int64_t i;
  std::memcpy(&i, b, sizeof(int64_t));
  return i;
}

inline int32_t i32_from_le_bytes(const uint8_t b[sizeof(int32_t)]) {
  int32_t i;
  std::memcpy(&i, b, sizeof(int32_t));
  return i;
}

template <typename T>
constexpr T to_le_bytes(const T v) {
  return v;
}

constexpr uint16_t to_be(const uint16_t v) {
  const uint8_t b0 = v & 0xFF;
  const uint8_t b1 = v >> 8 & 0xFF;
  return u16_from_le_bytes(b1, b0);
}

}  // namespace autd3::link
