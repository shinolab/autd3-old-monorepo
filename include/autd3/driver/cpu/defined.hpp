// File: defined.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cstdint>
#include <iterator>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

namespace autd3::driver {
constexpr uint8_t MSG_CLEAR = 0x00;
constexpr uint8_t MSG_RD_CPU_VERSION = 0x01;
constexpr uint8_t MSG_RD_FPGA_VERSION = 0x03;
constexpr uint8_t MSG_RD_FPGA_FUNCTION = 0x04;
constexpr uint8_t MSG_BEGIN = 0x05;
constexpr uint8_t MSG_END = 0xF0;
constexpr uint8_t MSG_SIMULATOR_GEOMETRY_SET = 0xFF;

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
    MOD_DELAY = 1 << 7,
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

  bool contains(const VALUE v) const noexcept { return (_value & v) == v; }

  [[nodiscard]] VALUE value() const noexcept { return _value; }

  [[nodiscard]] std::string to_string() const noexcept {
    std::vector<std::string> flags;
    if ((_value & MOD) == MOD) {
      if ((_value & MOD_BEGIN) == MOD_BEGIN) flags.emplace_back("MOD_BEGIN");
      if ((_value & MOD_END) == MOD_END) flags.emplace_back("MOD_END");
    } else {
      if ((_value & CONFIG_SILENCER) == CONFIG_SILENCER) flags.emplace_back("CONFIG_SILENCER");
      if ((_value & CONFIG_SYNC) == CONFIG_SYNC) flags.emplace_back("CONFIG_SYNC");
    }
    if ((_value & WRITE_BODY) == WRITE_BODY) flags.emplace_back("WRITE_BODY");
    if ((_value & STM_BEGIN) == STM_BEGIN) flags.emplace_back("STM_BEGIN");
    if ((_value & STM_END) == STM_END) flags.emplace_back("STM_END");
    if ((_value & IS_DUTY) == IS_DUTY) flags.emplace_back("IS_DUTY");
    if ((_value & MOD_DELAY) == MOD_DELAY) flags.emplace_back("MOD_DELAY");
    if (flags.size() == 0) flags.emplace_back("NONE");

    constexpr auto delim = " | ";
    std::ostringstream os;
    std::copy(flags.begin(), flags.end(), std::ostream_iterator<std::string>(os, delim));
    std::string s = os.str();
    s.erase(s.size() - std::char_traits<char>::length(delim));
    return s;
  }

 private:
  VALUE _value;
};

inline std::ostream& operator<<(std::ostream& os, const CPUControlFlags& flag) { return os << flag.to_string(); }

#pragma warning(pop)

enum class GainSTMMode : uint16_t {
  PhaseDutyFull = 0x0001,
  PhaseFull = 0x0002,
  PhaseHalf = 0x0004,
};

}  // namespace autd3::driver
