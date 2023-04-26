cmake_minimum_required(VERSION 3.21)

if(NOT ENABLE_BENCH)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/gbenchmark.cmake)

add_executable(bench_autd3)
target_link_libraries(bench_autd3 PRIVATE benchmark::benchmark)
target_link_libraries(bench_autd3 PRIVATE autd3)

target_sources(bench_autd3 PRIVATE bench.cpp)

add_subdirectory(gain)

set_target_properties(bench_autd3 PROPERTIES FOLDER "bench")
set_target_properties(benchmark PROPERTIES FOLDER "bench/banchmark")
set_target_properties(benchmark_main PROPERTIES FOLDER "bench/banchmark")
