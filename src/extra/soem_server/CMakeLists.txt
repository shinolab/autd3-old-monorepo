cmake_minimum_required(VERSION 3.21)

add_library(soem_server
    soem_server.cpp
)
target_link_libraries(soem_server autd3_common)

set_target_properties(soem_server
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(soem_server PROPERTIES FOLDER "extra")
set_target_properties(soem_server PROPERTIES POSITION_INDEPENDENT_CODE ON)
