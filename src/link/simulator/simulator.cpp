// File: simulator.cpp
// Project: simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/simulator.hpp"

#include <smem/smem.hpp>
#include <thread>

#include "../../spdlog.hpp"
#include "autd3/core/interface.hpp"
#include "autd3/core/link.hpp"
#include "autd3/driver/common/cpu/ec_config.hpp"

namespace autd3::link {

class SimulatorImpl final : public core::Link {
 public:
  SimulatorImpl() noexcept : Link() {}
  ~SimulatorImpl() override { close(); }
  SimulatorImpl(const SimulatorImpl& v) noexcept = delete;
  SimulatorImpl& operator=(const SimulatorImpl& obj) = delete;
  SimulatorImpl(SimulatorImpl&& obj) = delete;
  SimulatorImpl& operator=(SimulatorImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    if (is_open()) {
      spdlog::warn("Link is already opened.");
      return false;
    }

    _input_offset = driver::HEADER_SIZE + geometry.num_transducers() * sizeof(uint16_t);
    const auto datagram_size =
        driver::HEADER_SIZE + geometry.num_transducers() * sizeof(uint16_t) + geometry.device_map().size() * driver::EC_INPUT_FRAME_SIZE;
    const auto geometry_size =
        sizeof(uint8_t) + sizeof(uint32_t) + sizeof(uint32_t) * geometry.device_map().size() + geometry.num_transducers() * sizeof(float) * 7;

    const auto size = (std::max)(datagram_size, geometry_size);
    try {
      _smem.create("autd3_simulator_smem", size);
    } catch (std::exception& ex) {
      spdlog::error("Failed to create shared memory: {}", ex.what());
      return false;
    }
    _ptr = static_cast<uint8_t*>(_smem.map());

    if (!send(simulator_init_datagram(geometry))) {
      spdlog::error("Failed to init simulator.");
      return false;
    }

    for (size_t i = 0; i < 20; i++) {
      std::this_thread::sleep_for(std::chrono::milliseconds(100));
      if (_ptr[0] != driver::MSG_SIMULATOR_INIT) return true;
    }

    _smem.unmap();
    _ptr = nullptr;
    spdlog::error("Failed to open simulator. Make sure simulator is running.");
    return false;
  }

  bool close() override {
    if (!is_open()) return true;

    if (!send(simulator_close_datagram())) {
      spdlog::error("Failed to close simulator.");
      return false;
    }

    _smem.unmap();
    _ptr = nullptr;

    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    if (_ptr == nullptr) return false;
    std::memcpy(_ptr, tx.data().data(), tx.effective_size());
    return true;
  }

  bool receive(driver::RxDatagram& rx) override {
    if (_ptr == nullptr) return false;
    rx.copy_from(reinterpret_cast<const driver::RxMessage*>(_ptr + _input_offset));
    return true;
  }

  bool is_open() override { return _ptr != nullptr; }

 private:
  smem::SMem _smem;
  uint8_t* _ptr{nullptr};

  size_t _input_offset{0};

  [[nodiscard]] bool send(const std::vector<uint8_t>& raw) const {
    if (_ptr == nullptr) return false;
    std::memcpy(_ptr, raw.data(), raw.size());
    return true;
  }

  static std::vector<uint8_t> simulator_init_datagram(const core::Geometry& geometry) {
    std::vector<uint8_t> data;
    const auto geometry_size =
        sizeof(uint8_t) + sizeof(uint32_t) + sizeof(uint32_t) * geometry.device_map().size() + geometry.num_transducers() * sizeof(float) * 7;
    data.resize(geometry_size);

    auto* cursor = data.data();
    *cursor++ = driver::MSG_SIMULATOR_INIT;
    *reinterpret_cast<uint32_t*>(cursor) = static_cast<uint32_t>(geometry.device_map().size());
    cursor += sizeof(uint32_t);

    size_t i = 0;
    size_t c = 0;
    for (size_t dev = 0; dev < geometry.device_map().size(); dev++) {
      c += geometry.device_map()[dev];
      *reinterpret_cast<uint32_t*>(cursor) = static_cast<uint32_t>(geometry.device_map()[dev]);
      cursor += sizeof(uint32_t);
      auto* p = reinterpret_cast<float*>(cursor);
      for (; i < c; i++) {
        auto& tr = geometry[i];
        auto origin = tr.position().cast<float>();
        auto rot = tr.rotation().cast<float>();
        p[0] = origin.x();
        p[1] = origin.y();
        p[2] = origin.z();
        p[3] = rot.w();
        p[4] = rot.x();
        p[5] = rot.y();
        p[6] = rot.z();
        p += 7;
        cursor += 7 * sizeof(float);
      }
    }
    return data;
  }

  static std::vector<uint8_t> simulator_close_datagram() { return {driver::MSG_SIMULATOR_CLOSE}; }
};

core::LinkPtr Simulator::build() const {
  core::LinkPtr link = std::make_unique<SimulatorImpl>();
  return link;
}

}  // namespace autd3::link
