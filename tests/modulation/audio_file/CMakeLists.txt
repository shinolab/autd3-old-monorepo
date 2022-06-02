file(GLOB test_files *.cpp)

target_sources(test_autd3 PRIVATE
  ${test_files}
)

target_link_libraries(test_autd3 modulation_audio_file)

target_compile_definitions(test_autd3 PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/examples/resources")
