cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_GEOMETRY_VIEWER)
  return()
endif()

add_library(autd3capi-extra-geometry-viewer SHARED c_api.cpp)

target_link_libraries(autd3capi-extra-geometry-viewer PRIVATE autd3::extra::geometry_viewer autd3::common)

set_target_properties(autd3capi-extra-geometry-viewer
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-extra-geometry-viewer PROPERTIES FOLDER "capi/extra")
