// File: link_soem_impl.hpp
// Project: soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <functional>
#include <memory>
#include <string>
#include <utility>

#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/link/soem.hpp"
#include "soem_handler.hpp"

namespace autd3::link {

class SOEMLink final : public core::Link {
 public:
  SOEMLink(const bool high_precision, std::string ifname, const uint16_t sync0_cycle, const uint16_t send_cycle,
           std::function<void(std::string)> on_lost, const SyncMode sync_mode, const std::chrono::milliseconds state_check_interval,
           std::shared_ptr<spdlog::logger> logger)
      : Link(),
        _handler(std::make_unique<SOEMHandler>(high_precision, std::move(ifname), sync0_cycle, send_cycle, std::move(on_lost), sync_mode,
                                               state_check_interval, std::move(logger))) {}
  ~SOEMLink() override = default;
  SOEMLink(const SOEMLink& v) noexcept = delete;
  SOEMLink& operator=(const SOEMLink& obj) = delete;
  SOEMLink(SOEMLink&& obj) = default;
  SOEMLink& operator=(SOEMLink&& obj) = default;

  bool open(const core::Geometry& geometry) override;
  bool send(const driver::TxDatagram& tx) override;
  bool receive(driver::RxDatagram& rx) override;
  bool close() override;
  bool is_open() override;

 private:
  std::unique_ptr<SOEMHandler> _handler;
};

}  // namespace autd3::link
