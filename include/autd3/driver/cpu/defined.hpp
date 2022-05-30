// File: defined.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::driver {
constexpr uint8_t MSG_CLEAR = 0x00;
constexpr uint8_t MSG_RD_CPU_VERSION = 0x01;
constexpr uint8_t MSG_RD_FPGA_VERSION = 0x03;
constexpr uint8_t MSG_RD_FPGA_FUNCTION = 0x04;
constexpr uint8_t MSG_BEGIN = 0x05;
constexpr uint8_t MSG_END = 0xF0;
constexpr uint8_t MSG_EMU_GEOMETRY_SET = 0xFF;

constexpr size_t MOD_HEAD_DATA_SIZE = 120;
constexpr size_t MOD_BODY_DATA_SIZE = 124;

constexpr size_t POINT_STM_HEAD_DATA_SIZE = 61;
constexpr size_t POINT_STM_BODY_DATA_SIZE = 62;

#pragma warning(push)
#pragma warning(disable : 26812)
class CPUControlFlags final {
 public:
  enum VALUE : uint8_t {
    NONE = 0,
    MOD = 1 << 0,
    MOD_BEGIN = 1 << 1,
    MOD_END = 1 << 2,
    CONFIG_EN_N = 1 << 0,
    CONFIG_SILENCER = 1 << 1,
    CONFIG_SYNC = 1 << 2,
    WRITE_BODY = 1 << 3,
    STM_BEGIN = 1 << 4,
    STM_END = 1 << 5,
    IS_DUTY = 1 << 6,
    READS_FPGA_INFO = 1 << 7,
  };

  CPUControlFlags() = default;
  explicit CPUControlFlags(const VALUE value) noexcept : _value(value) {}

  ~CPUControlFlags() = default;
  CPUControlFlags(const CPUControlFlags& v) noexcept = default;
  CPUControlFlags& operator=(const CPUControlFlags& obj) = default;
  CPUControlFlags& operator=(const VALUE v) noexcept {
    _value = v;
    return *this;
  }
  CPUControlFlags(CPUControlFlags&& obj) = default;
  CPUControlFlags& operator=(CPUControlFlags&& obj) = default;

  constexpr bool operator==(const VALUE a) const { return _value == a; }
  constexpr bool operator!=(const VALUE a) const { return _value != a; }
  constexpr bool operator==(const CPUControlFlags a) const { return _value == a._value; }
  constexpr bool operator!=(const CPUControlFlags a) const { return _value != a._value; }

  void set(const VALUE v) noexcept { _value = static_cast<VALUE>(_value | v); }

  void remove(const VALUE v) noexcept { _value = static_cast<VALUE>(_value & ~v); }

  [[nodiscard]] VALUE value() const noexcept { return _value; }

 private:
  VALUE _value;
};
#pragma warning(pop)

}  // namespace autd3::driver
