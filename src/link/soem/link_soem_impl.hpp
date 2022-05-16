// File: link_soem_impl.hpp
// Project: soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <atomic>
#include <cstring>
#include <functional>
#include <memory>
#include <mutex>
#include <queue>
#include <string>
#include <thread>
#include <utility>

#include "autd3/core/link.hpp"
#include "autd3/driver/cpu/body.hpp"
#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/driver/cpu/header.hpp"

namespace autd3::link {

struct IOMap {
  IOMap() : _size(0), _buf(nullptr), _device_num(0) {}

  explicit IOMap(const size_t device_num)
      : _size(device_num * (driver::HEADER_SIZE + driver::BODY_SIZE + driver::EC_INPUT_FRAME_SIZE)),
        _buf(std::make_unique<uint8_t[]>(_size)),
        _device_num(device_num) {}

  void resize(const size_t device_num) {
    if (const auto size = device_num * (driver::HEADER_SIZE + driver::BODY_SIZE + driver::EC_INPUT_FRAME_SIZE); _size != size) {
      _device_num = device_num;
      _size = size;
      _buf = std::make_unique<uint8_t[]>(_size);
    }
  }

  [[nodiscard]] size_t size() const { return _size; }

  driver::GlobalHeader* header(const size_t i) {
    return reinterpret_cast<driver::GlobalHeader*>(&_buf[(driver::HEADER_SIZE + driver::BODY_SIZE) * i + driver::BODY_SIZE]);
  }

  driver::Body* body(const size_t i) { return reinterpret_cast<driver::Body*>(&_buf[(driver::HEADER_SIZE + driver::BODY_SIZE) * i]); }

  [[nodiscard]] const driver::RxMessage* input() const {
    return reinterpret_cast<const driver::RxMessage*>(&_buf[(driver::HEADER_SIZE + driver::BODY_SIZE) * _device_num]);
  }

  void copy_from(driver::TxDatagram& tx) {
    for (size_t i = 0; i < tx.num_bodies; i++) std::memcpy(body(i), tx.bodies() + i, sizeof(driver::Body));
    for (size_t i = 0; i < _device_num; i++) std::memcpy(header(i), tx.data().data(), sizeof(driver::GlobalHeader));
  }

  [[nodiscard]] uint8_t* get() const { return _buf.get(); }

 private:
  size_t _size;
  std::unique_ptr<uint8_t[]> _buf;
  size_t _device_num;
};

class SOEMLink final : public core::Link {
 public:
  SOEMLink(const bool high_precision, std::string ifname, const size_t dev_num, const uint16_t cycle_ticks, std::function<void(std::string)> on_lost)
      : Link(cycle_ticks),
        _high_precision(high_precision),
        _ifname(std::move(ifname)),
        _cycle_ticks(cycle_ticks),
        _on_lost(std::move(on_lost)),
        _dev_num(dev_num),
        _is_open(false),
        _is_running(false) {}

  ~SOEMLink() override;
  SOEMLink(const SOEMLink& v) noexcept = delete;
  SOEMLink& operator=(const SOEMLink& obj) = delete;
  SOEMLink(SOEMLink&& obj) = delete;
  SOEMLink& operator=(SOEMLink&& obj) = delete;

  void open() override;
  bool send(const driver::TxDatagram& tx) override;
  bool receive(driver::RxDatagram& rx) override;
  void close() override;
  bool is_open() override;

 private:
  bool _high_precision;
  std::string _ifname;
  uint16_t _cycle_ticks;

  std::function<void(std::string)> _on_lost = nullptr;

  IOMap _io_map;
  size_t _dev_num;
  std::atomic<bool> _is_open;
  std::unique_ptr<uint32_t[]> _user_data;

  bool _is_running;
  std::thread _ecat_thread;

  std::queue<driver::TxDatagram> _send_buf;
  std::mutex _send_mtx;
};

}  // namespace autd3::link
