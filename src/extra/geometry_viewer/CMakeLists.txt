cmake_minimum_required(VERSION 3.16)

add_library(geometry-viewer
    geometry_viewer.cpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/window_handler.hpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/vulkan_handler.hpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/vulkan_renderer.hpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/vulkan_imgui.hpp
    ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/model.hpp
)
target_include_directories(geometry-viewer PRIVATE ${PROJECT_SOURCE_DIR}/include ${Vulkan_INCLUDE_DIRS} ${STB_INCLUDE_PATH})
target_link_libraries(geometry-viewer PRIVATE autd3_common ${Vulkan_LIBRARIES} glm::glm glfw nlohmann_json fx-gltf imgui-vulkan)

if(WIN32)
  target_link_options(geometry-viewer INTERFACE /ignore:4044)
  target_link_options(geometry-viewer INTERFACE /NODEFAULTLIB:libcmt.lib /NODEFAULTLIB:libcmtd.lib /NODEFAULTLIB:msvcrtd.lib)
endif()

set_target_properties(geometry-viewer PROPERTIES FOLDER "extra")
