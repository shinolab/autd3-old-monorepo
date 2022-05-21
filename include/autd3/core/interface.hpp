// File: interface.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/driver/cpu/datagram.hpp"
#include "geometry/geometry.hpp"
#include "geometry/transducer.hpp"

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

  virtual void init() = 0;
  virtual void pack(uint8_t msg_id, driver::TxDatagram& tx) = 0;
  [[nodiscard]] virtual bool is_finished() const = 0;
};

/**
 * @brief DatagramBody is a data to be packed in the Body part of the driver::TxDatagram
 */
template <typename T, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct DatagramBody {
  DatagramBody() = default;
  virtual ~DatagramBody() = default;
  DatagramBody(const DatagramBody& v) = default;
  DatagramBody& operator=(const DatagramBody& obj) = default;
  DatagramBody(DatagramBody&& obj) = default;
  DatagramBody& operator=(DatagramBody&& obj) = default;

  virtual void init() = 0;
  virtual void pack(const Geometry<T>& geometry, driver::TxDatagram& tx) = 0;
  [[nodiscard]] virtual bool is_finished() const = 0;
};

struct NullHeader final : DatagramHeader {
  ~NullHeader() override = default;

  void init() override {}
  void pack(uint8_t msg_id, driver::TxDatagram& tx) override { driver::null_header(msg_id, tx); }

  bool is_finished() const override { return true; }
};

template <typename T, std::enable_if_t<std::is_base_of_v<Transducer<typename T::D>, T>, nullptr_t> = nullptr>
struct NullBody final : DatagramBody<T> {
  ~NullBody() override = default;

  void init() override {}

  void pack(const Geometry<T>&, driver::TxDatagram& tx) { driver::null_body(tx); }

  bool is_finished() const override { return true; }
};

}  // namespace autd3::core
