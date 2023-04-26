cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_BACKEND_BLAS)
  return()
endif()

add_library(autd3capi-backend-blas SHARED c_api.cpp)

target_link_libraries(autd3capi-backend-blas PRIVATE autd3::common autd3::backend::blas)
if(WIN32)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-backend-blas PRIVATE -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-backend-blas
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-backend-blas PROPERTIES FOLDER "capi/gain/backend")
set_target_properties(autd3capi-backend-blas PROPERTIES POSITION_INDEPENDENT_CODE ON)
