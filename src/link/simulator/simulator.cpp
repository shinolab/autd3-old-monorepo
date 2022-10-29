// File: simulator.cpp
// Project: simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/simulator.hpp"

#include <smem/smem.hpp>
#include <thread>

#include "autd3/core/interface.hpp"
#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/ec_config.hpp"

namespace autd3::link {

class SimulatorImpl final : public core::Link {
 public:
  SimulatorImpl() noexcept : Link() {}
  ~SimulatorImpl() override { close(); }
  SimulatorImpl(const SimulatorImpl& v) noexcept = delete;
  SimulatorImpl& operator=(const SimulatorImpl& obj) = delete;
  SimulatorImpl(SimulatorImpl&& obj) = delete;
  SimulatorImpl& operator=(SimulatorImpl&& obj) = delete;

  void open(const core::Geometry& geometry) override {
    if (is_open()) return;

    const auto size = driver::HEADER_SIZE + geometry.num_devices() * (driver::BODY_SIZE + driver::EC_INPUT_FRAME_SIZE);
    _smem.create("autd3_simulator_smem", size);
    _ptr = static_cast<uint8_t*>(_smem.map());

    _num_devices = geometry.num_devices();

    send(simulator_init_datagram(geometry));
    while (_ptr[0] == driver::MSG_SIMULATOR_INIT) std::this_thread::sleep_for(std::chrono::milliseconds(100));
  }

  void close() override {
    if (!is_open()) return;
    send(simulator_close_datagram(_num_devices));

    _smem.unmap();
    _ptr = nullptr;
  }

  bool send(const driver::TxDatagram& tx) override {
    if (_ptr == nullptr) return false;
    std::memcpy(_ptr, tx.data().data(), tx.effective_size());
    return true;
  }

  bool receive(driver::RxDatagram& rx) override {
    if (_ptr == nullptr) return false;
    rx.copy_from(reinterpret_cast<const driver::RxMessage*>(_ptr + driver::HEADER_SIZE + _num_devices * driver::BODY_SIZE));
    return true;
  }

  bool is_open() override { return _ptr != nullptr; }

 private:
  size_t _num_devices{0};

  smem::SMem _smem;
  uint8_t* _ptr{nullptr};

  static driver::TxDatagram simulator_init_datagram(const core::Geometry& geometry) {
    driver::TxDatagram buf(geometry.num_devices());
    auto& uh = buf.header();
    uh.msg_id = driver::MSG_SIMULATOR_INIT;
    uh.fpga_flag = driver::FPGAControlFlags::NONE;
    uh.cpu_flag = driver::CPUControlFlags::NONE;
    uh.size = static_cast<uint8_t>(geometry.num_devices());
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
#ifdef AUTD3_USE_LEFT_HANDED
      cursor[0] = origin.x() * scale;
      cursor[1] = origin.y() * scale;
      cursor[2] = -origin.z() * scale;
      cursor[3] = rot.w();
      cursor[4] = -rot.x();
      cursor[5] = -rot.y();
      cursor[6] = rot.z();
#else
      cursor[0] = origin.x() * scale;
      cursor[1] = origin.y() * scale;
      cursor[2] = origin.z() * scale;
      cursor[3] = rot.w();
      cursor[4] = rot.x();
      cursor[5] = rot.y();
      cursor[6] = rot.z();
#endif
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
  core::LinkPtr link = std::make_unique<SimulatorImpl>();
  return link;
}

}  // namespace autd3::link
