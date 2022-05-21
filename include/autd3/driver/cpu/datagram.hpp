// File: datagram.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/05/2022
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
  ~TxDatagram() = default;
  TxDatagram(const TxDatagram &v) noexcept = delete;
  TxDatagram &operator=(const TxDatagram &obj) = delete;
  TxDatagram(TxDatagram &&obj) = default;
  TxDatagram &operator=(TxDatagram &&obj) = default;

  [[nodiscard]] TxDatagram clone() const {
    TxDatagram tx(_size);
    std::copy(_data.begin(), _data.end(), tx._data.begin());
    return tx;
  }

  [[nodiscard]] size_t size() const noexcept { return _size; }

  [[nodiscard]] size_t effective_size() const noexcept { return sizeof(GlobalHeader) + sizeof(Body) * num_bodies; }

  std::vector<uint8_t> &data() noexcept { return _data; }

  [[nodiscard]] const std::vector<uint8_t> &data() const noexcept { return _data; }

  GlobalHeader &header() noexcept { return *reinterpret_cast<GlobalHeader *>(_data.data()); }
  [[nodiscard]] GlobalHeader const &header() const noexcept { return *reinterpret_cast<GlobalHeader const *const>(_data.data()); }

  Body *bodies() noexcept { return reinterpret_cast<Body *>(_data.data() + sizeof(GlobalHeader)); }

 private:
  size_t _size;
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

 private:
  std::vector<RxMessage> _data;
};

}  // namespace autd3::driver
