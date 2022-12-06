// File: iomap.hpp
// Project: soem
// Created Date: 01/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::link {

struct IOMap {
  IOMap() : _size(0), _buf(nullptr), _trans_num_prefix_sum() {}

  void resize(const std::vector<size_t>& device_map) {
    std::vector<size_t> trans_num_prefix_sum;
    trans_num_prefix_sum.resize(device_map.size() + 1, 0);
    for (size_t i = 0; i < device_map.size(); i++)
      trans_num_prefix_sum[i + 1] = trans_num_prefix_sum[i] + (driver::HEADER_SIZE + device_map[i] * sizeof(uint16_t));

    if (trans_num_prefix_sum.size() != _trans_num_prefix_sum.size() ||
        !std::equal(trans_num_prefix_sum.cbegin(), trans_num_prefix_sum.cend(), _trans_num_prefix_sum.cbegin())) {
      _trans_num_prefix_sum = trans_num_prefix_sum;
      _size = _trans_num_prefix_sum[_trans_num_prefix_sum.size() - 1] + device_map.size() * driver::EC_INPUT_FRAME_SIZE;
      _buf = std::make_unique<uint8_t[]>(_size);
      _device_map = device_map;
    }
  }

  driver::GlobalHeader* header(const size_t i) {
    return reinterpret_cast<driver::GlobalHeader*>(&_buf[_trans_num_prefix_sum[i] + _device_map[i] * sizeof(uint16_t)]);
  }

  driver::Body* body(const size_t i) { return reinterpret_cast<driver::Body*>(&_buf[_trans_num_prefix_sum[i]]); }

  [[nodiscard]] const driver::RxMessage* input() const {
    return reinterpret_cast<const driver::RxMessage*>(&_buf[_trans_num_prefix_sum[_trans_num_prefix_sum.size() - 1]]);
  }

  void copy_from(driver::TxDatagram& tx) {
    for (size_t i = 0; i < tx.num_bodies; i++) std::memcpy(body(i)->data(), tx.body(i).data(), _device_map[i] * sizeof(uint16_t));
    for (size_t i = 0; i < _device_map.size(); i++) std::memcpy(header(i), tx.data().data(), sizeof(driver::GlobalHeader));
  }

  [[nodiscard]] uint8_t* get() const { return _buf.get(); }

 private:
  size_t _size;
  std::unique_ptr<uint8_t[]> _buf;
  std::vector<size_t> _trans_num_prefix_sum;
  std::vector<size_t> _device_map;
};

}  // namespace autd3::link
