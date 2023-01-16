// File: clear.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 16/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/operation/operation.hpp"

namespace autd3::driver {

struct Clear final : Operation {
  void init() override { _sent = false; }

  void pack(TxDatagram& tx) override {
    tx.header().msg_id = MSG_CLEAR;
    tx.num_bodies = 0;
    _sent = true;
  }

  [[nodiscard]] bool is_finished() const override { return _sent; }

 private:
  bool _sent{false};
};

}  // namespace autd3::driver
