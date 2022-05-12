// File: twincat_link.cpp
// Project: twincat
// Created Date: 08/03/2021
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2021 Hapis Lab. All rights reserved.
//

#include "autd3/link/emulator.hpp"

#if _WINDOWS
#include <WS2tcpip.h>
#else
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>
#endif

#include "autd3/core/geometry/normal_transducer.hpp"
#include "autd3/core/interface.hpp"
#include "autd3/core/link.hpp"

namespace autd3::link {

template <typename T>
class EmulatorImpl final : public core::Link {
 public:
  explicit EmulatorImpl(const uint16_t port, const core::Geometry<T>& geometry)
      : Link(), _is_open(false), _port(port), _geometry_datagram(init_geometry_datagram(geometry)) {}
  ~EmulatorImpl() override = default;
  EmulatorImpl(const EmulatorImpl& v) noexcept = delete;
  EmulatorImpl& operator=(const EmulatorImpl& obj) = delete;
  EmulatorImpl(EmulatorImpl&& obj) = delete;
  EmulatorImpl& operator=(EmulatorImpl&& obj) = delete;

  void open() override {
    if (is_open()) return;

#if _WINDOWS
#pragma warning(push)
#pragma warning(disable : 6031)
    WSAData wsa_data{};
    WSAStartup(MAKEWORD(2, 0), &wsa_data);
#pragma warning(pop)
#endif

    _socket = socket(AF_INET, SOCK_DGRAM, 0);
#if _WINDOWS
    if (_socket == INVALID_SOCKET)
#else
    if (_socket < 0)
#endif
      throw std::runtime_error("cannot connect to emulator");

    _addr.sin_family = AF_INET;
    _addr.sin_port = htons(_port);
#if _WINDOWS
    const auto ip_addr("127.0.0.1");
    inet_pton(AF_INET, ip_addr, &_addr.sin_addr.S_un.S_addr);
#else
    _addr.sin_addr.s_addr = inet_addr("127.0.0.1");
#endif

    _is_open = true;
    send(_geometry_datagram);
  }

  void close() override {
    if (!is_open()) return;
#if _WINDOWS
    closesocket(_socket);
    WSACleanup();
#else
    ::close(_socket);
#endif
    _is_open = false;
  }

  bool send(const driver::TxDatagram& tx) override {
    _last_msg_id = tx.header().msg_id;
    if (sendto(_socket, reinterpret_cast<const char*>(tx.data().data()), static_cast<int>(tx.effective_size()), 0,
               reinterpret_cast<sockaddr*>(&_addr), sizeof _addr) == -1)
      throw std::runtime_error("failed to send data");
    return true;
  }
  bool receive(driver::RxDatagram& rx) override {
    for (auto& [_, msg_id] : rx) msg_id = _last_msg_id;

    const auto set = [&rx](const uint8_t value) {
      for (auto& [ack, _] : rx) ack = value;
    };

    switch (_last_msg_id) {
      case driver::MSG_CLEAR:
        break;
      case driver::MSG_RD_CPU_VERSION:
      case driver::MSG_RD_FPGA_VERSION:
      case driver::MSG_RD_FPGA_FUNCTION:
        set(0xFF);
        break;
      default:
        break;
    }
    return true;
  }
  bool is_open() override { return _is_open; }

 private:
  bool _is_open;
  uint16_t _port;
#if _WINDOWS
  SOCKET _socket = {};
#else
  int _socket = 0;
#endif
  sockaddr_in _addr = {};

  uint8_t _last_msg_id = 0;
  driver::TxDatagram _geometry_datagram;

  static driver::TxDatagram init_geometry_datagram(const core::Geometry<T>& geometry) {
    driver::TxDatagram buf(geometry.num_devices());

    auto& uh = buf.header();
    uh.msg_id = driver::MSG_EMU_GEOMETRY_SET;
    uh.fpga_flag = driver::FPGAControlFlags::NONE;
    uh.cpu_flag = driver::CPUControlFlags::NONE;
    uh.size = 0;

    for (size_t i = 0; i < geometry.num_devices(); i++) {
      auto* const cursor = reinterpret_cast<float*>(buf.bodies()[i].data);
      auto& tr = geometry[i][0];
      auto origin = tr.position().template cast<float>();
      auto right = tr.x_direction().template cast<float>();
      auto up = tr.y_direction().template cast<float>();
      cursor[0] = origin.x();
      cursor[1] = origin.y();
      cursor[2] = origin.z();
      cursor[3] = right.x();
      cursor[4] = right.y();
      cursor[5] = right.z();
      cursor[6] = up.x();
      cursor[7] = up.y();
      cursor[8] = up.z();
    }

    return buf;
  }
};

template <>
core::LinkPtr Emulator<core::LegacyTransducer>::build() {
  core::LinkPtr link = std::make_unique<EmulatorImpl<core::LegacyTransducer>>(_port, _geometry);
  return link;
}

template <>
core::LinkPtr Emulator<core::NormalTransducer>::build() {
  core::LinkPtr link = std::make_unique<EmulatorImpl<core::NormalTransducer>>(_port, _geometry);
  return link;
}

}  // namespace autd3::link
