cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_REMOTE_SOEM)
  return()
endif()

add_library(autd3_link_remote_soem
  remote_soem.cpp
  remote_soem_local.hpp
  remote_soem_tcp.hpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/remote_soem.hpp
)
add_library(autd3::link::remote_soem ALIAS autd3_link_remote_soem)
target_link_libraries(autd3_link_remote_soem PRIVATE autd3::common)
target_include_directories(autd3_link_remote_soem PRIVATE ${PROJECT_SOURCE_DIR}/src/smem)
target_compile_definitions(autd3_link_remote_soem PRIVATE SMEM_HEADER_ONLY)

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
