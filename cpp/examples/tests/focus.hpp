/*
 * @Author: Mingxin Zhang m.zhang@hapis.k.u-tokyo.ac.jp
 * @Date: 2023-07-27 22:01:35
 * @LastEditors: Mingxin Zhang
 * @LastEditTime: 2023-08-13 21:25:33
 * Copyright (c) 2023 by Mingxin Zhang, All Rights Reserved. 
 */
// File: focus.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3.hpp"

inline void focus_test(autd3::Controller& autd) {
  autd3::SilencerConfig silencer;
  autd.send(silencer);

  auto m = autd3::modulation::Sine(150);

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(center);

  autd.send(m, g);
}
