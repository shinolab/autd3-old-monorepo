// File: bench.cpp
// Project: bench
// Created Date: 07/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <benchmark/benchmark.h>

int main() {
  char arg0_default[] = "benchmark";
  char arg1_default[] = "--benchmark_time_unit=ms";
  std::vector args_default = {arg0_default, arg1_default};
  int argc = 2;
  char** argv = args_default.data();
  benchmark::Initialize(&argc, argv);
  if (benchmark::ReportUnrecognizedArguments(argc, argv)) return 1;
  benchmark::RunSpecifiedBenchmarks();
  benchmark::Shutdown();
  return 0;
}