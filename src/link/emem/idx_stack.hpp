// File: idx_stack.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "consts.hpp"

namespace autd3::link {
struct IdxStack {
  IdxStack() : _pushed(0), _pulled(0), _idx(), _data(), _length(), _dc_offset() {}

  void push_index(const uint8_t idx, uint8_t* data, const uint16_t length, const uint16_t dc_offset) {
    if (const auto pushed = static_cast<size_t>(_pushed); pushed < EC_BUF_SIZE) {
      _idx[pushed] = idx;
      _data[pushed] = data;
      _length[pushed] = length;
      _dc_offset[pushed] = dc_offset;
      _pushed++;
    }
  }

  int32_t pull_index() {
    if (_pulled >= _pushed) return -1;
    return _pulled++;
  }

  void clear_index() {
    _pushed = 0;
    _pulled = 0;
  }

  [[nodiscard]] uint8_t idx(const size_t i) const { return _idx[i]; }
  [[nodiscard]] uint8_t* data(const size_t i) const { return _data[i]; }
  [[nodiscard]] uint16_t length(const size_t i) const { return _length[i]; }
  [[nodiscard]] uint16_t dc_offset(const size_t i) const { return _dc_offset[i]; }

 private:
  uint8_t _pushed;
  uint8_t _pulled;
  std::array<uint8_t, EC_BUF_SIZE> _idx;
  std::array<uint8_t*, EC_BUF_SIZE> _data;
  std::array<uint16_t, EC_BUF_SIZE> _length;
  std::array<uint16_t, EC_BUF_SIZE> _dc_offset;
};

}  // namespace autd3::link
