cmake_minimum_required(VERSION 3.16)

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

if(WIN32)
  set_target_properties(gain_holo PROPERTIES FOLDER "gain")
else()
  set_target_properties(gain_holo PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()

add_subdirectory(eigen)
target_link_libraries(gain_holo INTERFACE backend_eigen)

if(BUILD_BACKEND_CUDA)
  add_subdirectory(cuda)
  target_link_libraries(gain_holo INTERFACE backend_cuda)
endif()

if(BUILD_BACKEND_BLAS)
  add_subdirectory(blas)
  target_link_libraries(gain_holo INTERFACE backend_blas)
endif()
