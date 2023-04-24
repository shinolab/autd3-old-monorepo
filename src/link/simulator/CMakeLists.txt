cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_SIMULATOR)
  return()
endif()

add_library(autd3_link_simulator simulator.cpp)
add_library(autd3::link::simulator ALIAS autd3_link_simulator)

target_link_libraries(autd3_link_simulator PRIVATE autd3::common)
target_include_directories(autd3_link_simulator PRIVATE ${PROJECT_SOURCE_DIR}/src/smem)
target_compile_definitions(autd3_link_simulator PRIVATE SMEM_HEADER_ONLY)

set_target_properties(autd3_link_simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_link_simulator PROPERTIES FOLDER "link")
set_target_properties(autd3_link_simulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
