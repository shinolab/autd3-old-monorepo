cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_REMOTE_SOEM)
  return()
endif()

add_library(link_remote_soem
  remote_soem.cpp
  remote_soem_local.hpp
  remote_soem_tcp.hpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/remote_soem.hpp
)
target_link_libraries(link_remote_soem PUBLIC autd3_common smem)

if(WIN32)
  target_link_libraries(link_remote_soem PUBLIC ws2_32)
endif()

set_target_properties(link_remote_soem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_remote_soem PROPERTIES FOLDER "link")
set_target_properties(link_remote_soem PROPERTIES POSITION_INDEPENDENT_CODE ON)
