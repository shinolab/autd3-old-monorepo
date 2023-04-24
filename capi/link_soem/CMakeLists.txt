cmake_minimum_required(VERSION 3.21)

if (NOT BUILD_LINK_SOEM)
  return()
endif()

add_library(autd3capi-link-soem SHARED c_api.cpp) 

target_link_libraries(autd3capi-link-soem PRIVATE autd3::common autd3::link::soem)
if(WIN32)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-link-soem PRIVATE -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-link-soem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-link-soem PROPERTIES FOLDER "capi/link")
set_target_properties(autd3capi-link-soem PROPERTIES POSITION_INDEPENDENT_CODE ON)
