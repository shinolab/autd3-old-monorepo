cmake_minimum_required(VERSION 3.21)

add_library(autd3
  ${PROJECT_SOURCE_DIR}/include/autd3/controller.hpp
  controller.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/modulation/lpf.hpp
  modulation/lpf/lpf.cpp
)
target_link_libraries(autd3 PUBLIC autd3_common)

set_target_properties(autd3
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3 PROPERTIES POSITION_INDEPENDENT_CODE ON)
