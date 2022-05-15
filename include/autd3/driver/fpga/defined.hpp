// File: defined.hpp
// Project: fpga
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::driver {

constexpr size_t FPGA_CLK_FREQ = 163840000;

constexpr uint16_t MAX_CYCLE = 8191;

constexpr uint32_t MOD_SAMPLING_FREQ_DIV_MIN = 2320;
constexpr size_t MOD_BUF_SIZE_MAX = 65536;

constexpr double POINT_STM_FIXED_NUM_UNIT = 0.025;  // mm

constexpr uint32_t STM_SAMPLING_FREQ_DIV_MIN = 3224;
constexpr size_t POINT_STM_BUF_SIZE_MAX = 65536;
constexpr size_t GAIN_STM_BUF_SIZE_MAX = 1024;

constexpr uint16_t SILENCER_CYCLE_MIN = 2088;

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

  void set(const VALUE v) noexcept { _value = static_cast<VALUE>(_value | v); }

  void remove(const VALUE v) noexcept { _value = static_cast<VALUE>(_value & ~v); }

 private:
  VALUE _value;
};
#pragma warning(pop)

struct LegacyDrive {
  uint8_t phase;
  uint8_t duty;
};

struct Phase {
  uint16_t phase;
};

struct Duty {
  uint16_t duty;
};

struct FPGAInfo {
  uint8_t info;

  FPGAInfo() noexcept : info(0) {}
  explicit FPGAInfo(const uint8_t ack) noexcept : info(ack) {}

  [[nodiscard]] bool is_thermal_assert() const noexcept { return (info & 0x01) != 0; }

  [[nodiscard]] std::string to_string() const {
    std::stringstream ss;
    ss << "Thermal assert = " << std::boolalpha << is_thermal_assert();
    return ss.str();
  }
};

inline std::ostream& operator<<(std::ostream& os, const FPGAInfo& obj) {
  os << obj.to_string();
  return os;
}

}  // namespace autd3::driver
