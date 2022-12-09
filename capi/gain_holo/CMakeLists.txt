cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_GAIN_HOLO)
  return()
endif()

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

set_target_properties(autd3capi-gain-holo PROPERTIES FOLDER "capi/gain")
set_target_properties(autd3capi-gain-holo PROPERTIES POSITION_INDEPENDENT_CODE ON)

add_subdirectory(blas)
add_subdirectory(cuda)
