cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_TWINCAT)
  return()
endif()

add_library(autd3_link_twincat twincat.cpp)
add_library(autd3::link::twincat ALIAS autd3_link_twincat)

target_link_libraries(autd3_link_twincat PRIVATE autd3::common)

set_target_properties(autd3_link_twincat
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_link_twincat PROPERTIES FOLDER "link")
set_target_properties(autd3_link_twincat PROPERTIES POSITION_INDEPENDENT_CODE ON)
