cmake_minimum_required(VERSION 3.16)

add_library(autd3capi-backend-blas SHARED
  c_api.cpp
  blas_backend.h
  ../../base/header.h
  ../../base/wrapper.hpp
)

target_include_directories(autd3capi-backend-blas PRIVATE ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH})
target_link_libraries(autd3capi-backend-blas backend_blas)
if(WIN32)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-backend-blas -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-backend-blas
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

if(WIN32)
  set_target_properties(autd3capi-backend-blas PROPERTIES FOLDER "capi/gain/backend")
else()
  set_target_properties(autd3capi-backend-blas PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
