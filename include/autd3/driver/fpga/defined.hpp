// File: defined.hpp
// Project: fpga
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cstdint>
#include <iterator>
#include <sstream>
#include <string>
#include <vector>

#include "autd3/driver/hardware.hpp"
#include "autd3/driver/utils.hpp"

namespace autd3::driver {

constexpr size_t FPGA_CLK_FREQ = 163840000;

constexpr uint16_t MAX_CYCLE = 8191;

constexpr uint32_t MOD_SAMPLING_FREQ_DIV_MIN = 1160;
constexpr size_t MOD_BUF_SIZE_MAX = 65536;

constexpr double POINT_STM_FIXED_NUM_UNIT = 0.025;  // mm

constexpr uint32_t POINT_STM_SAMPLING_FREQ_DIV_MIN = 1612;
constexpr uint32_t GAIN_STM_SAMPLING_FREQ_DIV_MIN = 276;
constexpr uint32_t GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN = 152;
constexpr size_t POINT_STM_BUF_SIZE_MAX = 65536;
constexpr size_t GAIN_STM_BUF_SIZE_MAX = 1024;
constexpr size_t GAIN_STM_LEGACY_BUF_SIZE_MAX = 2048;

constexpr uint16_t SILENCER_CYCLE_MIN = 1044;

#pragma warning(push)
#pragma warning(disable : 26812)
class FPGAControlFlags final {
 public:
  enum VALUE : uint8_t {
    NONE = 0,
    LEGACY_MODE = 1 << 0,
    FORCE_FAN = 1 << 4,
    STM_MODE = 1 << 5,
    STM_GAIN_MODE = 1 << 6,
    READS_FPGA_INFO = 1 << 7,
  };

  FPGAControlFlags() = default;
  explicit FPGAControlFlags(const VALUE value) noexcept : _value(value) {}

  ~FPGAControlFlags() = default;
  FPGAControlFlags(const FPGAControlFlags& v) noexcept = default;
  FPGAControlFlags& operator=(const FPGAControlFlags& obj) = default;
  FPGAControlFlags& operator=(const VALUE v) noexcept {
    _value = v;
    return *this;
  }
  FPGAControlFlags(FPGAControlFlags&& obj) = default;
  FPGAControlFlags& operator=(FPGAControlFlags&& obj) = default;

  constexpr bool operator==(const FPGAControlFlags a) const { return _value == a._value; }
  constexpr bool operator!=(const FPGAControlFlags a) const { return _value != a._value; }
  constexpr bool operator==(const VALUE a) const { return _value == a; }
  constexpr bool operator!=(const VALUE a) const { return _value != a; }

  void set(const VALUE v) noexcept { _value = static_cast<VALUE>(_value | v); }

  void remove(const VALUE v) noexcept { _value = static_cast<VALUE>(_value & ~v); }

  bool contains(const VALUE v) const noexcept { return (_value & v) == v; }

  [[nodiscard]] VALUE value() const noexcept { return _value; }

  [[nodiscard]] std::string to_string() const noexcept {
    std::vector<std::string> flags;
    if ((_value & LEGACY_MODE) == LEGACY_MODE) flags.emplace_back("LEGACY_MODE");
    if ((_value & FORCE_FAN) == FORCE_FAN) flags.emplace_back("FORCE_FAN");
    if ((_value & STM_MODE) == STM_MODE) flags.emplace_back("STM_MODE");
    if ((_value & STM_GAIN_MODE) == STM_GAIN_MODE) flags.emplace_back("STM_GAIN_MODE");
    if ((_value & READS_FPGA_INFO) == READS_FPGA_INFO) flags.emplace_back("READS_FPGA_INFO");
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

inline std::ostream& operator<<(std::ostream& os, const FPGAControlFlags& flag) { return os << flag.to_string(); }

#pragma warning(pop)

struct Drive {
  double phase;
  double amp;
  uint16_t cycle;
};

struct LegacyDrive {
  uint8_t phase;
  uint8_t duty;

  static uint8_t to_phase(const Drive d) { return static_cast<uint8_t>(static_cast<int32_t>(std::round(d.phase * 256.0)) & 0xFF); }
  static uint8_t to_duty(const Drive d) { return std::round(510.0 * std::asin(std::clamp(d.amp, 0.0, 1.0)) / autd3::driver::pi); }

  void set(const Drive d) {
    phase = to_phase(d);
    duty = to_duty(d);
  }
};

struct Phase {
  uint16_t phase;

  static uint16_t to_phase(const Drive d) {
    return static_cast<uint16_t>(
        autd3::driver::rem_euclid(static_cast<int32_t>(std::round(d.phase * static_cast<double>(d.cycle))), static_cast<int32_t>(d.cycle)));
  }

  void set(const Drive d) { phase = to_phase(d); }
};

struct Duty {
  uint16_t duty;

  static uint16_t to_duty(const Drive d) {
    return static_cast<uint16_t>(std::round(static_cast<double>(d.cycle) * std::asin(std::clamp(d.amp, 0.0, 1.0)) / driver::pi));
  }

  void set(const Drive d) { duty = to_duty(d); }
};

struct FPGAInfo {
  uint8_t info;

  FPGAInfo() noexcept : info(0) {}
  explicit FPGAInfo(const uint8_t ack) noexcept : info(ack) {}

  [[nodiscard]] bool is_thermal_assert() const noexcept { return (info & 0x01) != 0; }

  [[nodiscard]] std::string to_string() const { return "Thermal assert = " + std::to_string(is_thermal_assert()); }
};

inline std::ostream& operator<<(std::ostream& os, const FPGAInfo& obj) {
  os << obj.to_string();
  return os;
}

}  // namespace autd3::driver
