// File: transtest.hpp
// Project: tests
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>

inline void tran_test(autd3::Controller& autd) {
  autd3::Silencer silencer;
  autd.send(silencer);

  autd3::modulation::Sine m(150);  // 150Hz AM

  const auto g = autd3::gain::TransducerTest().set(0, 0, 0, 1).set(0, 248, 0, 1);

  autd.send(m, g);
}
