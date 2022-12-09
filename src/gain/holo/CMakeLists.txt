cmake_minimum_required(VERSION 3.21)

add_library(gain_holo
  holo.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/holo.hpp
)
target_link_libraries(gain_holo PUBLIC autd3_common)

set_target_properties(gain_holo
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(gain_holo PROPERTIES FOLDER "gain")
set_target_properties(gain_holo PROPERTIES POSITION_INDEPENDENT_CODE ON)

add_subdirectory(eigen)
add_subdirectory(cuda)
add_subdirectory(blas)
add_subdirectory(arrayfire)
