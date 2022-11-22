cmake_minimum_required(VERSION 3.21)

add_library(autd3
  ${PROJECT_SOURCE_DIR}/include/autd3/controller.hpp
  controller.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/soft_stm.hpp
  soft_stm.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/modulation/lpf.hpp
  modulation/lpf/lpf.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/driver/v2_2/driver.hpp
  driver/v2_2/driver.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/driver/v2_3/driver.hpp
  driver/v2_3/driver.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/driver/v2_4/driver.hpp
  driver/v2_4/driver.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/driver/v2_5/driver.hpp
  driver/v2_5/driver.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/driver/v2_6/driver.hpp
  driver/v2_6/driver.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/core/mode.hpp
  core/mode.cpp
)
target_link_libraries(autd3 PUBLIC autd3_common)

set_target_properties(autd3
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3 PROPERTIES POSITION_INDEPENDENT_CODE ON)
