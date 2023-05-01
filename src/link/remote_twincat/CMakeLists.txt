cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_REMOTE_TWINCAT)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/ads.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/boost.cmake)

add_library(autd3_link_remote_twincat remote_twincat.cpp)
add_library(autd3::link::remote_twincat ALIAS autd3_link_remote_twincat)

target_include_directories(autd3_link_remote_twincat PRIVATE ${Boost_INCLUDE_DIRS})
target_link_libraries(autd3_link_remote_twincat PRIVATE autd3::common ads)

if(WIN32)
  target_compile_definitions(autd3_link_remote_twincat PRIVATE NOMINMAX)
endif()
enable_cxx_compiler_flag_if_supported(autd3_link_remote_twincat PRIVATE -Wno-mismatched-new-delete)
target_compile_definitions(autd3_link_remote_twincat PRIVATE -DCONFIG_DEFAULT_LOGLEVEL=1)

set_target_properties(autd3_link_remote_twincat
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_link_remote_twincat PROPERTIES FOLDER "link")
set_target_properties(autd3_link_remote_twincat PROPERTIES POSITION_INDEPENDENT_CODE ON)
