cmake_minimum_required(VERSION 3.21)

add_library(autd3
  link/log/log.cpp
  link/debug/debug.cpp
)
add_library(autd3::autd3 ALIAS autd3)

target_link_libraries(autd3 PUBLIC autd3::common)

if(WIN32)
  target_link_libraries(autd3 INTERFACE winmm)
endif()

set_target_properties(autd3
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3 PROPERTIES POSITION_INDEPENDENT_CODE ON)

add_subdirectory(link)
add_subdirectory(gain)
add_subdirectory(modulation)
add_subdirectory(extra)
