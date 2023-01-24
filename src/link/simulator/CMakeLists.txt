cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_SIMULATOR)
  return()
endif()

add_library(link_simulator
  simulator.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/simulator.hpp
)
target_link_libraries(link_simulator PUBLIC autd3_common)
target_include_directories(link_simulator PRIVATE ${PROJECT_SOURCE_DIR}/src/smem)
target_compile_definitions(link_simulator PRIVATE SMEM_HEADER_ONLY)

set_target_properties(link_simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_simulator PROPERTIES FOLDER "link")
set_target_properties(link_simulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
