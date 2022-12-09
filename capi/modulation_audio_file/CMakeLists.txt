cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_MODULATION_AUDIO_FILE)
  return()
endif()

add_library(autd3capi-modulation-audio-file SHARED
  c_api.cpp
  audio_file_modulation.h
  ../base/header.hpp
)

target_link_libraries(autd3capi-modulation-audio-file modulation_audio_file)
if(WIN32)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-modulation-audio-file -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-modulation-audio-file
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-modulation-audio-file PROPERTIES FOLDER "capi/modulation")
set_target_properties(autd3capi-modulation-audio-file PROPERTIES POSITION_INDEPENDENT_CODE ON)
