cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_TWINCAT)
  return()
endif()

add_library(link_twincat
  twincat.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/twincat.hpp
)
target_link_libraries(link_twincat PUBLIC autd3_common)

set_target_properties(link_twincat
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_twincat PROPERTIES FOLDER "link")
set_target_properties(link_twincat PROPERTIES POSITION_INDEPENDENT_CODE ON)
