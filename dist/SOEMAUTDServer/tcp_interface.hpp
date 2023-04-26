// File: udp_interface.hpp
// Project: SOEMAUTDServer
// Created Date: 01/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#ifdef WIN32
#include <SDKDDKVer.h>
#endif

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 4312)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include <boost/asio.hpp>
#include <boost/format.hpp>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

#include <atomic>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "../../spdlog.hpp"
#include "interface.hpp"

namespace autd3::publish {

class TcpInterface final : public Interface {
 public:
  TcpInterface(std::string ip, const uint16_t port, const size_t dev) noexcept : _is_open(false), _dev(dev), _port(port) {}
  ~TcpInterface() override = default;
  TcpInterface(const TcpInterface& v) noexcept = delete;
  TcpInterface& operator=(const TcpInterface& obj) = delete;
  TcpInterface(TcpInterface&& obj) = delete;
  TcpInterface& operator=(TcpInterface&& obj) = delete;

  void session() {
    spdlog::info("Connected to client: {}", _sock->remote_endpoint().address().to_string());
    try {
      for (;;) {
        uint8_t buffer[65536];
        boost::system::error_code error;
        size_t len = _sock->read_some(boost::asio::buffer(buffer), error);
        if (error == boost::asio::error::eof || error == boost::asio::error::connection_reset)
          break;
        else if (error)
          throw boost::system::system_error(error);
        if (len < driver::HEADER_SIZE) {
          spdlog::warn("Received data size unknown: {}", len);
          continue;
        }
        if (const auto body_len = len - driver::HEADER_SIZE; body_len % (AUTD3::NUM_TRANS_IN_UNIT * sizeof(uint16_t)) != 0) {
          spdlog::warn("Received data size unknown: {}", len);
          continue;
        }
        std::memcpy(_ptr.get(), buffer, len);
      }
    } catch (std::exception& e) {
      spdlog::error("Exception in thread: {}", e.what());
    }

    spdlog::info("Disconnected from client");
  }

  void connect() override {
    const auto size = driver::HEADER_SIZE + _dev * AUTD3::NUM_TRANS_IN_UNIT * sizeof(uint16_t);
    _ptr = std::make_unique<uint8_t[]>(size);
    std::memset(_ptr.get(), 0, size);

    std::thread([this]() {
      boost::asio::io_context io_context;
      boost::asio::ip::tcp::acceptor a(io_context, boost::asio::ip::tcp::endpoint(boost::asio::ip::tcp::v4(), _port));
      for (;;) {
        _is_open.store(false);
        spdlog::info("Waiting for client connection...");
        _sock = std::make_shared<boost::asio::ip::tcp::socket>(a.accept());
        _is_open.store(true);
        session();
      }
    }).detach();

    while (!is_open()) std::this_thread::sleep_for(std::chrono::milliseconds(100));

    _last_msg_id = 0;
  }

  void close() override {}

  bool tx(driver::TxDatagram& tx) override {
    if (!is_open()) return false;
    const auto msg_id = _ptr[0];
    if (_last_msg_id == msg_id) return false;
    _last_msg_id = msg_id;
    std::memcpy(tx.data().data(), _ptr.get(), tx.transmitting_size_in_bytes());
    return true;
  }

  bool rx(driver::RxDatagram& rx) override {
    if (!is_open()) return false;
    try {
      _sock->send(boost::asio::buffer(rx.messages(), rx.messages().size() * sizeof(driver::RxMessage)));
    } catch (const std::exception&) {
      return false;
    }
    return true;
  }

 private:
  [[nodiscard]] bool is_open() const { return _is_open.load(); }

  uint16_t _port;

  std::atomic<bool> _is_open;
  uint8_t _last_msg_id{0};

  size_t _dev;
  std::unique_ptr<uint8_t[]> _ptr;

  std::shared_ptr<boost::asio::ip::tcp::socket> _sock{nullptr};
};

}  // namespace autd3::publish
