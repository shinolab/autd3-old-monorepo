cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_GEOMETRY_VIEWER)
  return()
endif()

add_library(autd3capi-extra-geometry-viewer SHARED
  c_api.cpp
  geometry_viewer.h
  ../base/header.hpp
)

target_link_libraries(autd3capi-extra-geometry-viewer geometry_viewer autd3_common)

set_target_properties(autd3capi-extra-geometry-viewer
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-extra-geometry-viewer PROPERTIES FOLDER "capi/extra")
