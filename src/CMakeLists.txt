cmake_minimum_required(VERSION 3.21)

file(GLOB HEADER_FILES ${PROJECT_SOURCE_DIR}/include/autd3/*.hpp)
file(GLOB_RECURSE CORE_HEADER_FILES ${PROJECT_SOURCE_DIR}/include/autd3/core/*.hpp)
file(GLOB_RECURSE DRIVER_HEADER_FILES ${PROJECT_SOURCE_DIR}/include/autd3/driver/*.hpp)
file(GLOB_RECURSE EXTRA_HEADER_FILES ${PROJECT_SOURCE_DIR}/include/autd3/extra/*.hpp)
file(GLOB_RECURSE MODULATION_HEADER_FILES ${PROJECT_SOURCE_DIR}/include/autd3/modulation/*.hpp)

add_library(autd3
  controller.cpp
  soft_stm.cpp
  modulation/lpf/lpf.cpp
  driver/v2_2/driver.cpp
  driver/v2_3/driver.cpp
  driver/v2_4/driver.cpp
  driver/v2_5/driver.cpp
  driver/v2_6/driver.cpp
  driver/v2_7/driver.cpp
  core/mode.cpp
  ${HEADER_FILES}
  ${CORE_HEADER_FILES}
  ${DRIVER_HEADER_FILES}
  ${EXTRA_HEADER_FILES}
  ${MODULATION_HEADER_FILES}
)
target_link_libraries(autd3 PUBLIC autd3_common)

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