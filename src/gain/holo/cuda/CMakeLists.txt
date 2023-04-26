cmake_minimum_required(VERSION 3.17)

if(NOT BUILD_BACKEND_CUDA)
  return()
endif()

find_package(CUDA REQUIRED)
find_package(CUDAToolkit REQUIRED)
message("CUDA version: " ${CUDA_VERSION})
message("CUDA toolkit version: " ${CUDAToolkit_VERSION})

set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} --expt-relaxed-constexpr -Xcudafe "--diag_suppress=esa_on_defaulted_function_ignored")
if(WIN32)
  if(MSVC_TOOLSET_VERSION VERSION_EQUAL 143)
    set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -allow-unsupported-compiler)
  endif()
  if(BUILD_WITH_STATIC_CRT)
    set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -Xcompiler /MT)
  else()
    set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -Xcompiler /MD)
  endif()
else()
  set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -Xcompiler -fPIC)
endif()

cuda_add_library(autd3_backend_cuda
  backend_cuda.cpp
  kernel.h
  kernel.cu
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/backend_cuda.hpp
)
add_library(autd3::backend::cuda ALIAS autd3_backend_cuda)

cuda_add_cublas_to_target(autd3_backend_cuda)
target_link_libraries(autd3_backend_cuda autd3::common)
target_include_directories(autd3_backend_cuda PRIVATE ${PROJECT_SOURCE_DIR}/src/gain/holo)
target_link_libraries(autd3_backend_cuda ${CUDA_cusolver_LIBRARY})

set_target_properties(autd3_backend_cuda
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_backend_cuda
  PROPERTIES
  CUDA_SEPARABLE_COMPILATION ON
)

set_target_properties(autd3_backend_cuda PROPERTIES FOLDER "gain/backend")
set_target_properties(autd3_backend_cuda PROPERTIES POSITION_INDEPENDENT_CODE ON)
