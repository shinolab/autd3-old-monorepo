// File: simulator.cpp
// Project: simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/10/2022
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
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/extra/cpu_emulator.hpp"

namespace autd3::link {

class SimulatorImpl final : public core::Link {
 public:
  explicit SimulatorImpl(const uint16_t port, std::string ip_addr) : Link(), _is_open(false), _port(port), _ip_addr(std::move(ip_addr)) {}
  ~SimulatorImpl() override { close(); }
  SimulatorImpl(const SimulatorImpl& v) noexcept = delete;
  SimulatorImpl& operator=(const SimulatorImpl& obj) = delete;
  SimulatorImpl(SimulatorImpl&& obj) = delete;
  SimulatorImpl& operator=(SimulatorImpl&& obj) = delete;

  void open(const core::Geometry& geometry) override {
    if (is_open()) return;

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
    _addr.sin_addr.s_addr = inet_addr(_ip_addr.c_str());
#endif

    _is_open = true;
    _num_devices = geometry.num_devices();
    _cpus.reserve(_num_devices);
    for (size_t i = 0; i < _num_devices; i++) {
      extra::CPU cpu(i, false);
      cpu.init();
      _cpus.emplace_back(cpu);
    }

    send(simulator_init_datagram(geometry));
  }

  void close() override {
    if (!is_open()) return;
    _is_open = false;

    send(simulator_close_datagram(_num_devices));

    _cpus.clear();

#if WIN32
    closesocket(_socket);
    WSACleanup();
#else
    ::close(_socket);
#endif
  }

  bool send(const driver::TxDatagram& tx) override {
    if (sendto(_socket, reinterpret_cast<const char*>(tx.data().data()), static_cast<int>(tx.effective_size()), 0,
               reinterpret_cast<sockaddr*>(&_addr), sizeof _addr) == -1)
      throw std::runtime_error("failed to send data");

    for (size_t i = 0; i < tx.size(); i++) _cpus[i].send(tx);

    return true;
  }

  bool receive(driver::RxDatagram& rx) override {
    for (size_t i = 0; i < _num_devices; i++) {
      rx[i].msg_id = _cpus[i].msg_id();
      rx[i].ack = _cpus[i].ack();
    }
    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  bool _is_open;
  uint16_t _port;
  std::string _ip_addr;
#if WIN32
  SOCKET _socket{};
#else
  int _socket{0};
#endif
  sockaddr_in _addr{};

  size_t _num_devices{0};

  std::vector<extra::CPU> _cpus;

  static driver::TxDatagram simulator_init_datagram(const core::Geometry& geometry) {
    driver::TxDatagram buf(geometry.num_devices());
    auto& uh = buf.header();
    uh.msg_id = driver::MSG_SIMULATOR_INIT;
    uh.fpga_flag = driver::FPGAControlFlags::NONE;
    uh.cpu_flag = driver::CPUControlFlags::NONE;
    uh.size = 0;
    for (size_t i = 0; i < geometry.num_devices(); i++) {
#ifdef AUTD3_USE_METER
      constexpr float scale = 1e3f;
#else
      constexpr float scale = 1;
#endif
      auto* const cursor = reinterpret_cast<float*>(buf.bodies()[i].data);
      auto& tr = geometry[i][0];
      auto origin = tr.position().cast<float>();
      auto rot = geometry[i].rotation().cast<float>();
      cursor[0] = origin.x() * scale;
      cursor[1] = origin.y() * scale;
      cursor[2] = origin.z() * scale;
      cursor[3] = rot.w();
      cursor[4] = rot.x();
      cursor[5] = rot.y();
      cursor[6] = rot.z();
    }
    return buf;
  }

  static driver::TxDatagram simulator_close_datagram(const size_t num_devices) {
    driver::TxDatagram buf(num_devices);
    auto& uh = buf.header();
    uh.msg_id = driver::MSG_SIMULATOR_CLOSE;
    uh.fpga_flag = driver::FPGAControlFlags::NONE;
    uh.cpu_flag = driver::CPUControlFlags::NONE;
    uh.size = 0;
    buf.num_bodies = 0;
    return buf;
  }
};
core::LinkPtr Simulator::build() const {
  core::LinkPtr link = std::make_unique<SimulatorImpl>(_port, _ip_addr);
  return link;
}

}  // namespace autd3::link
