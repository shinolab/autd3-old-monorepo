cmake_minimum_required(VERSION 3.16)

add_library(autd3capi-link-remote-twincat SHARED
  c_api.cpp
  remote_twincat_link.h
  ../base/header.hpp
  ../base/wrapper_link.hpp
)

target_link_libraries(autd3capi-link-remote-twincat link_remote_twincat)
if(WIN32)
  target_link_libraries(autd3capi-link-remote-twincat ws2_32)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-link-remote-twincat -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-link-remote-twincat
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

if(WIN32)
  set_target_properties(autd3capi-link-remote-twincat PROPERTIES FOLDER "capi/link")
else()
  set_target_properties(autd3capi-link-remote-twincat PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
