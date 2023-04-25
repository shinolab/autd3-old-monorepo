// File: simulator.cpp
// Project: simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/simulator.hpp"

#include <boost/interprocess/managed_shared_memory.hpp>
#include <boost/interprocess/sync/interprocess_mutex.hpp>
#include <mutex>
#include <thread>

#include "autd3/core/datagram.hpp"
#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/ec_config.hpp"

namespace autd3::link {

class SimulatorImpl final : public core::Link {
  static constexpr std::string_view SHMEM_NAME{"autd3_simulator_shmem"};
  static constexpr std::string_view SHMEM_MTX_NAME{"autd3_simulator_shmem_mtx"};
  static constexpr std::string_view SHMEM_DATA_NAME{"autd3_simulator_shmem_ptr"};

 public:
  explicit SimulatorImpl(const core::Duration timeout) noexcept : Link(timeout) {}
  ~SimulatorImpl() override { close(); }
  SimulatorImpl(const SimulatorImpl& v) noexcept = delete;
  SimulatorImpl& operator=(const SimulatorImpl& obj) = delete;
  SimulatorImpl(SimulatorImpl&& obj) = delete;
  SimulatorImpl& operator=(SimulatorImpl&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    if (is_open()) return false;

    _input_offset = driver::HEADER_SIZE + geometry.num_transducers() * sizeof(uint16_t);
    const auto datagram_size =
        driver::HEADER_SIZE + geometry.num_transducers() * sizeof(uint16_t) + geometry.num_devices() * driver::EC_INPUT_FRAME_SIZE;
    const auto geometry_size =
        sizeof(uint8_t) + sizeof(uint32_t) + sizeof(uint32_t) * geometry.num_devices() + geometry.num_transducers() * sizeof(float) * 7;

    const auto size = (std::max)(datagram_size, geometry_size);

    _segment = boost::interprocess::managed_shared_memory(boost::interprocess::open_only, std::string(SHMEM_NAME).c_str());
    _ptr = _segment.find<uint8_t>(std::string(SHMEM_DATA_NAME).c_str()).first;
    _mtx = _segment.find<boost::interprocess::interprocess_mutex>(std::string(SHMEM_MTX_NAME).c_str()).first;

    if (!send(simulator_init_datagram(geometry))) throw std::runtime_error("Failed to init simulator.");

    for (size_t i = 0; i < 20; i++) {
      std::this_thread::sleep_for(std::chrono::milliseconds(100));
      if (_ptr[0] != driver::MSG_SIMULATOR_INIT) {
        _is_open = true;
        return true;
      }
    }

    throw std::runtime_error("Failed to open simulator. Make sure simulator is running.");
  }

  bool close() override {
    if (!is_open()) return true;

    if (!send(simulator_close_datagram())) throw std::runtime_error("Failed to close simulator.");

    _is_open = false;

    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    if (!is_open()) return false;

    {
      std::unique_lock lk(*_mtx);
      std::memcpy(_ptr, tx.data().data(), tx.transmitting_size_in_bytes());
    }

    return true;
  }

  bool receive(driver::RxDatagram& rx) override {
    if (!is_open()) return false;

    {
      std::unique_lock lk(*_mtx);
      rx.copy_from(reinterpret_cast<const driver::RxMessage*>(_ptr + _input_offset));
    }

    return true;
  }

  bool is_open() override { return _is_open; }

 private:
  boost::interprocess::managed_shared_memory _segment{};
  boost::interprocess::interprocess_mutex* _mtx{nullptr};
  uint8_t* _ptr{nullptr};

  size_t _input_offset{0};
  bool _is_open{false};

  [[nodiscard]] bool send(const std::vector<uint8_t>& raw) const {
    if (_ptr == nullptr) return false;
    std::memcpy(_ptr, raw.data(), raw.size());
    return true;
  }

  static std::vector<uint8_t> simulator_init_datagram(const core::Geometry& geometry) {
    std::vector<uint8_t> data;
    const auto geometry_size =
        sizeof(uint8_t) + sizeof(uint32_t) + sizeof(uint32_t) * geometry.num_devices() + geometry.num_transducers() * sizeof(float) * 7;
    data.resize(geometry_size);

    auto* cursor = data.data();
    *cursor++ = driver::MSG_SIMULATOR_INIT;
    *reinterpret_cast<uint32_t*>(cursor) = static_cast<uint32_t>(geometry.num_devices());
    cursor += sizeof(uint32_t);

    size_t i = 0;
    size_t c = 0;
    for (const size_t dev : geometry.device_map()) {
      c += dev;
      *reinterpret_cast<uint32_t*>(cursor) = static_cast<uint32_t>(dev);
      cursor += sizeof(uint32_t);
      auto* p = reinterpret_cast<float*>(cursor);
      while (i < c) {
        auto& tr = geometry[i++];
        Eigen::Vector3<float> origin = tr.position().cast<float>();
        Eigen::Quaternion<float> rot = tr.rotation().cast<float>();
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
  core::LinkPtr link = std::make_unique<SimulatorImpl>(_timeout);
  return link;
}

}  // namespace autd3::link
