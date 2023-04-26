cmake_minimum_required(VERSION 3.21)

if(NOT (BUILD_GEOMETRY_VIEWER OR BUILD_SIMULATOR))
  return()
endif()

find_package(Vulkan)
include(${PROJECT_SOURCE_DIR}/cmakes/imgui.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/glfw.cmake)

add_library(autd3_imgui_vulkan ${imgui_SOURCE_DIR}/imgui.cpp ${imgui_SOURCE_DIR}/imgui_draw.cpp ${imgui_SOURCE_DIR}/imgui_demo.cpp ${imgui_SOURCE_DIR}/imgui_widgets.cpp ${imgui_SOURCE_DIR}/imgui_tables.cpp ${imgui_SOURCE_DIR}/backends/imgui_impl_glfw.cpp ${imgui_SOURCE_DIR}/backends/imgui_impl_vulkan.cpp)
add_library(autd3::extra::imgui_vulkan ALIAS autd3_imgui_vulkan)
target_include_directories(autd3_imgui_vulkan PUBLIC ${imgui_SOURCE_DIR} ${imgui_SOURCE_DIR}/backends)
target_include_directories(autd3_imgui_vulkan PRIVATE ${Vulkan_INCLUDE_DIRS})
target_link_libraries(autd3_imgui_vulkan PRIVATE glfw)

set_target_properties(autd3_imgui_vulkan PROPERTIES FOLDER "3rdparty")
set_target_properties(autd3_imgui_vulkan PROPERTIES POSITION_INDEPENDENT_CODE ON)
