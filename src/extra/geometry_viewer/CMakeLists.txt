cmake_minimum_required(VERSION 3.21)

add_library(geometry_viewer
    geometry_viewer.cpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/vulkan_handler.hpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/vulkan_renderer.hpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/vulkan_imgui.hpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/model.hpp
)
target_include_directories(geometry_viewer PRIVATE ${PROJECT_SOURCE_DIR}/include ${STB_INCLUDE_PATH} ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/shaders)
target_link_libraries(geometry_viewer PRIVATE autd3_common vk_helper glm::glm glfw nlohmann_json fx-gltf imgui_vulkan)

set_target_properties(geometry_viewer
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(geometry_viewer PROPERTIES FOLDER "extra")
set_target_properties(geometry_viewer PROPERTIES POSITION_INDEPENDENT_CODE ON)
