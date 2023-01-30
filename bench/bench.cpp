// File: bench.cpp
// Project: bench
// Created Date: 07/12/2022
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
