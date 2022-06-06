cmake_minimum_required(VERSION 3.16)

add_library(autd3capi-gain-holo SHARED
  c_api.cpp
  holo_gain.h
  ../base/header.hpp
  ../base/wrapper.hpp
)

target_include_directories(autd3capi-gain-holo PRIVATE ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH})
target_link_libraries(autd3capi-gain-holo gain_holo backend_eigen)
if(WIN32)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-gain-holo -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-gain-holo
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

if(WIN32)
  set_target_properties(autd3capi-gain-holo PROPERTIES FOLDER "capi/gain")
else()
  set_target_properties(autd3capi-gain-holo PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()

if(BUILD_BACKEND_BLAS)
  add_subdirectory(blas)
endif()

if(BUILD_BACKEND_CUDA)
  add_subdirectory(cuda)
endif()
