cmake_minimum_required(VERSION 3.21)

add_library(autd3capi-link-debug SHARED
  c_api.cpp
  debug_link.h
  ../base/header.hpp
  ../base/wrapper_link.hpp
)
 
target_link_libraries(autd3capi-link-debug link_debug)

set_target_properties(autd3capi-link-debug
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-link-debug PROPERTIES FOLDER "capi/link")
