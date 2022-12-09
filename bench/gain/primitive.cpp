// File: primitive.cpp
// Project: gain
// Created Date: 09/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 09/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <benchmark/benchmark.h>

#include "../utils.hpp"

static void bm_gain_primitive_focus(benchmark::State& state) {
  autd3::Controller autd;
  setup_autd(autd, state.range(0), state.range(1));

  const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(center);

  for (auto _ : state) {
    g.rebuild(autd.geometry());
  }
}

BENCHMARK(bm_gain_primitive_focus)->Args({1, 1})->Args({ 2, 2 })->Args({ 3, 3 })->Args({ 4, 4 })->Args({ 10, 10 });
