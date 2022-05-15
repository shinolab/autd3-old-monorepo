cmake_minimum_required(VERSION 3.16)

add_library(backend_eigen
  backend_eigen.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/backend.hpp
)
target_include_directories(backend_eigen PRIVATE ${PROJECT_SOURCE_DIR}/src/gain/holo ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH})

set_target_properties(backend_eigen
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

if(WIN32)
  set_target_properties(backend_eigen PROPERTIES FOLDER "gain/backend")
else()
  set_target_properties(backend_eigen PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
