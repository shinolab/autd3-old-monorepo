cmake_minimum_required(VERSION 3.21)

add_library(link_bundle
  bundle.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/bundle.hpp
)
target_link_libraries(link_bundle PUBLIC autd3_common)

set_target_properties(link_bundle
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_bundle PROPERTIES FOLDER "link")
set_target_properties(link_bundle PROPERTIES POSITION_INDEPENDENT_CODE ON)
