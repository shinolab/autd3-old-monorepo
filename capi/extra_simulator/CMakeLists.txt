cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_SIMULATOR)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/json.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/tinycolormap.cmake)

add_library(autd3capi-extra-simulator SHARED c_api.cpp)
 
target_link_libraries(autd3capi-extra-simulator PRIVATE autd3::extra::simulator autd3::common nlohmann_json::nlohmann_json tinycolormap)

set_target_properties(autd3capi-extra-simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi-extra-simulator PROPERTIES FOLDER "capi/extra")
