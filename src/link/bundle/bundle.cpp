// File: bundle.cpp
// Project: bundle
// Created Date: 16/08/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/bundle.hpp"

#include "autd3/core/link.hpp"

namespace autd3::link {

class BundleImpl final : public core::Link {
 public:
  explicit BundleImpl(std::vector<core::LinkPtr> links) : Link(), _is_open(false), _links(std::move(links)) {}
  ~BundleImpl() override = default;
  BundleImpl(const BundleImpl& v) noexcept = delete;
  BundleImpl& operator=(const BundleImpl& obj) = delete;
  BundleImpl(BundleImpl&& obj) = delete;
  BundleImpl& operator=(BundleImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    if (is_open()) return true;
    _is_open = true;
    return std::all_of(_links.begin(), _links.end(), [&geometry](const auto& link) { return link->open(geometry); });
  }

  bool close() override {
    _is_open = false;
    return std::all_of(_links.begin(), _links.end(), [](const auto& link) { return link->close(); });
  }

  bool send(const driver::TxDatagram& tx) override {
    return std::accumulate(_links.begin(), _links.end(), true, [&tx](const bool acc, const auto& link) { return acc & link->send(tx); });
  }

  bool receive(driver::RxDatagram& rx) override {
    return std::accumulate(_links.rbegin(), _links.rend(), true, [&rx](const bool acc, const auto& link) { return acc & link->receive(rx); });
  }

  bool is_open() override { return _is_open; }

 private:
  bool _is_open;
  std::vector<core::LinkPtr> _links;
};

core::LinkPtr Bundle::build() {
  core::LinkPtr link = std::make_unique<BundleImpl>(std::move(_links));
  return link;
}

}  // namespace autd3::link
