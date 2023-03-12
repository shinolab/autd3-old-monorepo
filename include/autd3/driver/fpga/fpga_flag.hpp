// File: defined.hpp
// Project: fpga
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
  enum Value : uint8_t {
    None = 0,
    /**
     * @brief Set when legacy mode
     */
    LegacyMode = 1 << 0,
    /**
     * @brief Set when using STM finish idx
     */
    UseSTMFinishIdx = 1 << 2,
    /**
     * @brief Set when using STM start idx
     */
    UseSTMStartIdx = 1 << 3,
    /**
     * @brief Set when forcing fan
     */
    ForceFan = 1 << 4,
    /**
     * @brief Set when STM
     */
    STMMode = 1 << 5,
    /**
     * @brief Set when GainSTM
     */
    STMGainMode = 1 << 6,
    /**
     * @brief Set when returning FPGA information
     */
    ReadsFPGAInfo = 1 << 7,
  };

  FPGAControlFlags() = default;
  explicit FPGAControlFlags(const Value value) noexcept : _value(value) {}

  ~FPGAControlFlags() = default;
  FPGAControlFlags(const FPGAControlFlags& v) noexcept = default;
  FPGAControlFlags& operator=(const FPGAControlFlags& obj) = default;
  FPGAControlFlags& operator=(const Value v) noexcept {
    _value = v;
    return *this;
  }
  FPGAControlFlags(FPGAControlFlags&& obj) = default;
  FPGAControlFlags& operator=(FPGAControlFlags&& obj) = default;

  constexpr bool operator==(const FPGAControlFlags a) const { return _value == a._value; }
  constexpr bool operator!=(const FPGAControlFlags a) const { return _value != a._value; }
  constexpr bool operator==(const Value a) const { return _value == a; }
  constexpr bool operator!=(const Value a) const { return _value != a; }

  void set(const Value v) noexcept { _value = static_cast<Value>(_value | v); }

  void remove(const Value v) noexcept { _value = static_cast<Value>(_value & ~v); }

  [[nodiscard]] bool contains(const Value v) const noexcept { return (_value & v) == v; }

  [[nodiscard]] Value value() const noexcept { return _value; }

  [[nodiscard]] std::string to_string() const noexcept {
    std::vector<std::string> flags;
    if (contains(LegacyMode)) flags.emplace_back("LegacyMode");
    if (contains(UseSTMStartIdx)) flags.emplace_back("UseSTMStartIdx");
    if (contains(UseSTMFinishIdx)) flags.emplace_back("UseSTMFinishIdx");
    if (contains(ForceFan)) flags.emplace_back("ForceFan");
    if (contains(STMMode)) flags.emplace_back("STMMode");
    if (contains(STMGainMode)) flags.emplace_back("STMGainMode");
    if (contains(ReadsFPGAInfo)) flags.emplace_back("ReadsFPGAInfo");
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

inline std::ostream& operator<<(std::ostream& os, const FPGAControlFlags& flag) { return os << flag.to_string(); }

#ifdef _MSC_VER
#pragma warning(pop)
#endif

}  // namespace autd3::driver
