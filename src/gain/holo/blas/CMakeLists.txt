cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_BACKEND_BLAS)
  return()
endif()

set(CMAKE_PREFIX_PATH "${CMAKE_PREFIX_PATH};${BLAS_LIB_DIR}")
set(BLA_STATIC ON)
find_package(BLAS REQUIRED)

add_library(autd3_backend_blas backend_blas.cpp)
add_library(autd3::backend::blas ALIAS autd3_backend_blas)
target_link_libraries(autd3_backend_blas PRIVATE autd3::common)
target_include_directories(autd3_backend_blas PRIVATE ${BLAS_INCLUDE_DIR})

if(USE_MKL)
  target_compile_definitions(autd3_backend_blas PRIVATE USE_BLAS_MKL)
endif()

target_link_directories(autd3_backend_blas PRIVATE ${BLAS_DEPEND_LIB_DIR})
target_link_libraries(autd3_backend_blas PRIVATE ${BLAS_LIBRARIES})
if(NOT WIN32)
  target_link_libraries(autd3_backend_blas INTERFACE gfortran)
endif()

set_target_properties(autd3_backend_blas
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_backend_blas PROPERTIES FOLDER "gain/backend")
set_target_properties(autd3_backend_blas PROPERTIES POSITION_INDEPENDENT_CODE ON)
