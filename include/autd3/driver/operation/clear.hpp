// File: clear.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/cpu/datagram.hpp"

namespace autd3::driver {

struct Clear final {
  void init() { _sent = false; }

  void pack(TxDatagram& tx) {
    tx.header().msg_id = MSG_CLEAR;
    tx.num_bodies = 0;
    _sent = true;
  }

  [[nodiscard]] bool is_finished() const { return _sent; }

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
