// File: defined.hpp
// Project: fpga
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/11/2022
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

namespace autd3::driver {

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26812)
#endif
/**
 * @brief Flags to control FPGA firmware
 */
class FPGAControlFlags final {
 public:
  enum VALUE : uint8_t {
    NONE = 0,
    /**
     * @brief Set when legacy mode
     */
    LEGACY_MODE = 1 << 0,
    /**
     * @brief Set when forcing fan
     */
    FORCE_FAN = 1 << 4,
    /**
     * @brief Set when STM
     */
    STM_MODE = 1 << 5,
    /**
     * @brief Set when GainSTM
     */
    STM_GAIN_MODE = 1 << 6,
    /**
     * @brief Set when returning FPGA information
     */
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

  [[nodiscard]] bool contains(const VALUE v) const noexcept { return (_value & v) == v; }

  [[nodiscard]] VALUE value() const noexcept { return _value; }

  [[nodiscard]] std::string to_string() const noexcept {
    std::vector<std::string> flags;
    if ((_value & LEGACY_MODE) == LEGACY_MODE) flags.emplace_back("LEGACY_MODE");
    if ((_value & FORCE_FAN) == FORCE_FAN) flags.emplace_back("FORCE_FAN");
    if ((_value & STM_MODE) == STM_MODE) flags.emplace_back("STM_MODE");
    if ((_value & STM_GAIN_MODE) == STM_GAIN_MODE) flags.emplace_back("STM_GAIN_MODE");
    if ((_value & READS_FPGA_INFO) == READS_FPGA_INFO) flags.emplace_back("READS_FPGA_INFO");
    if (flags.empty()) flags.emplace_back("NONE");

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

#ifdef _MSC_VER
#pragma warning(pop)
#endif

}  // namespace autd3::driver
