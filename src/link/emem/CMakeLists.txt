cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_EMEM)
  return()
endif()

add_library(link_emem
  emem.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/emem.hpp
)
target_link_libraries(link_emem PUBLIC autd3_common)

set_target_properties(link_emem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_emem PROPERTIES FOLDER "link")
set_target_properties(link_emem PROPERTIES POSITION_INDEPENDENT_CODE ON)
