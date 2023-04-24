cmake_minimum_required(VERSION 3.21)

add_library(autd3_backend_eigen backend_eigen.cpp)
add_library(autd3::backend::eigen ALIAS autd3_backend_eigen)
target_link_libraries(autd3_backend_eigen PRIVATE autd3::common)

set_target_properties(autd3_backend_eigen
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_backend_eigen PROPERTIES FOLDER "gain/backend")
set_target_properties(autd3_backend_eigen PROPERTIES POSITION_INDEPENDENT_CODE ON)
