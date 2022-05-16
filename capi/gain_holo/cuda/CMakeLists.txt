cmake_minimum_required(VERSION 3.16)

add_library(autd3capi-backend-cuda SHARED
  c_api.cpp
  cuda_backend.h
  ../../base/header.h
  ../../base/wrapper.hpp
)

target_include_directories(autd3capi-backend-cuda PRIVATE ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH})
target_link_libraries(autd3capi-backend-cuda backend_cuda)
if(WIN32)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-backend-cuda -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-backend-cuda
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

if(WIN32)
  set_target_properties(autd3capi-backend-cuda PROPERTIES FOLDER "capi/gain/backend")
else()
  set_target_properties(autd3capi-backend-cuda PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
