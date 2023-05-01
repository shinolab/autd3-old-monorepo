cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_REMOTE_SIMULATOR)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/boost.cmake)

add_library(autd3_link_remote_simulator remote_simulator.cpp)
add_library(autd3::link::remote_simulator ALIAS autd3_link_remote_simulator)
target_include_directories(autd3_link_remote_simulator PRIVATE ${Boost_INCLUDE_DIRS})
target_link_libraries(autd3_link_remote_simulator PRIVATE autd3::common)

if(WIN32)
  target_link_libraries(autd3_link_remote_simulator INTERFACE ws2_32)
endif()

set_target_properties(autd3_link_remote_simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_link_remote_simulator PROPERTIES FOLDER "link")
set_target_properties(autd3_link_remote_simulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
