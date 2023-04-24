cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_GEOMETRY_VIEWER)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/json.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/fx-gltf.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/glm.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/stb.cmake)

add_subdirectory(models)

add_library(autd3_geometry_viewer geometry_viewer.cpp)
add_library(autd3::extra::geometry_viewer ALIAS autd3_geometry_viewer)
target_include_directories(autd3_geometry_viewer PRIVATE ${stb_SOURCE_DIR})
target_link_libraries(autd3_geometry_viewer PRIVATE autd3::common autd3::extra::vk_helper autd3::extra::imgui_vulkan autd3::extra::model glm::glm glfw nlohmann_json::nlohmann_json fx-gltf::fx-gltf)

set_target_properties(autd3_geometry_viewer
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_geometry_viewer PROPERTIES FOLDER "extra")
set_target_properties(autd3_geometry_viewer PROPERTIES POSITION_INDEPENDENT_CODE ON)
