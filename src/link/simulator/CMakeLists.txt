cmake_minimum_required(VERSION 3.16)

add_library(link_simulator
  simulator.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/simulator.hpp
)
target_link_libraries(link_simulator PUBLIC autd3_common)
target_link_libraries(link_simulator INTERFACE simulator)

if(WIN32)
  target_link_libraries(link_simulator PUBLIC ws2_32)
endif()

set_target_properties(link_simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_simulator PROPERTIES FOLDER "link")
set_target_properties(link_simulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
