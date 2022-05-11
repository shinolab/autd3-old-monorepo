cmake_minimum_required(VERSION 3.16)

add_library(link_emulator
  emulator.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/emulator.hpp
)
target_include_directories(link_emulator PUBLIC ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH})
target_link_libraries(link_emulator PRIVATE GSL)

if(WIN32)
  target_link_libraries(link_emulator ${AUTD_LINK_LIBRARIES_KEYWORD} ws2_32)
endif()

set_target_properties(link_emulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

if(WIN32)
  set_target_properties(link_emulator PROPERTIES FOLDER "link")
else()
  set_target_properties(link_emulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
