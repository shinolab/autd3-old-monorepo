cmake_minimum_required(VERSION 3.21)

add_library(link_debug
  debug.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/debug.hpp
)
target_link_libraries(link_debug PUBLIC autd3_common)

set_target_properties(link_debug
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_debug PROPERTIES FOLDER "link")
set_target_properties(link_debug PROPERTIES POSITION_INDEPENDENT_CODE ON)
