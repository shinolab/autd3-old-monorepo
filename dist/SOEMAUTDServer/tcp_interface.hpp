// File: udp_interface.hpp
// Project: SOEMAUTDServer
// Created Date: 01/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
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

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "../../spdlog.hpp"
#include "interface.hpp"

namespace autd3::publish {

class TcpInterface final : public Interface {
 public:
  TcpInterface(std::string ip, const uint16_t port, const size_t dev) noexcept
      : _is_open(false),
        _dev(dev),
        _acc(_io_service, boost::asio::ip::tcp::endpoint(boost::asio::ip::address::from_string(ip), port)),
        _socket(_io_service) {}
  ~TcpInterface() override = default;
  TcpInterface(const TcpInterface& v) noexcept = delete;
  TcpInterface& operator=(const TcpInterface& obj) = delete;
  TcpInterface(TcpInterface&& obj) = delete;
  TcpInterface& operator=(TcpInterface&& obj) = delete;

  void connect() override {
    spdlog::info("Waiting for client connection...");
    boost::system::error_code error;
    _acc.accept(_socket, error);
    if (error) throw std::runtime_error((boost::format("Cannot connect to client: %1%") % error.message()).str());
    spdlog::info("Connected to client");

    const auto size = driver::HEADER_SIZE + _dev * AUTD3::NUM_TRANS_IN_UNIT * sizeof(uint16_t);
    _ptr = std::make_unique<uint8_t[]>(size);
    std::memset(_ptr.get(), 0, size);

    _run = true;
    _th = std::thread([this, size] {
      boost::asio::streambuf receive_buffer;
      while (_run) {
        boost::system::error_code e;
        const auto len = read(_socket, receive_buffer, boost::asio::transfer_all(), e);
        if (e) {
          spdlog::warn("Receive failed: {}", e.message());
          continue;
        }
        if (len < driver::HEADER_SIZE) {
          spdlog::warn("Received data size unknown: {}", len);
          continue;
        }
        if (const auto body_len = len - driver::HEADER_SIZE; body_len % (AUTD3::NUM_TRANS_IN_UNIT * sizeof(uint16_t)) != 0) {
          spdlog::warn("Received data size unknown: {}", len);
          continue;
        }
        const auto* buffer = boost::asio::buffer_cast<const uint8_t*>(receive_buffer.data());
        std::memcpy(_ptr.get(), buffer, len);
      }
    });

    _last_msg_id = 0;
    _is_open = true;
  }

  void close() override {
    _is_open = false;

    if (_run) {
      _run = false;
      if (_th.joinable()) _th.join();
    }

    _socket.close();
  }

  bool tx(driver::TxDatagram& tx) override {
    const auto msg_id = _ptr[0];
    if (_last_msg_id == msg_id) return false;
    _last_msg_id = msg_id;
    std::memcpy(tx.data().data(), _ptr.get(), tx.transmitting_size_in_bytes());
    return true;
  }

  bool rx(driver::RxDatagram& rx) override {
    boost::system::error_code error;
    write(_socket, boost::asio::buffer(rx.messages()), error);
    if (!error) return true;
    if (error) spdlog::warn("Send failed: {}", error.message());
    return false;
  }

 private:
  bool _is_open;
  uint8_t _last_msg_id{0};

  size_t _dev;
  std::unique_ptr<uint8_t[]> _ptr;

  bool _run{false};
  std::thread _th;

  boost::asio::io_service _io_service;
  boost::asio::ip::tcp::acceptor _acc;
  boost::asio::ip::tcp::socket _socket;
};

}  // namespace autd3::publish
