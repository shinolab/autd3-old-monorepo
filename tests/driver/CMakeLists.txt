file(GLOB test_files *.cpp)

target_sources(test_autd3 PRIVATE
  ${test_files}
)
