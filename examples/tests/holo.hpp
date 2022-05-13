// File: holo.hpp
// Project: tests
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
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

  const autd3::Vector3 center = autd.geometry().center();

  std::vector<autd3::Vector3> foci = {center + autd3::Vector3(30.0, 0.0, 0.0), center - autd3::Vector3(30.0, 0.0, 0.0)};
  std::vector<double> amps = {1.0, 1.0};

  // auto backend = autd3::gain::holo::EigenBackend::create();
  auto backend = autd3::gain::holo::CUDABackend::create();
  autd3::gain::holo::SDP<T> g(backend, foci, amps);

  autd.send(m, g);
}
