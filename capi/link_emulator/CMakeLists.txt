cmake_minimum_required(VERSION 3.16)

add_library(autd3capi-link-emulator SHARED
  c_api.cpp
  emulator_link.h
  ../base/header.hpp
  ../base/wrapper_link.hpp
)

target_link_libraries(autd3capi-link-emulator link_emulator autd3)
if(WIN32)
  target_link_libraries(autd3capi-link-emulator ws2_32 winmm)
elseif(APPLE)
else()
  target_link_libraries(autd3capi-link-emulator -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-link-emulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-link-emulator PROPERTIES FOLDER "capi/link")
set_target_properties(autd3capi-link-emulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
