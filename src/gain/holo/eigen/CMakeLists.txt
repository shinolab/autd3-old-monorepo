cmake_minimum_required(VERSION 3.16)

add_library(backend_eigen
  backend_eigen.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/backend.hpp
)
target_link_libraries(backend_eigen PUBLIC autd3_common)
target_include_directories(backend_eigen PRIVATE ${PROJECT_SOURCE_DIR}/src/gain/holo)

set_target_properties(backend_eigen
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(backend_eigen PROPERTIES FOLDER "gain/backend")
set_target_properties(backend_eigen PROPERTIES POSITION_INDEPENDENT_CODE ON)
