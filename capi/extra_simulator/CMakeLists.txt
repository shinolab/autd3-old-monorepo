cmake_minimum_required(VERSION 3.16)

add_library(autd3capi-extra-simulator SHARED
  c_api.cpp
  simulator.h
  ../base/header.hpp
)
 
target_link_libraries(autd3capi-extra-simulator simulator autd3_common)

set_target_properties(autd3capi-extra-simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-extra-simulator PROPERTIES FOLDER "capi/extra")
