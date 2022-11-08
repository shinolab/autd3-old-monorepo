// File: null_link.hpp
// Project: tests
// Created Date: 08/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/core/link.hpp"

namespace autd3::test {

class NullLink : public core::Link {
 public:
  core::LinkPtr build() {
    core::LinkPtr link = std::make_unique<NullLink>();
    return link;
  }

  NullLink() : Link() {}
  ~NullLink() override = default;
  NullLink(const NullLink& v) noexcept = delete;
  NullLink& operator=(const NullLink& obj) = delete;
  NullLink(NullLink&& obj) = delete;
  NullLink& operator=(NullLink&& obj) = delete;

  void open(const core::Geometry& geometry) override {}

  void close() override {}

  bool send(const driver::TxDatagram& tx) override { return true; }
  bool receive(driver::RxDatagram& rx) override { return true; }

  bool is_open() override { return true; }
};
}  // namespace autd3::test
