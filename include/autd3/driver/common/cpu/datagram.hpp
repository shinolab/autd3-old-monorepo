// File: datagram.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cstdint>
#include <numeric>
#include <vector>

#include "body.hpp"
#include "header.hpp"

namespace autd3::driver {

struct TxDatagram {
  size_t num_bodies;

  explicit TxDatagram(const std::vector<size_t> &device_map) : num_bodies(device_map.size()) {
    _trans_num_prefix_sum.resize(device_map.size() + 1, 0);
    for (size_t i = 0; i < device_map.size(); i++) _trans_num_prefix_sum[i + 1] = _trans_num_prefix_sum[i] + device_map[i];
    _data.resize(sizeof(GlobalHeader) + sizeof(uint16_t) * _trans_num_prefix_sum[_trans_num_prefix_sum.size() - 1], 0x00);
  }
  ~TxDatagram() = default;
  TxDatagram(const TxDatagram &v) noexcept = delete;
  TxDatagram &operator=(const TxDatagram &obj) = delete;
  TxDatagram(TxDatagram &&obj) = default;
  TxDatagram &operator=(TxDatagram &&obj) = default;

  [[nodiscard]] TxDatagram clone() const {
    TxDatagram tx;
    tx.num_bodies = num_bodies;
    tx._trans_num_prefix_sum = _trans_num_prefix_sum;
    tx._data = _data;
    return tx;
  }

  [[nodiscard]] size_t num_devices() const noexcept { return _trans_num_prefix_sum.size() - 1; }

  [[nodiscard]] size_t effective_size() const noexcept {
    const auto num_transducers = _trans_num_prefix_sum[num_bodies];
    return sizeof(GlobalHeader) + sizeof(uint16_t) * num_transducers;
  }

  [[nodiscard]] size_t bodies_size() const noexcept {
    const auto num_transducers = _trans_num_prefix_sum[num_bodies];
    return sizeof(uint16_t) * num_transducers;
  }

  std::vector<uint8_t> &data() noexcept { return _data; }
  [[nodiscard]] const std::vector<uint8_t> &data() const noexcept { return _data; }

  GlobalHeader &header() noexcept { return *reinterpret_cast<GlobalHeader *>(_data.data()); }
  [[nodiscard]] GlobalHeader const &header() const noexcept { return *reinterpret_cast<GlobalHeader const *const>(_data.data()); }

  Body *bodies_ptr() noexcept { return reinterpret_cast<Body *>(_data.data() + sizeof(GlobalHeader)); }

  Body &body(const size_t idx) noexcept {
    const auto start = _trans_num_prefix_sum[idx];
    return *reinterpret_cast<Body *>(_data.data() + sizeof(GlobalHeader) + start);
  }

  [[nodiscard]] const Body &body(const size_t idx) const noexcept {
    const auto start = _trans_num_prefix_sum[idx];
    return *reinterpret_cast<const Body *>(_data.data() + sizeof(GlobalHeader) + start);
  }

  void clear() { std::memset(_data.data(), 0, _data.size()); }

 private:
  TxDatagram() : num_bodies(0) {}

  std::vector<size_t> _trans_num_prefix_sum;
  std::vector<uint8_t> _data;
};

struct RxMessage {
  uint8_t ack;
  uint8_t msg_id;

  RxMessage() noexcept : ack(0), msg_id() {}
};

struct RxDatagram {
  explicit RxDatagram(const size_t size) { _data.resize(size); }

  std::vector<RxMessage> &messages() noexcept { return _data; }
  [[nodiscard]] const std::vector<RxMessage> &messages() const noexcept { return _data; }

  bool is_msg_processed(uint8_t msg_id) {
    return std::all_of(_data.begin(), _data.end(), [msg_id](const RxMessage msg) noexcept { return msg.msg_id == msg_id; });
  }

  void copy_from(const RxMessage *const src) { std::memcpy(_data.data(), src, _data.size() * sizeof(RxMessage)); }

  [[nodiscard]] std::vector<RxMessage>::const_iterator begin() const noexcept { return _data.begin(); }
  [[nodiscard]] std::vector<RxMessage>::const_iterator end() const noexcept { return _data.end(); }
  [[nodiscard]] std::vector<RxMessage>::iterator begin() noexcept { return _data.begin(); }
  [[nodiscard]] std::vector<RxMessage>::iterator end() noexcept { return _data.end(); }

  RxMessage &operator[](const size_t i) noexcept { return _data.at(i); }
  const RxMessage &operator[](const size_t i) const noexcept { return _data.at(i); }

  void clear() {
    std::for_each(_data.begin(), _data.end(), [](RxMessage &msg) {
      msg.ack = 0;
      msg.msg_id = 0;
    });
  }

 private:
  std::vector<RxMessage> _data;
};

}  // namespace autd3::driver
