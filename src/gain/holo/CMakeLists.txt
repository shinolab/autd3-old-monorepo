cmake_minimum_required(VERSION 3.16)

add_library(gain_holo
  holo.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/gain/holo.hpp
)
target_include_directories(gain_holo PUBLIC ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH})

set_target_properties(gain_holo
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

if(WIN32)
  set_target_properties(gain_holo PROPERTIES FOLDER "gain")
else()
  set_target_properties(gain_holo PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()

add_subdirectory(eigen)
target_link_libraries(gain_holo INTERFACE backend_eigen)

if(BUILD_BACKEND_CUDA)
  find_package(CUDA REQUIRED)
  find_package(CUDAToolkit REQUIRED)
  if(CUDA_FOUND AND CUDAToolkit_FOUND)
    message("CUDA version: " ${CUDA_VERSION})
    message("CUDA toolkit version: " ${CUDAToolkit_VERSION})
    add_subdirectory(cuda)
    target_link_libraries(gain_holo INTERFACE backend_cuda)
  endif()
endif()
