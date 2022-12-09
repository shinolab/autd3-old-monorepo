cmake_minimum_required(VERSION 3.21)

if(NOT (BUILD_LINK_TWINCAT AND WIN32))
  return()
endif()

add_library(autd3capi-link-twincat SHARED
  c_api.cpp
  twincat_link.h
  ../base/header.hpp
  ../base/wrapper_link.hpp
)
 
target_link_libraries(autd3capi-link-twincat link_twincat)

set_target_properties(autd3capi-link-twincat
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-link-twincat PROPERTIES FOLDER "capi/link")
