cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_DEBUG)
  return()
endif()

add_library(link_debug_proxy
  debug_proxy.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/debug_proxy.hpp
)
target_link_libraries(link_debug_proxy PUBLIC autd3_common)

set_target_properties(link_debug_proxy
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_debug_proxy PROPERTIES FOLDER "link")
set_target_properties(link_debug_proxy PROPERTIES POSITION_INDEPENDENT_CODE ON)
