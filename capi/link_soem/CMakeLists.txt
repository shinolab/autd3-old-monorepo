cmake_minimum_required(VERSION 3.21)

add_library(autd3capi-link-soem SHARED
  c_api.cpp
  soem_link.h
  ./custom_sink.hpp
  ../base/header.hpp
  ../base/wrapper_link.hpp
)

target_link_libraries(autd3capi-link-soem link_soem)
if(WIN32)
  target_link_libraries(autd3capi-link-soem winmm ws2_32)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-link-soem -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-link-soem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-link-soem PROPERTIES FOLDER "capi/link")
set_target_properties(autd3capi-link-soem PROPERTIES POSITION_INDEPENDENT_CODE ON)
