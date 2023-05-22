cmake_minimum_required(VERSION 3.21)

add_library(autd3_model autd3_model.cpp)
add_library(autd3::extra::model ALIAS autd3_model)

set_target_properties(autd3_model
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_model PROPERTIES FOLDER "extra")
set_target_properties(autd3_model PROPERTIES POSITION_INDEPENDENT_CODE ON)
