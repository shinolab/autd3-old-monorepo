// File: remote_simulator.cpp
// Project: remote_simulator
// Created Date: 28/04/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#ifdef WIN32
#include <SDKDDKVer.h>  // for boost/asio
#endif

#include <boost/asio.hpp>
#include <boost/format.hpp>
#include <memory>
#include <string>
#include <thread>
#include <utility>
#include <vector>

#include "../../spdlog.hpp"
#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/link/remote_simulator.hpp"

namespace autd3::link {

class RemoteSimulatorImpl final : public core::Link {
 public:
  RemoteSimulatorImpl(const core::Duration timeout, std::string ip, const uint16_t port)
      : Link(timeout), _is_open(false), _ip(std::move(ip)), _port(port), _socket(_io_service) {}
  ~RemoteSimulatorImpl() override = default;
  RemoteSimulatorImpl(const RemoteSimulatorImpl& v) noexcept = delete;
  RemoteSimulatorImpl& operator=(const RemoteSimulatorImpl& obj) = delete;
  RemoteSimulatorImpl(RemoteSimulatorImpl&& obj) = delete;
  RemoteSimulatorImpl& operator=(RemoteSimulatorImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    boost::system::error_code error;
    _socket.connect(boost::asio::ip::tcp::endpoint(boost::asio::ip::address::from_string(_ip), _port), error);
    if (error) throw std::runtime_error((boost::format("Cannot connect to Simulator: %1%") % error.message()).str());

    const auto size = geometry.num_devices() * driver::EC_INPUT_FRAME_SIZE;

    _ptr = std::make_unique<uint8_t[]>(size);
    std::memset(_ptr.get(), 0, size);

    _is_open = true;
    std::thread([this, size] {
      while (_is_open) {
        uint8_t buffer[65536];
        boost::system::error_code e;
        const auto len = _socket.read_some(boost::asio::buffer(buffer), e);
        if (e == boost::asio::error::eof) {
          _is_open = false;
          break;
        }
        if (e) {
          spdlog::warn("Receive failed: {}", e.message());
          continue;
        }
        std::memcpy(_ptr.get(), buffer, size);
      }
    }).detach();

    if (!send(simulator_init_datagram(geometry))) throw std::runtime_error("Failed to init Simulator.");

    for (size_t i = 0; i < 20; i++) {
      std::this_thread::sleep_for(autd3::core::Milliseconds(100));
      if (_ptr[0] == driver::MSG_SIMULATOR_INIT) {
        _is_open = true;
        return true;
      }
    }

    throw std::runtime_error("Failed to open Simulator. Make sure Simulator is running.");
  }

  bool close() override {
    if (!is_open()) return true;

    if (!send(simulator_close_datagram())) throw std::runtime_error("Failed to close simulator.");

    _is_open = false;

    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    boost::system::error_code error;
    const auto tx_size = write(_socket, boost::asio::buffer(tx.data(), tx.transmitting_size_in_bytes()), error);
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
  static std::vector<uint8_t> simulator_init_datagram(const core::Geometry& geometry) {
    std::vector<uint8_t> data;
    const auto geometry_size = sizeof(uint8_t) + sizeof(uint32_t) + geometry.num_devices() * sizeof(float) * 7;
    data.resize(geometry_size);

    auto* cursor = data.data();
    *cursor++ = driver::MSG_SIMULATOR_INIT;
    *reinterpret_cast<uint32_t*>(cursor) = static_cast<uint32_t>(geometry.num_devices());
    cursor += sizeof(uint32_t);

    size_t i = 0;
    for (const size_t dev : geometry.device_map()) {
      auto* p = reinterpret_cast<float*>(cursor);
      auto& tr = geometry[i];
      Eigen::Vector3<float> origin = tr.position().cast<float>();
      Eigen::Quaternion<float> rot = tr.rotation().cast<float>();
      p[0] = origin.x();
      p[1] = origin.y();
      p[2] = origin.z();
      p[3] = rot.w();
      p[4] = rot.x();
      p[5] = rot.y();
      p[6] = rot.z();
      cursor += 7 * sizeof(float);
      i += dev;
    }
    return data;
  }

  static std::vector<uint8_t> simulator_close_datagram() { return {driver::MSG_SIMULATOR_CLOSE}; }

  [[nodiscard]] bool send(const std::vector<uint8_t>& raw) {
    boost::system::error_code error;
    const auto tx_size = write(_socket, boost::asio::buffer(raw.data(), raw.size()), error);
    if (!error && tx_size == raw.size()) return true;
    if (error)
      spdlog::warn("Send failed: {}", error.message());
    else
      spdlog::warn("Send failed: Tx data size is {}, but {} was sent.", raw.size(), tx_size);
    return false;
  }

  bool _is_open;

  std::string _ip;
  uint16_t _port;

  std::unique_ptr<uint8_t[]> _ptr;

  boost::asio::io_service _io_service;
  boost::asio::ip::tcp::socket _socket;
};

core::LinkPtr RemoteSimulator::build_() { return std::make_unique<RemoteSimulatorImpl>(_timeout, _ip, _port); }

}  // namespace autd3::link
