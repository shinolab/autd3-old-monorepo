// File: defined.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::driver {
constexpr uint8_t MSG_CLEAR = 0x00;
constexpr uint8_t MSG_RD_CPU_VERSION = 0x01;
constexpr uint8_t MSG_RD_FPGA_VERSION = 0x03;
constexpr uint8_t MSG_RD_FPGA_FUNCTION = 0x04;
constexpr uint8_t MSG_HEADER_ONLY_BEGINNING = 0x05;
constexpr uint8_t MSG_HEADER_ONLY_END = 0x7F;
constexpr uint8_t MSG_CONTAIN_BODY_BEGINNING = 0x80;
constexpr uint8_t MSG_CONTAIN_BODY_END = 0xF0;
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
    MOD_BEGIN = 1 << 0,
    MOD_END = 1 << 1,
    STM_BEGIN = 1 << 2,
    STM_END = 1 << 3,
    IS_DUTY = 1 << 4,
    CONFIG_SILENCER = 1 << 5,
    READS_FPGA_INFO = 1 << 6,
    DO_SYNC = 1 << 7,
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

  constexpr bool operator==(const CPUControlFlags a) const { return _value == a._value; }
  constexpr bool operator!=(const CPUControlFlags a) const { return _value != a._value; }

  void set(const VALUE v) noexcept { _value = static_cast<VALUE>(_value | v); }

  void remove(const VALUE v) noexcept { _value = static_cast<VALUE>(_value & ~v); }

 private:
  VALUE _value;
};
#pragma warning(pop)

}  // namespace autd3::driver
