// File: bench.cpp
// Project: bench
// Created Date: 07/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <benchmark/benchmark.h>

int main() {
  char arg0[] = "benchmark";
  char arg1[] = "--benchmark_time_unit=ms";
  std::vector args = {arg0, arg1};
  int argc = static_cast<int>(args.size());
  char** argv = args.data();
  benchmark::Initialize(&argc, argv);
  if (benchmark::ReportUnrecognizedArguments(argc, argv)) return 1;
  benchmark::RunSpecifiedBenchmarks();
  benchmark::Shutdown();
  return 0;
}
