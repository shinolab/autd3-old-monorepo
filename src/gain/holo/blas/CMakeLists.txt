cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_BACKEND_BLAS)
  return()
endif()

set(CMAKE_PREFIX_PATH "${CMAKE_PREFIX_PATH};${BLAS_LIB_DIR}")
set(BLA_STATIC ON)
find_package(BLAS REQUIRED)

add_library(backend_blas
  backend_blas.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/backend.hpp
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/backend_blas.hpp
)
target_link_libraries(backend_blas PUBLIC autd3_common)
target_include_directories(backend_blas PRIVATE ${BLAS_INCLUDE_DIR} ${PROJECT_SOURCE_DIR}/src/gain/holo)

if(USE_MKL)
  target_compile_definitions(backend_blas PRIVATE USE_BLAS_MKL)
endif()

target_link_directories(backend_blas PUBLIC ${BLAS_DEPEND_LIB_DIR})
target_link_libraries(backend_blas PUBLIC ${BLAS_LIBRARIES})
if(NOT WIN32)
  target_link_libraries(backend_blas INTERFACE gfortran)
endif()

set_target_properties(backend_blas
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(backend_blas PROPERTIES FOLDER "gain/backend")
set_target_properties(backend_blas PROPERTIES POSITION_INDEPENDENT_CODE ON)
