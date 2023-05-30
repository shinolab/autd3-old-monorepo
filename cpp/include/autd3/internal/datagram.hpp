// File: datagram.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
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
  explicit SpecialData(void* ptr) : _ptr(ptr) {}
  SpecialData(const SpecialData& v) noexcept = default;
  SpecialData& operator=(const SpecialData& obj) = default;
  SpecialData(SpecialData&& obj) = default;
  SpecialData& operator=(SpecialData&& obj) = default;
  ~SpecialData() {
    if (_ptr != nullptr) {
      native_methods::AUTDDeleteSpecialData(_ptr);
    }
  }

  [[nodiscard]] void* ptr() const { return _ptr; }

 protected:
  void* _ptr;
};

template <typename S>
using is_special = std::is_base_of<SpecialData, std::remove_reference_t<S>>;

template <typename S>
constexpr bool is_special_v = is_special<S>::value;

class Header {
 public:
  explicit Header(void* ptr) : _ptr(ptr) {}
  Header(const Header& v) noexcept = default;
  Header& operator=(const Header& obj) = default;
  Header(Header&& obj) = default;
  Header& operator=(Header&& obj) = default;
  virtual ~Header() = default;

  [[nodiscard]] virtual void* ptr() { return _ptr; }

 protected:
  void* _ptr;
};

class NullHeader final : public Header {
 public:
  NullHeader() : Header(nullptr) {}
  ~NullHeader() override = default;
  NullHeader(const NullHeader& v) noexcept = default;
  NullHeader& operator=(const NullHeader& obj) = default;
  NullHeader(NullHeader&& obj) = default;
  NullHeader& operator=(NullHeader&& obj) = default;
};

template <typename H>
using is_header = std::is_base_of<Header, std::remove_reference_t<H>>;

template <typename H>
constexpr bool is_header_v = is_header<H>::value;

class Body {
 public:
  explicit Body(void* ptr) : _ptr(ptr) {}
  virtual ~Body() = default;
  Body(const Body& v) noexcept = default;
  Body& operator=(const Body& obj) = default;
  Body(Body&& obj) = default;
  Body& operator=(Body&& obj) = default;

  [[nodiscard]] void* ptr() const { return _ptr; }
  [[nodiscard]] virtual void* calc_ptr(const Geometry&) { return _ptr; }

 protected:
  void* _ptr;
};

class NullBody final : public Body {
 public:
  NullBody() : Body(nullptr) {}
  ~NullBody() override = default;
  NullBody(const NullBody& v) noexcept = default;
  NullBody& operator=(const NullBody& obj) = default;
  NullBody(NullBody&& obj) = default;
  NullBody& operator=(NullBody&& obj) = default;
};

template <typename B>
using is_body = std::is_base_of<Body, std::remove_reference_t<B>>;

template <typename B>
constexpr bool is_body_v = is_body<B>::value;

}  // namespace autd3::internal
