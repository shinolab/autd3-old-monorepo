cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_MODULATION_AUDIO_FILE)
  return()
endif()

add_library(autd3_modulation_audio_file audio_file.cpp)
add_library(autd3::modulation::audio_file ALIAS autd3_modulation_audio_file)

target_link_libraries(autd3_modulation_audio_file PUBLIC autd3::common)

set_target_properties(autd3_modulation_audio_file
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_modulation_audio_file PROPERTIES FOLDER "modulation")
set_target_properties(autd3_modulation_audio_file PROPERTIES POSITION_INDEPENDENT_CODE ON)
