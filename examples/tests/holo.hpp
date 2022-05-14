// File: holo.hpp
// Project: tests
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "autd3.hpp"
#include "autd3/gain/backend_cuda.hpp"
#include "autd3/gain/holo.hpp"

template <typename T>
void holo_test(autd3::Controller<T>& autd) {
  const auto config = autd3::SilencerConfig();
  autd.config_silencer(config);

  autd3::modulation::Sine m(150);  // 150Hz AM

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);

  // auto backend = autd3::gain::holo::EigenBackend<T>::create();
  auto backend = autd3::gain::holo::CUDABackend<T>::create();
  autd3::gain::holo::SDP<T> g(backend);
  g.add_focus(center + autd3::Vector3(30.0, 0.0, 0.0), 1.0);
  g.add_focus(center - autd3::Vector3(30.0, 0.0, 0.0), 1.0);

  autd.send(m, g);
}
