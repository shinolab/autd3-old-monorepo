cmake_minimum_required(VERSION 3.17)

find_package(ArrayFire REQUIRED)
message("ArrayFire version: " ${ArrayFire_VERSION})

add_library(backend_arrayfire
  backend_arrayfire.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/backend_arrayfire.hpp
)
target_link_libraries(backend_arrayfire PUBLIC autd3_common)
target_include_directories(backend_arrayfire PRIVATE ${ArrayFire_INCLUDE_DIRS})
target_link_libraries(backend_arrayfire PUBLIC ${ArrayFire_Unified_LIBRARIES})

set_target_properties(backend_arrayfire
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(backend_arrayfire PROPERTIES FOLDER "gain/backend")
set_target_properties(backend_arrayfire PROPERTIES POSITION_INDEPENDENT_CODE ON)
