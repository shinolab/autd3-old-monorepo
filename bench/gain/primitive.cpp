// File: primitive.cpp
// Project: gain
// Created Date: 09/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 31/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26495)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include <benchmark/benchmark.h>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

#include "../utils.hpp"

static void bm_gain_primitive_focus(benchmark::State& state) {
  const auto geometry = setup_geometry(state.range(0), state.range(1));

  const autd3::Vector3 center = geometry.center() + autd3::Vector3(0.0, 0.0, 150.0);
  autd3::gain::Focus g(center);

  for (auto _ : state) {
    g.calc(geometry);
  }
}

BENCHMARK(bm_gain_primitive_focus)->Args({1, 1})->Args({2, 2})->Args({3, 3})->Args({4, 4})->Args({10, 10});
