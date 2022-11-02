// File: iomap.hpp
// Project: soem
// Created Date: 01/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

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

}  // namespace autd3::link
