// File: datagram.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <type_traits>

#include "autd3/internal/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class SpecialData {
 public:
  SpecialData() = default;
  SpecialData(const SpecialData& obj) = default;
  SpecialData& operator=(const SpecialData& obj) = default;
  SpecialData(SpecialData&& obj) = default;
  SpecialData& operator=(SpecialData&& obj) = default;
  virtual ~SpecialData() = default;

  [[nodiscard]] virtual native_methods::DatagramSpecialPtr ptr() const = 0;
};

template <typename S>
using is_special = std::is_base_of<SpecialData, std::remove_reference_t<S>>;

template <typename S>
constexpr bool is_special_v = is_special<S>::value;

class Header {
 public:
  Header() = default;
  Header(const Header& v) noexcept = default;
  Header& operator=(const Header& obj) = default;
  Header(Header&& obj) = default;
  Header& operator=(Header&& obj) = default;
  virtual ~Header() = default;

  [[nodiscard]] virtual native_methods::DatagramHeaderPtr ptr() const = 0;
};

class NullHeader final : public Header {
 public:
  NullHeader() = default;
  ~NullHeader() override = default;
  NullHeader(const NullHeader& v) noexcept = default;
  NullHeader& operator=(const NullHeader& obj) = default;
  NullHeader(NullHeader&& obj) = default;
  NullHeader& operator=(NullHeader&& obj) = default;

  [[nodiscard]] native_methods::DatagramHeaderPtr ptr() const override { return native_methods::DatagramHeaderPtr{nullptr}; }
};

template <typename H>
using is_header = std::is_base_of<Header, std::remove_reference_t<H>>;

template <typename H>
constexpr bool is_header_v = is_header<H>::value;

class Body {
 public:
  Body() = default;
  virtual ~Body() = default;
  Body(const Body& v) noexcept = default;
  Body& operator=(const Body& obj) = default;
  Body(Body&& obj) = default;
  Body& operator=(Body&& obj) = default;

  [[nodiscard]] virtual native_methods::DatagramBodyPtr ptr(const Geometry&) const = 0;
};

class NullBody final : public Body {
 public:
  NullBody() = default;
  ~NullBody() override = default;
  NullBody(const NullBody& v) noexcept = default;
  NullBody& operator=(const NullBody& obj) = default;
  NullBody(NullBody&& obj) = default;
  NullBody& operator=(NullBody&& obj) = default;

  [[nodiscard]] native_methods::DatagramBodyPtr ptr(const Geometry&) const override { return native_methods::DatagramBodyPtr{nullptr}; }
};

template <typename B>
using is_body = std::is_base_of<Body, std::remove_reference_t<B>>;

template <typename B>
constexpr bool is_body_v = is_body<B>::value;

}  // namespace autd3::internal
