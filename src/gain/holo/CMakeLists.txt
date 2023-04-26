cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_GAIN_HOLO)
  return()
endif()

add_library(autd3_gain_holo holo.cpp)
add_library(autd3::gain::holo ALIAS autd3_gain_holo)
target_link_libraries(autd3_gain_holo PRIVATE autd3::common)
if(HOLO_PARALLEL_FOR)
  target_compile_definitions(autd3_gain_holo PRIVATE HOLO_PARALLEL_FOR)
endif()

set_target_properties(autd3_gain_holo
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_gain_holo PROPERTIES FOLDER "gain")
set_target_properties(autd3_gain_holo PROPERTIES POSITION_INDEPENDENT_CODE ON)

add_subdirectory(eigen)
add_subdirectory(cuda)
add_subdirectory(blas)
add_subdirectory(arrayfire)
