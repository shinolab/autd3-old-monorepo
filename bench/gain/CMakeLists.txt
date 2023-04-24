target_sources(bench_autd3 PRIVATE primitive.cpp)

if(BUILD_GAIN_HOLO)
  target_sources(bench_autd3 PRIVATE holo.cpp)
  target_link_libraries(bench_autd3 PRIVATE autd3::gain::holo)
  target_link_libraries(bench_autd3 PRIVATE autd3::backend::eigen)
endif()
