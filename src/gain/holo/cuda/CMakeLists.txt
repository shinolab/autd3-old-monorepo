cmake_minimum_required(VERSION 3.16)

set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} --expt-relaxed-constexpr -Xcudafe "--diag_suppress=esa_on_defaulted_function_ignored")
if(WIN32 AND (MSVC_TOOLSET_VERSION VERSION_EQUAL 143))
  set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -allow-unsupported-compiler)
endif()

cuda_add_library(backend_cuda
  backend_cuda.cpp
  kernel.h
  kernel.cu
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/backend_cuda.hpp
)

cuda_add_cublas_to_target(backend_cuda)
target_include_directories(backend_cuda PRIVATE ${PROJECT_SOURCE_DIR}/src/gain/holo ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH})
target_include_directories(backend_cuda INTERFACE ${CUDA_INCLUDE_DIRS})
target_link_libraries(backend_cuda ${CUDA_cusolver_LIBRARY})

set_target_properties(backend_cuda
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(backend_cuda
  PROPERTIES
  CUDA_SEPARABLE_COMPILATION ON
)

if(WIN32)
  set_target_properties(backend_cuda PROPERTIES FOLDER "gain/backend")
else()
  set_target_properties(backend_cuda PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
