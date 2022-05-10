// File: datagram.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <algorithm>
#include <cstdint>
#include <vector>

#include "body.hpp"
#include "header.hpp"

namespace autd3::driver {

struct TxDatagram {
  size_t num_bodies;

  explicit TxDatagram(const size_t size) : num_bodies(size), _size(size) { _data.resize(sizeof(GlobalHeader) + sizeof(Body) * size, 0x00); }

  [[nodiscard]] size_t size() const { return sizeof(GlobalHeader) + sizeof(Body) * num_bodies; }

  std::span<uint8_t> data() { return std::span{_data}; }

  GlobalHeader &header() { return *reinterpret_cast<GlobalHeader *>(_data.data()); }
  [[nodiscard]] GlobalHeader const &header() const { return *reinterpret_cast<GlobalHeader const *const>(_data.data()); }

  std::span<Body> bodies() { return std::span{reinterpret_cast<Body *>(_data.data() + sizeof(GlobalHeader)), _size}; }

  void clear() {
    header().clear();
    num_bodies = 0;
  }

 private:
  std::vector<uint8_t> _data;
  size_t _size;
};

struct RxMessage {
  uint8_t ack;
  uint8_t msg_id;

  RxMessage() noexcept : ack(0), msg_id() {}
};

struct RxDatagram {
  explicit RxDatagram(const size_t size) { _data.resize(size); }

  std::span<RxMessage> messages() { return std::span{_data}; }

  bool is_msg_processed(uint8_t msg_id) {
    return std::ranges::all_of(_data, [msg_id](const RxMessage msg) { return msg.msg_id == msg_id; });
  }

 private:
  std::vector<RxMessage> _data;
};

}  // namespace autd3::driver
