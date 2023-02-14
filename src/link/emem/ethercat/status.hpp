// File: status.hpp
// Project: ethercat
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cstdint>
#include <iterator>
#include <sstream>
#include <string>
#include <vector>

namespace autd3::link::ethercat {

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26812)
#endif
class EcState final {
 public:
  enum Value : uint8_t { None = 0x00, Init = 0x01, PreOp = 0x02, SafeOp = 0x04, Operational = 0x08, Ack = 0x10, Error = 0x10 };

  EcState() = default;
  EcState(const Value value) noexcept : _value(value) {}
  static EcState from(const uint16_t value) {
    const auto v = static_cast<Value>(value);
    return EcState{v};
  }
  static EcState from(const int value) {
    const auto v = static_cast<Value>(value);
    return EcState{v};
  }
  ~EcState() = default;
  EcState(const EcState& v) noexcept = default;
  EcState& operator=(const EcState& obj) = default;
  EcState& operator=(const Value v) noexcept {
    _value = v;
    return *this;
  }
  EcState(EcState&& obj) = default;
  EcState& operator=(EcState&& obj) = default;

  constexpr bool operator==(const EcState a) const { return _value == a._value; }
  constexpr bool operator!=(const EcState a) const { return _value != a._value; }
  constexpr bool operator==(const Value a) const { return _value == a; }
  constexpr bool operator!=(const Value a) const { return _value != a; }

  void set(const Value v) noexcept { _value = static_cast<Value>(_value | v); }

  void remove(const Value v) noexcept { _value = static_cast<Value>(_value & ~v); }

  [[nodiscard]] bool contains(const Value v) const noexcept { return (_value & v) == v; }

  [[nodiscard]] Value value() const noexcept { return _value; }

  [[nodiscard]] std::string to_string() const noexcept {
    std::vector<std::string> flags;
    if ((_value & Init) == Init) flags.emplace_back("INIT");
    if ((_value & PreOp) == PreOp) flags.emplace_back("PRE_OP");
    if ((_value & SafeOp) == SafeOp) flags.emplace_back("SAFE_OP");
    if ((_value & Operational) == Operational) flags.emplace_back("OPERATIONAL");
    if ((_value & Ack) == Ack) flags.emplace_back("ACK");
    if ((_value & Error) == Error) flags.emplace_back("Error");
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

inline std::ostream& operator<<(std::ostream& os, const EcState& flag) { return os << flag.to_string(); }

#ifdef _MSC_VER
#pragma warning(pop)
#endif

#pragma pack(push)
#pragma pack(1)
struct EcAlStatus {
  uint16_t al_status{};
  [[maybe_unused]] uint16_t _unused{};
  uint16_t al_status_code{};
};
#pragma pack(pop)

}  // namespace autd3::link::ethercat
