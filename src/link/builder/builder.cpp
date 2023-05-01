// File: builder.cpp
// Project: builder
// Created Date: 28/04/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/builder.hpp"

#include "../../spdlog.hpp"
#include "autd3/core/link.hpp"

namespace autd3::link {

class LogImpl final : public core::Link {
 public:
  explicit LogImpl(core::LinkPtr link, std::shared_ptr<spdlog::logger> logger) : Link(), _link(std::move(link)), _logger(std::move(logger)) {}
  ~LogImpl() override = default;
  LogImpl(const LogImpl& v) noexcept = delete;
  LogImpl& operator=(const LogImpl& obj) = delete;
  LogImpl(LogImpl&& obj) = delete;
  LogImpl& operator=(LogImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    _logger->trace("Open Log link");

    if (is_open()) {
      _logger->warn("Link is already opened");
      return true;
    }

    if (!_link->open(geometry)) {
      _logger->error("Failed to open link");
      return false;
    }

    return true;
  }

  bool close() override {
    _logger->trace("Close Log link");

    if (!is_open()) {
      _logger->warn("Link is not opened");
      return false;
    }

    _synchronized = false;
    if (!_link->close()) {
      _logger->error("Failed to close link");
      return false;
    }

    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    _logger->debug("Send data");

    if (!is_open()) {
      _logger->warn("Link is closed");
      return false;
    }

    switch (tx.header().msg_id) {
      case driver::MSG_CLEAR:
      case driver::MSG_RD_CPU_VERSION_MAJOR:
      case driver::MSG_RD_FPGA_VERSION_MAJOR:
      case driver::MSG_RD_CPU_VERSION_MINOR:
      case driver::MSG_RD_FPGA_VERSION_MINOR:
      case driver::MSG_RD_FPGA_FUNCTION:
        break;
      default:
        if (!tx.header().cpu_flag.contains(driver::CPUControlFlags::ConfigEnN) && tx.header().cpu_flag.contains(driver::CPUControlFlags::ConfigSync))
          _synchronized = true;
        if (!_synchronized) _logger->warn("Devices are not synchronized!");
        break;
    }

    if (!_link->send(tx)) {
      _logger->error("Failed to send data");
      return false;
    }

    return true;
  }

  bool receive(driver::RxDatagram& rx) override {
    _logger->debug("Receive data");

    if (!is_open()) {
      _logger->warn("Link is closed");
      return false;
    }

    if (!_link->receive(rx)) {
      _logger->error("Failed to receive data");
      return false;
    }

    return true;
  }

  bool send_receive(const driver::TxDatagram& tx, driver::RxDatagram& rx, const std::optional<core::Duration> timeout) override {
    if (!send(tx)) return false;
    const auto timeout_ = timeout.value_or(_link->timeout());
    if (timeout_ == core::Duration::zero()) return receive(rx);
    if (!wait_msg_processed(tx.header().msg_id, rx, timeout_)) {
      _logger->error("Failed to confirm that the data was processed");
      return false;
    }
    return true;
  }

  bool is_open() override { return _link->is_open(); }

 private:
  bool _synchronized{false};
  core::LinkPtr _link;
  std::shared_ptr<spdlog::logger> _logger;
};

core::LinkPtr make_log_link(core::LinkPtr link, const driver::LogLevel level, std::function<void(std::string)> out, std::function<void()> flush) {
  const auto name = "AUTD3";
  std::shared_ptr<spdlog::logger> logger = spdlog::get(name);
  if (logger == nullptr) {
    spdlog::sink_ptr sink =
        out == nullptr || flush == nullptr ? get_default_sink() : std::make_shared<CustomSink<std::mutex>>(std::move(out), std::move(flush));
    logger = std::make_shared<spdlog::logger>(name, std::move(sink));
    logger->set_level(static_cast<spdlog::level::level_enum>(level));
  }
  return std::make_unique<LogImpl>(std::move(link), std::move(logger));
}

}  // namespace autd3::link
