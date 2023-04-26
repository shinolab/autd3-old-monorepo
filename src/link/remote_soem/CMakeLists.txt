cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_REMOTE_SOEM)
  return()
endif()

add_library(autd3_link_remote_soem remote_soem.cpp)
add_library(autd3::link::remote_soem ALIAS autd3_link_remote_soem)
target_link_libraries(autd3_link_remote_soem PRIVATE autd3::common)

if(WIN32)
  target_link_libraries(autd3_link_remote_soem INTERFACE ws2_32)
endif()

set_target_properties(autd3_link_remote_soem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_link_remote_soem PROPERTIES FOLDER "link")
set_target_properties(autd3_link_remote_soem PROPERTIES POSITION_INDEPENDENT_CODE ON)
