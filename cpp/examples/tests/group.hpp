// File: group.hpp
// Project: tests
// Created Date: 15/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 15/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

inline void group_test(autd3::Controller& autd) {
  autd3::Silencer silencer;
  autd.send(silencer);

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  autd.group([](const autd3::Device& dev) -> std::optional<const char*> {
        if (dev.idx() == 0) {
          return "null";
        } else if (dev.idx() == 1) {
          return "focus";
        } else {
          return std::nullopt;
        }
      })
      .set("null", autd3::modulation::Static(), autd3::gain::Null())
      .set("focus", autd3::modulation::Sine(150), autd3::gain::Focus(center))
      .send();
}
