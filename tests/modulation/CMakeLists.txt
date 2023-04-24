target_sources(test_autd3 PRIVATE
  primitive.cpp
)

if(BUILD_MODULATION_AUDIO_FILE)
  target_sources(test_autd3 PRIVATE audio_file.cpp)
  target_link_libraries(test_autd3 autd3::modulation::audio_file)
  target_compile_definitions(test_autd3 PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/examples/resources")
endif()
