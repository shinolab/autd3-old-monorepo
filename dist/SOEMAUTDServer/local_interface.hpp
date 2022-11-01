// File: local_interface.hpp
// Project: SOEMAUTDServer
// Created Date: 01/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/driver/cpu/ec_config.hpp>
#include <smem/smem.hpp>

#include "interface.hpp"

namespace autd3::publish {

class LocalInterface final : public Interface {
 public:
  explicit LocalInterface(const size_t dev) noexcept : _dev(dev) {}
  ~LocalInterface() override = default;
  LocalInterface(const LocalInterface& v) noexcept = delete;
  LocalInterface& operator=(const LocalInterface& obj) = delete;
  LocalInterface(LocalInterface&& obj) = default;
  LocalInterface& operator=(LocalInterface&& obj) = default;

  void connect() override {
    const auto size = driver::HEADER_SIZE + _dev * (driver::BODY_SIZE + driver::EC_INPUT_FRAME_SIZE);
    _smem.create("autd3_soem_server_smem", size);
    _ptr = static_cast<uint8_t*>(_smem.map());
    std::memset(_ptr, 0, size);
    _last_msg_id = 0;
  }

  void close() override {
    _smem.unmap();
    _smem.close();
  }

  bool tx(driver::TxDatagram& tx) override {
    const auto msg_id = _ptr[0];
    if (_last_msg_id == msg_id) return false;
    _last_msg_id = msg_id;
    std::memcpy(tx.data().data(), _ptr, tx.effective_size());
    return true;
  }

  void rx(driver::RxDatagram& rx) override {
    std::memcpy(_ptr + driver::HEADER_SIZE + rx.messages().size() * driver::BODY_SIZE, rx.messages().data(),
                rx.messages().size() * driver::EC_INPUT_FRAME_SIZE);
  }

 private:
  smem::SMem _smem{};
  size_t _dev{0};
  uint8_t* _ptr{nullptr};
  uint8_t _last_msg_id{0};
};

}  // namespace autd3::publish
