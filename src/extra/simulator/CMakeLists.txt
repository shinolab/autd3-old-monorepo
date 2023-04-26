cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_SIMULATOR)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/glm.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/json.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/stb.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/tinycolormap.cmake)

add_library(autd3_simulator simulator.cpp)
add_library(autd3::extra::simulator ALIAS autd3_simulator)
target_include_directories(autd3_simulator PRIVATE ${stb_SOURCE_DIR} ${Vulkan_INCLUDE_DIRS})
target_link_libraries(autd3_simulator PRIVATE autd3::common autd3::extra::vk_helper autd3::extra::imgui_vulkan glm::glm glfw tinycolormap)
target_link_libraries(autd3_simulator PRIVATE nlohmann_json::nlohmann_json)

set_target_properties(autd3_simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_simulator PROPERTIES FOLDER "extra")
set_target_properties(autd3_simulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
