// File: bitflags.hpp
// Project: driver
// Created Date: 17/03/2023
// Author: Shun Suzuki
// -----
// Last Modified: 17/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <type_traits>

namespace autd3::driver {

template <typename E>
struct is_scoped_enum : std::integral_constant<bool, std::is_enum_v<E> && !std::is_convertible_v<E, int>> {};

template <typename E>
inline constexpr bool is_scoped_enum_v = is_scoped_enum<E>::value;

template <typename T, std::enable_if_t<is_scoped_enum_v<T>, std::nullptr_t> = nullptr>
class BitFlags {
 public:
  typedef std::underlying_type_t<T> value_type;

  constexpr BitFlags() : _value() {}
  constexpr BitFlags(T value) : _value(static_cast<value_type>(value)) {}
  constexpr BitFlags(const BitFlags& value) = default;
  constexpr BitFlags& operator=(const BitFlags& obj) = default;
  constexpr BitFlags(BitFlags&& obj) = default;
  constexpr BitFlags& operator=(BitFlags&& obj) = default;
  ~BitFlags() = default;

  constexpr bool operator==(const BitFlags a) const { return _value == a._value; }
  constexpr bool operator!=(const BitFlags a) const { return _value != a._value; }
  constexpr bool operator==(const T a) const { return _value == static_cast<value_type>(a); }
  constexpr bool operator!=(const T a) const { return _value != static_cast<value_type>(a); }

  [[nodiscard]] constexpr value_type value() const noexcept { return _value; }

  [[nodiscard]] constexpr bool contains(const T value) const noexcept {
    auto v = static_cast<value_type>(value);
    return (_value & v) == v;
  }

  void set(const T v) noexcept { _value = static_cast<value_type>(_value | static_cast<value_type>(v)); }

  void remove(const T v) noexcept { _value = static_cast<value_type>(_value & ~static_cast<value_type>(v)); }

  constexpr BitFlags& operator|=(BitFlags value) {
    _value |= static_cast<value_type>(value._value);
    return *this;
  }

 private:
  value_type _value;
};

template <typename T>
constexpr BitFlags<T> operator|(BitFlags<T> lhs, BitFlags<T> rhs) {
  return BitFlags<T>(lhs.value | rhs.value);
}

template <typename T>
constexpr BitFlags<T> operator|(T lhs, T rhs) {
  using value_type = typename BitFlags<T>::value_type;
  return static_cast<T>(static_cast<value_type>(lhs) | static_cast<value_type>(rhs));
}

}  // namespace autd3::driver
