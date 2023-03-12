// File: cpu_flag.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/03/2023
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
     * @brief Set when Body is duty data (used only in Advanced mode)
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
    if (contains(Mod)) {
      if (contains(ModBegin)) flags.emplace_back("ModBegin");
      if (contains(ModEnd)) flags.emplace_back("ModEnd");
    } else {
      if (contains(ConfigSilencer)) flags.emplace_back("ConfigSilencer");
      if (contains(ConfigSync)) flags.emplace_back("ConfigSync");
    }
    if (contains(WriteBody)) flags.emplace_back("WriteBody");
    if (contains(STMBegin)) flags.emplace_back("STMBegin");
    if (contains(STMEnd)) flags.emplace_back("STMEnd");
    if (contains(IsDuty)) flags.emplace_back("IsDuty");
    if (contains(ModDelay)) flags.emplace_back("ModDelay");
    if (flags.empty()) flags.emplace_back("None");

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
