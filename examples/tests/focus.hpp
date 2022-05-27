// File: focus.hpp
// Project: tests
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 27/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "Windows.h"
#include "autd3.hpp"

template <typename T>
void focus_test(autd3::ControllerX<T>& autd) {
  LARGE_INTEGER f;
  QueryPerformanceFrequency(&f);

  autd3::SilencerConfig config;
  LARGE_INTEGER start;
  QueryPerformanceCounter(&start);
  autd.send(config);
  LARGE_INTEGER now;
  QueryPerformanceCounter(&now);
  std::cout << (now.QuadPart - start.QuadPart) * 1000LL / f.QuadPart << std::endl;

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  autd3::gain::Focus<T> g(center);
  QueryPerformanceCounter(&start);
  autd.send(m, g);
  QueryPerformanceCounter(&now);
  std::cout << (now.QuadPart - start.QuadPart) * 1000LL / f.QuadPart << std::endl;
}
