// File: interface.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/driver.hpp"
#include "geometry.hpp"
#include "mode.hpp"

namespace autd3::core {

struct DatagramHeader;
struct DatagramBody;

template <typename H, typename B>
struct DatagramPack {
  H header;
  B body;
};

struct DatagramPackRef {
  DatagramHeader& header;
  DatagramBody& body;
};

template <typename H, typename B>
auto operator,(H&& h, B&& b) -> std::enable_if_t<std::is_base_of_v<DatagramHeader, H> && std::is_base_of_v<DatagramBody, B> &&
                                                     !std::is_lvalue_reference_v<H> && !std::is_lvalue_reference_v<B>,
                                                 DatagramPack<H, B>> {
  return {std::move(h), std::move(b)};
}

template <typename H, typename B>
auto operator,(H&& h, B&& b) -> std::enable_if_t<std::is_base_of_v<DatagramHeader, std::remove_reference_t<H>> &&
                                                     std::is_base_of_v<DatagramBody, std::remove_reference_t<B>> && std::is_lvalue_reference_v<H> &&
                                                     std::is_lvalue_reference_v<B>,
                                                 DatagramPackRef> {
  return {h, b};
}

template <typename H, typename B>
auto operator,(B&& b, H&& h) -> std::enable_if_t<std::is_base_of_v<DatagramHeader, H> && std::is_base_of_v<DatagramBody, B> &&
                                                     !std::is_lvalue_reference_v<H> && !std::is_lvalue_reference_v<B>,
                                                 DatagramPack<H, B>> {
  return {std::move(h), std::move(b)};
}

template <typename H, typename B>
auto operator,(B&& b, H&& h) -> std::enable_if_t<std::is_base_of_v<DatagramHeader, std::remove_reference_t<H>> &&
                                                     std::is_base_of_v<DatagramBody, std::remove_reference_t<B>> && std::is_lvalue_reference_v<H> &&
                                                     std::is_lvalue_reference_v<B>,
                                                 DatagramPackRef> {
  return {h, b};
}

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

  virtual bool init() = 0;
  virtual bool pack(const std::unique_ptr<const driver::Driver>& driver, uint8_t msg_id, driver::TxDatagram& tx) = 0;
  [[nodiscard]] virtual bool is_finished() const = 0;
};

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

  virtual void init() = 0;
  virtual void pack(const std::unique_ptr<const driver::Driver>& driver, const std::unique_ptr<const core::Mode>& mode, const Geometry& geometry,
                    driver::TxDatagram& tx) = 0;
  [[nodiscard]] virtual bool is_finished() const = 0;
};

struct NullHeader final : DatagramHeader {
  ~NullHeader() override = default;

  bool init() override { return true; }
  bool pack(const std::unique_ptr<const driver::Driver>& driver, uint8_t msg_id, driver::TxDatagram& tx) override {
    driver->null_header(msg_id, tx);
    return true;
  }

  bool is_finished() const override { return true; }
};

struct NullBody final : DatagramBody {
  ~NullBody() override = default;

  void init() override {}

  void pack(const std::unique_ptr<const driver::Driver>& driver, const std::unique_ptr<const core::Mode>&, const Geometry&,
            driver::TxDatagram& tx) override {
    driver->null_body(tx);
  }

  bool is_finished() const override { return true; }
};

}  // namespace autd3::core
