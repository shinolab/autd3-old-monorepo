cmake_minimum_required(VERSION 3.17)

if(NOT BUILD_BACKEND_ARRAYFIRE)
  return()
endif()

find_package(ArrayFire REQUIRED)
message("ArrayFire version: " ${ArrayFire_VERSION})

add_library(autd3_backend_arrayfire backend_arrayfire.cpp)
add_library(autd3::backend::arrayfire ALIAS autd3_backend_arrayfire)
target_link_libraries(autd3_backend_arrayfire PRIVATE autd3::common)
target_include_directories(autd3_backend_arrayfire PRIVATE ${ArrayFire_INCLUDE_DIRS})
target_link_libraries(autd3_backend_arrayfire PRIVATE ${ArrayFire_Unified_LIBRARIES})

set_target_properties(autd3_backend_arrayfire
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_backend_arrayfire PROPERTIES FOLDER "gain/backend")
set_target_properties(autd3_backend_arrayfire PROPERTIES POSITION_INDEPENDENT_CODE ON)
