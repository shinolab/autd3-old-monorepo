// File: remote_soem_local.hpp
// Project: remote_soem
// Created Date: 02/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 20/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/core/link.hpp>
#include <autd3/driver/cpu/ec_config.hpp>

#include "smem.hpp"

namespace autd3::link {

class RemoteSOEMLocal final : public core::Link {
 public:
  RemoteSOEMLocal() : Link(), _ptr(nullptr), _output_size(0) {}
  ~RemoteSOEMLocal() override = default;
  RemoteSOEMLocal(const RemoteSOEMLocal& v) noexcept = delete;
  RemoteSOEMLocal& operator=(const RemoteSOEMLocal& obj) = delete;
  RemoteSOEMLocal(RemoteSOEMLocal&& obj) = delete;
  RemoteSOEMLocal& operator=(RemoteSOEMLocal&& obj) = delete;

  bool open(const core::Geometry& geometry) override {
    if (is_open()) return true;

    _output_size = driver::HEADER_SIZE + std::accumulate(geometry.device_map().begin(), geometry.device_map().end(), size_t{0}) * sizeof(uint16_t);

    const auto size = _output_size + geometry.num_devices() * driver::EC_INPUT_FRAME_SIZE;
    _smem.create("autd3_soem_server_smem", size);
    _ptr = static_cast<uint8_t*>(_smem.map());

    return true;
  }

  bool close() override {
    if (!is_open()) return true;

    _ptr[0] = driver::MSG_SERVER_CLOSE;

    _smem.unmap();
    _ptr = nullptr;

    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    if (_ptr == nullptr) return false;
    std::memcpy(_ptr, tx.data().data(), tx.transmitting_size_in_bytes());
    return true;
  }

  bool receive(driver::RxDatagram& rx) override {
    if (_ptr == nullptr) return false;
    rx.copy_from(reinterpret_cast<const driver::RxMessage*>(_ptr + _output_size));
    return true;
  }

  bool is_open() override { return _ptr != nullptr; }

 private:
  smem::SMem _smem;
  uint8_t* _ptr;
  size_t _output_size;
};

}  // namespace autd3::link
