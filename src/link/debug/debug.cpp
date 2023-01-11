// File: debug.cpp
// Project: debug
// Created Date: 26/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/debug.hpp"

#include "autd3/core/link.hpp"
#include "autd3/link/debug_proxy.hpp"

namespace autd3::link {

class NullLink final : public core::Link {
 public:
	[[nodiscard]] core::LinkPtr build() const {
    core::LinkPtr link = std::make_unique<NullLink>();
    return link;
  }

  NullLink() : Link() {}
  ~NullLink() override = default;
  NullLink(const NullLink& v) noexcept = delete;
  NullLink& operator=(const NullLink& obj) = delete;
  NullLink(NullLink&& obj) = delete;
  NullLink& operator=(NullLink&& obj) = delete;

  bool open(const core::Geometry&) override { return true; }
  bool close() override { return true; }
  bool send(const driver::TxDatagram&) override { return true; }
  bool receive(driver::RxDatagram&) override { return true; }
  bool is_open() override { return true; }
};

class DebugImpl final : public core::Link {
 public:
  DebugImpl() : Link(), _proxy_link(DebugProxy(NullLink().build()).build()) {}
  ~DebugImpl() override = default;
  DebugImpl(const DebugImpl& v) noexcept = delete;
  DebugImpl& operator=(const DebugImpl& obj) = delete;
  DebugImpl(DebugImpl&& obj) = delete;
  DebugImpl& operator=(DebugImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override { return _proxy_link->open(geometry); }

  bool close() override { return _proxy_link->close(); }

  bool send(const driver::TxDatagram& tx) override { return _proxy_link->send(tx); }

  bool receive(driver::RxDatagram& rx) override { return _proxy_link->receive(rx); }

  bool is_open() override { return _proxy_link->is_open(); }

 private:
  core::LinkPtr _proxy_link;
};

core::LinkPtr Debug::build() const {
  core::LinkPtr link = std::make_unique<DebugImpl>();
  return link;
}

}  // namespace autd3::link
