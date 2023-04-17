// File: datagram.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <memory>
#include <utility>

#include "autd3/core/geometry.hpp"
#include "autd3/core/link.hpp"
#include "autd3/driver/operation/null.hpp"

namespace autd3::core {

/**
 * @brief DatagramHeader is a data to be packed in the Header part of the driver::TxDatagram
 */
struct DatagramHeader {
  DatagramHeader() = default;
  virtual ~DatagramHeader() = default;
  DatagramHeader(const DatagramHeader& v) = default;
  DatagramHeader& operator=(const DatagramHeader& obj) = default;
  DatagramHeader(DatagramHeader&& obj) = default;
  DatagramHeader& operator=(DatagramHeader&& obj) = default;

  virtual std::unique_ptr<driver::Operation> operation() = 0;
};

template <typename H>
using is_header = std::is_base_of<DatagramHeader, std::remove_reference_t<H>>;

template <typename H>
constexpr bool is_header_v = is_header<H>::value;

/**
 * @brief DatagramBody is a data to be packed in the Body part of the driver::TxDatagram
 */
struct DatagramBody {
  DatagramBody() = default;
  virtual ~DatagramBody() = default;
  DatagramBody(const DatagramBody& v) = default;
  DatagramBody& operator=(const DatagramBody& obj) = default;
  DatagramBody(DatagramBody&& obj) = default;
  DatagramBody& operator=(DatagramBody&& obj) = default;

  virtual std::unique_ptr<driver::Operation> operation(const Geometry& geometry) = 0;
};

template <typename B>
using is_body = std::is_base_of<DatagramBody, std::remove_reference_t<B>>;

template <typename B>
constexpr bool is_body_v = is_body<B>::value;

/**
 * @brief Structure with DatagramHeader and DatagramBody for performing special operations
 */
class SpecialData {
 public:
  virtual ~SpecialData() = default;
  SpecialData(const SpecialData& v) noexcept = delete;
  SpecialData& operator=(const SpecialData& obj) = delete;
  SpecialData(SpecialData&& obj) = default;
  SpecialData& operator=(SpecialData&& obj) = default;

  std::unique_ptr<DatagramHeader> header() { return std::move(_h); }
  std::unique_ptr<DatagramBody> body() { return std::move(_b); }
  [[nodiscard]] std::optional<Duration> min_timeout() const noexcept { return _min_timeout; }

 protected:
  explicit SpecialData(const core::Duration min_timeout, std::unique_ptr<DatagramHeader> h, std::unique_ptr<DatagramBody> b)
      : _min_timeout(min_timeout), _h(std::move(h)), _b(std::move(b)) {}
  explicit SpecialData(std::unique_ptr<DatagramHeader> h, std::unique_ptr<DatagramBody> b)
      : _min_timeout(std::nullopt), _h(std::move(h)), _b(std::move(b)) {}

  std::optional<Duration> _min_timeout;
  std::unique_ptr<DatagramHeader> _h;
  std::unique_ptr<DatagramBody> _b;
};

template <typename S>
using is_special = std::is_base_of<SpecialData, std::remove_reference_t<S>>;

template <typename S>
constexpr bool is_special_v = is_special<S>::value;

/**
 * @brief DatagramHeader that does nothing
 */
struct NullHeader final : DatagramHeader {
  ~NullHeader() override = default;
  NullHeader() = default;
  NullHeader(const NullHeader& v) noexcept = default;
  NullHeader& operator=(const NullHeader& obj) = default;
  NullHeader(NullHeader&& obj) = default;
  NullHeader& operator=(NullHeader&& obj) = default;

  std::unique_ptr<driver::Operation> operation() override { return std::make_unique<driver::NullHeader>(); }
};

/**
 * @brief DatagramBody that does nothing
 */
struct NullBody final : DatagramBody {
  ~NullBody() override = default;
  NullBody() = default;
  NullBody(const NullBody& v) noexcept = default;
  NullBody& operator=(const NullBody& obj) = default;
  NullBody(NullBody&& obj) = default;
  NullBody& operator=(NullBody&& obj) = default;

  std::unique_ptr<driver::Operation> operation(const Geometry&) override { return std::make_unique<driver::NullBody>(); }
};

}  // namespace autd3::core
