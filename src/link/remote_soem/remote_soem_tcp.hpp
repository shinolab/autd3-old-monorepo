// File: remote_soem_tcp.hpp
// Project: remote_soem
// Created Date: 02/11/2022
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
#include <thread>
#include <utility>
#include <vector>

#include "../../spdlog.hpp"
#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/ec_config.hpp"

namespace autd3::link {

class RemoteSOEMTcp final : public core::Link {
 public:
  RemoteSOEMTcp(const core::Duration timeout, std::string ip, const uint16_t port)
      : Link(timeout), _is_open(false), _ip(std::move(ip)), _port(port), _socket(_io_service) {}
  ~RemoteSOEMTcp() override = default;
  RemoteSOEMTcp(const RemoteSOEMTcp& v) noexcept = delete;
  RemoteSOEMTcp& operator=(const RemoteSOEMTcp& obj) = delete;
  RemoteSOEMTcp(RemoteSOEMTcp&& obj) = delete;
  RemoteSOEMTcp& operator=(RemoteSOEMTcp&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    boost::system::error_code error;
    _socket.connect(boost::asio::ip::tcp::endpoint(boost::asio::ip::address::from_string(_ip), _port), error);
    if (error) throw std::runtime_error((boost::format("Cannot connect to SOEMServer: %1%") % error.message()).str());

    const auto size = geometry.num_devices() * driver::EC_INPUT_FRAME_SIZE;

    _ptr = std::make_unique<uint8_t[]>(size);
    std::memset(_ptr.get(), 0, size);

    _is_open = true;
    _th = std::thread([this, size] {
      boost::asio::streambuf receive_buffer;
      while (_is_open) {
        boost::system::error_code e;
        const auto len = read(_socket, receive_buffer, boost::asio::transfer_all(), e);
        if (e) {
          spdlog::warn("Receive failed: {}", e.message());
          continue;
        }
        const auto* buffer = boost::asio::buffer_cast<const uint8_t*>(receive_buffer.data());
        if (len % size != 0) {
          spdlog::warn("Received data size unknown: {}", len);
          continue;
        }
        const auto n = len / size;
        for (size_t i = 0; i < n; i++) std::memcpy(_ptr.get(), &buffer[i * size], len);
      }
    });

    return true;
  }

  bool close() override {
    if (!_is_open) return true;
    _is_open = false;
    if (_th.joinable()) _th.join();

    driver::TxDatagram tx({0});
    tx.header().msg_id = driver::MSG_SERVER_CLOSE;
    send(tx);

    _socket.close();

    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    boost::system::error_code error;
    const auto tx_size = write(_socket, boost::asio::buffer(tx.data()), error);
    if (!error && tx_size == tx.transmitting_size_in_bytes()) return true;
    if (error)
      spdlog::warn("Send failed: {}", error.message());
    else
      spdlog::warn("Send failed: Tx data size is {}, but {} was sent.", tx.transmitting_size_in_bytes(), tx_size);
    return false;
  }

  bool receive(driver::RxDatagram& rx) override {
    if (_ptr == nullptr) return false;
    rx.copy_from(reinterpret_cast<const driver::RxMessage*>(_ptr.get()));
    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  bool _is_open;

  std::string _ip;
  uint16_t _port;

  std::unique_ptr<uint8_t[]> _ptr;
  std::thread _th;

  boost::asio::io_service _io_service;
  boost::asio::ip::tcp::socket _socket;
};

}  // namespace autd3::link
