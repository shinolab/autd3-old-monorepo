// File: transtest.hpp
// Project: tests
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>

template <typename L>
inline void tran_test(autd3::Controller<L>& autd) {
  autd3::Silencer silencer;
  autd.send_async(silencer).get();

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::gain::TransducerTest g([](const auto& dev, const auto& tr) -> std::optional<autd3::Drive> {
    if (dev.idx() == 0 && tr.idx() == 0) return autd3::Drive(autd3::Phase(0), autd3::EmitIntensity::maximum());
    if (dev.idx() == 0 && tr.idx() == 248) return autd3::Drive(autd3::Phase(0), autd3::EmitIntensity::maximum());
    return std::nullopt;
  });
  autd.send_async(m, g).get();
}
