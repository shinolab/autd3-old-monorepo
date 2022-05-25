cmake_minimum_required(VERSION 3.16)

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
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

if(WIN32)
  set_target_properties(autd3capi-modulation-audio-file PROPERTIES FOLDER "capi/modulation")
else()
  set_target_properties(autd3capi-modulation-audio-file PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
