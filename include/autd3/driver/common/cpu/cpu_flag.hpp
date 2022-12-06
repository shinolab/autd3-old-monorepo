// File: cpu_flag.hpp
// Project: cpu
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
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

namespace autd3::driver {

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26812)
#endif
/**
 * @brief Flags to control CPU firmware
 */
class CPUControlFlags final {
 public:
  enum VALUE : uint8_t {
    NONE = 0,
    /**
     * @brief Set when Header contains modulation data
     */
    MOD = 1 << 0,
    /**
     * @brief Set when modulation data begins
     */
    MOD_BEGIN = 1 << 1,
    /**
     * @brief Set when modulation data ends
     */
    MOD_END = 1 << 2,
    /**
     * @brief Clear when Header contains silencer data or synchronization data
     */
    CONFIG_EN_N = 1 << 0,
    /**
     * @brief Set when Header contains silencer
     */
    CONFIG_SILENCER = 1 << 1,
    /**
     * @brief Set when Header synchronization data
     */
    CONFIG_SYNC = 1 << 2,
    /**
     * @brief Set when Body is valid
     */
    WRITE_BODY = 1 << 3,
    /**
     * @brief Set when Body contains STM data and STM begins
     */
    STM_BEGIN = 1 << 4,
    /**
     * @brief Set when Body contains STM data and STM ends
     */
    STM_END = 1 << 5,
    /**
     * @brief Set when Body is duty data (used only in Normal mode)
     */
    IS_DUTY = 1 << 6,
    /**
     * @brief Set when Body is modulation delay data
     */
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

  [[nodiscard]] bool contains(const VALUE v) const noexcept { return (_value & v) == v; }

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

inline std::ostream& operator<<(std::ostream& os, const CPUControlFlags& flag) { return os << flag.to_string(); }

#ifdef _MSC_VER
#pragma warning(pop)
#endif

}  // namespace autd3::driver
