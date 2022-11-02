// File: udp_interface.hpp
// Project: SOEMAUTDServer
// Created Date: 01/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "interface.hpp"

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

namespace autd3::publish {

class TcpInterface final : public Interface {
 public:
  TcpInterface(std::string ip, const uint16_t port, const size_t dev) noexcept : _is_open(false), _dev(dev), _ip(std::move(ip)), _port(port) {}
  ~TcpInterface() override = default;
  TcpInterface(const TcpInterface& v) noexcept = delete;
  TcpInterface& operator=(const TcpInterface& obj) = delete;
  TcpInterface(TcpInterface&& obj) = delete;
  TcpInterface& operator=(TcpInterface&& obj) = delete;

  void connect() override {
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
    _addr.sin_addr.S_un.S_addr = htonl(INADDR_ANY);
#else
    _addr.sin_addr.s_addr = htonl(INADDR_ANY);
#endif

    if (const auto e = bind(_socket, reinterpret_cast<sockaddr*>(&_addr), sizeof _addr); e != 0)
      throw std::runtime_error("failed to bind socket: " + std::to_string(_port));

    listen(_socket, 1);

    const auto size = driver::HEADER_SIZE + _dev * driver::BODY_SIZE;
    _ptr = std::make_unique<uint8_t[]>(size);
    std::memset(_ptr.get(), 0, size);

    socklen_t dst_addr_size = sizeof _dst_addr;
    spdlog::info("Waiting for client connection...");
    _dst_socket = accept(_socket, reinterpret_cast<sockaddr*>(&_dst_addr), &dst_addr_size);
    spdlog::info("Connected to client");

    u_long val = 1;
#if WIN32
    ioctlsocket(_socket, FIONBIO, &val);
#else
    ioctl(_socket, FIONBIO, &val);
#endif

    _run.store(true);
    _th = std::thread([this] {
      std::vector<char> buffer(driver::HEADER_SIZE + _dev * driver::BODY_SIZE);
      while (_run.load()) {
        const auto len = recv(_dst_socket, buffer.data(), sizeof(char) * 65536, 0);
        if (len <= 0) continue;
        const auto ulen = static_cast<size_t>(len);
        if (ulen < driver::HEADER_SIZE) {
          spdlog::error("Unknown data size: {}", ulen);
          continue;
        }
        if (const auto body_len = ulen - driver::HEADER_SIZE; body_len % driver::BODY_SIZE != 0) {
          spdlog::error("Unknown data size: {}", ulen);
          continue;
        }
        std::memcpy(_ptr.get(), buffer.data(), ulen);
      }
    });

    _last_msg_id = 0;
    _is_open = true;
  }

  void close() override {
    if (!_is_open) return;
    _is_open = false;

    _run.store(false);
    if (_th.joinable()) _th.join();

#if WIN32
    closesocket(_socket);
    closesocket(_dst_socket);
    _socket = INVALID_SOCKET;
    _dst_socket = INVALID_SOCKET;
    WSACleanup();
#else
    ::close(_socket);
    _socket = -1;
    ::close(_dst_socket);
    _dst_socket = -1;
#endif
  }

  bool tx(driver::TxDatagram& tx) override {
    const auto msg_id = _ptr[0];
    if (_last_msg_id == msg_id) return false;
    _last_msg_id = msg_id;
    std::memcpy(tx.data().data(), _ptr.get(), tx.effective_size());
    return true;
  }

  void rx(driver::RxDatagram& rx) override {
#if WIN32
    if (_dst_socket == INVALID_SOCKET) return;
#else
    if (_dst_socket < 0) return;
#endif
    send(_dst_socket, reinterpret_cast<const char*>(rx.messages().data()), static_cast<int>(rx.messages().size() * driver::EC_INPUT_FRAME_SIZE), 0);
  }

 private:
  bool _is_open;
  uint8_t _last_msg_id{0};

  size_t _dev;
  std::unique_ptr<uint8_t[]> _ptr;

  std::string _ip;
  uint16_t _port;

  std::atomic<bool> _run{false};
  std::thread _th;

#if WIN32
  SOCKET _socket{};
  SOCKET _dst_socket{};
#else
  int _socket{-1};
  int _dst_socket{-1};
#endif
  sockaddr_in _addr{};
  sockaddr_in _dst_addr{};
};

}  // namespace autd3::publish
