cmake_minimum_required(VERSION 3.16)

add_library(link_twincat
  twincat.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/twincat.hpp
)
target_include_directories(link_twincat PUBLIC ${PROJECT_SOURCE_DIR}/include)
target_link_libraries(link_twincat PRIVATE GSL)

set_target_properties(link_twincat
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

if(WIN32)
  set_target_properties(link_twincat PROPERTIES FOLDER "link")
else()
  set_target_properties(link_twincat PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
