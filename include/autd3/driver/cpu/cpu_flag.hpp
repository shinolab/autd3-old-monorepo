// File: cpu_flag.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
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
  enum Value : uint8_t {
    None = 0,
    /**
     * @brief Set when Header contains modulation data
     */
    Mod = 1 << 0,
    /**
     * @brief Set when modulation data begins
     */
    ModBegin = 1 << 1,
    /**
     * @brief Set when modulation data ends
     */
    ModEnd = 1 << 2,
    /**
     * @brief Clear when Header contains silencer data or synchronization data
     */
    ConfigEnN = 1 << 0,
    /**
     * @brief Set when Header contains silencer
     */
    ConfigSilencer = 1 << 1,
    /**
     * @brief Set when Header synchronization data
     */
    ConfigSync = 1 << 2,
    /**
     * @brief Set when Body is valid
     */
    WriteBody = 1 << 3,
    /**
     * @brief Set when Body contains STM data and STM begins
     */
    STMBegin = 1 << 4,
    /**
     * @brief Set when Body contains STM data and STM ends
     */
    STMEnd = 1 << 5,
    /**
     * @brief Set when Body is duty data (used only in Normal mode)
     */
    IsDuty = 1 << 6,
    /**
     * @brief Set when Body is modulation delay data
     */
    ModDelay = 1 << 7,
  };

  CPUControlFlags() = default;
  explicit CPUControlFlags(const Value value) noexcept : _value(value) {}

  ~CPUControlFlags() = default;
  CPUControlFlags(const CPUControlFlags& v) noexcept = default;
  CPUControlFlags& operator=(const CPUControlFlags& obj) = default;
  CPUControlFlags& operator=(const Value v) noexcept {
    _value = v;
    return *this;
  }
  CPUControlFlags(CPUControlFlags&& obj) = default;
  CPUControlFlags& operator=(CPUControlFlags&& obj) = default;

  constexpr bool operator==(const Value a) const { return _value == a; }
  constexpr bool operator!=(const Value a) const { return _value != a; }
  constexpr bool operator==(const CPUControlFlags a) const { return _value == a._value; }
  constexpr bool operator!=(const CPUControlFlags a) const { return _value != a._value; }

  void set(const Value v) noexcept { _value = static_cast<Value>(_value | v); }

  void remove(const Value v) noexcept { _value = static_cast<Value>(_value & ~v); }

  [[nodiscard]] bool contains(const Value v) const noexcept { return (_value & v) == v; }

  [[nodiscard]] Value value() const noexcept { return _value; }

  [[nodiscard]] std::string to_string() const noexcept {
    std::vector<std::string> flags;
    if ((_value & Mod) == Mod) {
      if ((_value & ModBegin) == ModBegin) flags.emplace_back("MOD_BEGIN");
      if ((_value & ModEnd) == ModEnd) flags.emplace_back("MOD_END");
    } else {
      if ((_value & ConfigSilencer) == ConfigSilencer) flags.emplace_back("CONFIG_SILENCER");
      if ((_value & ConfigSync) == ConfigSync) flags.emplace_back("CONFIG_SYNC");
    }
    if ((_value & WriteBody) == WriteBody) flags.emplace_back("WRITE_BODY");
    if ((_value & STMBegin) == STMBegin) flags.emplace_back("STM_BEGIN");
    if ((_value & STMEnd) == STMEnd) flags.emplace_back("STM_END");
    if ((_value & IsDuty) == IsDuty) flags.emplace_back("IS_DUTY");
    if ((_value & ModDelay) == ModDelay) flags.emplace_back("MOD_DELAY");
    if (flags.empty()) flags.emplace_back("NONE");

    constexpr auto delim = " | ";
    std::ostringstream os;
    std::copy(flags.begin(), flags.end(), std::ostream_iterator<std::string>(os, delim));
    std::string s = os.str();
    s.erase(s.size() - std::char_traits<char>::length(delim));
    return s;
  }

 private:
  Value _value;
};

inline std::ostream& operator<<(std::ostream& os, const CPUControlFlags& flag) { return os << flag.to_string(); }

#ifdef _MSC_VER
#pragma warning(pop)
#endif

}  // namespace autd3::driver
