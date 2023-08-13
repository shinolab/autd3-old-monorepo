/*
 * @Author: Mingxin Zhang m.zhang@hapis.k.u-tokyo.ac.jp
 * @Date: 2023-08-13 21:25:53
 * @LastEditors: Mingxin Zhang
 * @LastEditTime: 2023-08-13 21:31:18
 * Copyright (c) 2023 by Mingxin Zhang, All Rights Reserved. 
 */

#pragma once

#include "autd3.hpp"
#include <math.h>

#define PI acos(-1)

inline void fourier_test(autd3::Controller& autd) {
  autd3::SilencerConfig silencer;
  autd.send(silencer);

  auto sine_1 = autd3::modulation::Sine(100).with_phase(PI/4);
  auto sine_2 = autd3::modulation::Sine(150).with_phase(PI/2);
  auto m = autd3::modulation::Fourier()
                    .add_component(sine_1)
                    .add_component(sine_2);


  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(center);

  autd.send(m, g);
}