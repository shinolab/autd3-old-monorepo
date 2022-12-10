cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_BUNDLE)
  return()
endif()

add_library(autd3capi-link-bundle SHARED
  c_api.cpp
  bundle_link.h
  ../base/header.hpp
  ../base/wrapper_link.hpp
)
 
target_link_libraries(autd3capi-link-bundle link_bundle)

set_target_properties(autd3capi-link-bundle
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-link-bundle PROPERTIES FOLDER "capi/link")
