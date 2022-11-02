// File: remote_soem_local.hpp
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

#include <autd3/core/link.hpp>
#include <autd3/driver/cpu/ec_config.hpp>
#include <smem/smem.hpp>

namespace autd3::link {

class RemoteSOEMLocal final : public core::Link {
 public:
  RemoteSOEMLocal() : Link(), _ptr(nullptr) {}
  ~RemoteSOEMLocal() override = default;
  RemoteSOEMLocal(const RemoteSOEMLocal& v) noexcept = delete;
  RemoteSOEMLocal& operator=(const RemoteSOEMLocal& obj) = delete;
  RemoteSOEMLocal(RemoteSOEMLocal&& obj) = delete;
  RemoteSOEMLocal& operator=(RemoteSOEMLocal&& obj) = delete;

  void open(const core::Geometry& geometry) override {
    if (is_open()) return;

    const auto size = driver::HEADER_SIZE + geometry.num_devices() * (driver::BODY_SIZE + driver::EC_INPUT_FRAME_SIZE);
    _smem.create("autd3_soem_server_smem", size);
    _ptr = static_cast<uint8_t*>(_smem.map());
  }

  void close() override {
    if (!is_open()) return;

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
    rx.copy_from(reinterpret_cast<const driver::RxMessage*>(_ptr + driver::HEADER_SIZE + rx.messages().size() * driver::BODY_SIZE));
    return true;
  }

  bool is_open() override { return _ptr != nullptr; }

 private:
  smem::SMem _smem;
  uint8_t* _ptr;
};

}  // namespace autd3::link
