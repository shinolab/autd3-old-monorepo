// File: simulator.cpp
// Project: simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/simulator.hpp"

#if WIN32
#include <WS2tcpip.h>
#else
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>
#endif

#include "autd3/core/interface.hpp"
#include "autd3/core/link.hpp"

namespace autd3::link {

class SimulatorImpl final : public core::Link {
 public:
  explicit SimulatorImpl(const uint16_t port, std::string ip_addr) : Link(), _is_open(false), _port(port), _ip_addr(std::move(ip_addr)) {}
  ~SimulatorImpl() override = default;
  SimulatorImpl(const SimulatorImpl& v) noexcept = delete;
  SimulatorImpl& operator=(const SimulatorImpl& obj) = delete;
  SimulatorImpl(SimulatorImpl&& obj) = delete;
  SimulatorImpl& operator=(SimulatorImpl&& obj) = delete;

  void open(const core::Geometry& geometry) override {
    if (is_open()) return;

    const auto geometry_datagram = init_geometry_datagram(geometry);

#if WIN32
#pragma warning(push)
#pragma warning(disable : 6031)
    WSAData wsa_data{};
    WSAStartup(MAKEWORD(2, 0), &wsa_data);
#pragma warning(pop)
#endif

    _socket = socket(AF_INET, SOCK_DGRAM, 0);
#if WIN32
    if (_socket == INVALID_SOCKET)
#else
    if (_socket < 0)
#endif
      throw std::runtime_error("cannot connect to simulator");

    _addr.sin_family = AF_INET;
    _addr.sin_port = htons(_port);
#if WIN32
    inet_pton(AF_INET, _ip_addr.c_str(), &_addr.sin_addr.S_un.S_addr);
#else
    _addr.sin_addr.s_addr = _ip_addr.c_str();
#endif

    _is_open = true;
    send(geometry_datagram);
  }

  void close() override {
    if (!is_open()) return;
#if WIN32
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
    for_each(rx.begin(), rx.end(), [this](auto& msg) { msg.msg_id = this->_last_msg_id; });

    const auto set = [&rx](const uint8_t value) { for_each(rx.begin(), rx.end(), [value](auto& msg) { msg.ack = value; }); };

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
  std::string _ip_addr;
#if WIN32
  SOCKET _socket = {};
#else
  int _socket = 0;
#endif
  sockaddr_in _addr = {};

  uint8_t _last_msg_id = 0;

  static driver::TxDatagram init_geometry_datagram(const core::Geometry& geometry) {
    driver::TxDatagram buf(geometry.num_devices());
    auto& uh = buf.header();
    uh.msg_id = driver::MSG_SIMULATOR_GEOMETRY_SET;
    uh.fpga_flag = driver::FPGAControlFlags::NONE;
    uh.cpu_flag = driver::CPUControlFlags::NONE;
    uh.size = 0;
    for (size_t i = 0; i < geometry.num_devices(); i++) {
      auto* const cursor = reinterpret_cast<float*>(buf.bodies()[i].data);
      auto& tr = geometry[i][0];
      auto origin = tr.position().cast<float>();
      auto rot = geometry[i].rotation().cast<float>();
      cursor[0] = origin.x();
      cursor[1] = origin.y();
      cursor[2] = origin.z();
      cursor[3] = rot.w();
      cursor[4] = rot.x();
      cursor[5] = rot.y();
      cursor[6] = rot.z();
    }
    return buf;
  }
};
core::LinkPtr Simulator::build() const {
  core::LinkPtr link = std::make_unique<SimulatorImpl>(_port, _ip_addr);
  return link;
}

}  // namespace autd3::link
