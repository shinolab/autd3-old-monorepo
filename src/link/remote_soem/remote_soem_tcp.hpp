﻿// File: remote_soem_tcp.hpp
// Project: remote_soem
// Created Date: 02/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#if WIN32
#include <WS2tcpip.h>
#else
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>
#endif

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6285 6385 26437 26800 26498 26451 26495)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include <spdlog/spdlog.h>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

#include <autd3/core/link.hpp>
#include <autd3/driver/cpu/ec_config.hpp>
#include <thread>

namespace autd3::link {

#if WIN32
using socklen_t = int;
#endif

class RemoteSOEMTcp final : public core::Link {
 public:
  RemoteSOEMTcp(std::string ip, const uint16_t port) : Link(), _is_open(false), _ip(std::move(ip)), _port(port) {}
  ~RemoteSOEMTcp() override = default;
  RemoteSOEMTcp(const RemoteSOEMTcp& v) noexcept = delete;
  RemoteSOEMTcp& operator=(const RemoteSOEMTcp& obj) = delete;
  RemoteSOEMTcp(RemoteSOEMTcp&& obj) = delete;
  RemoteSOEMTcp& operator=(RemoteSOEMTcp&& obj) = delete;

  void open(const core::Geometry& geometry) override {
#if WIN32
#pragma warning(push)
#pragma warning(disable : 6031)
    WSAData wsa_data{};
    WSAStartup(MAKEWORD(2, 0), &wsa_data);
#pragma warning(pop)
#endif

    _socket = socket(AF_INET, SOCK_STREAM, 0);
#if WIN32
    if (_socket == INVALID_SOCKET)
#else
    if (_socket < 0)
#endif
      throw std::runtime_error("cannot connect to simulator");

    _addr.sin_family = AF_INET;
    _addr.sin_port = htons(_port);
#if WIN32
    inet_pton(AF_INET, _ip.c_str(), &_addr.sin_addr.S_un.S_addr);
#else
    _addr.sin_addr.s_addr = inet_addr(_ip.c_str());
#endif

    if (connect(_socket, reinterpret_cast<sockaddr*>(&_addr), sizeof _addr)) throw std::runtime_error("failed to connect server");

    const auto size = geometry.num_devices() * driver::EC_INPUT_FRAME_SIZE;

    _ptr = std::make_unique<uint8_t[]>(size);
    std::memset(_ptr.get(), 0, size);

    _is_open = true;
    _th = std::thread([this, size] {
      std::vector<char> buffer(size);
      while (_is_open) {
        const auto len = recv(_socket, buffer.data(), static_cast<int>(size), 0);
        if (len <= 0) continue;
        const auto ulen = static_cast<size_t>(len);
        if (ulen % size != 0) {
          spdlog::error("Unknown data size: {}", ulen);
          continue;
        }
        const auto n = ulen / size;
        for (size_t i = 0; i < n; i++) std::memcpy(_ptr.get(), buffer.data() + i * size, ulen);
      }
    });
  }

  void close() override {
    if (!_is_open) return;
    _is_open = false;
    if (_th.joinable()) _th.join();

    driver::TxDatagram tx(0);
    tx.header().msg_id = driver::MSG_SERVER_CLOSE;
    send(tx);

#if WIN32
    closesocket(_socket);
    WSACleanup();
#else
    ::close(_socket);
#endif
  }

  bool send(const driver::TxDatagram& tx) override {
    return ::send(_socket, reinterpret_cast<const char*>(tx.data().data()), static_cast<int>(tx.effective_size()), 0) ==
           static_cast<int>(tx.effective_size());
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

#if WIN32
  SOCKET _socket{};
#else
  int _socket{0};
#endif
  sockaddr_in _addr{};
};

}  // namespace autd3::link
