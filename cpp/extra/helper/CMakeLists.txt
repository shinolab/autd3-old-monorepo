cmake_minimum_required(VERSION 3.21)

if(NOT (BUILD_GEOMETRY_VIEWER OR BUILD_SIMULATOR))
  return()
endif()

find_package(Vulkan)
include(${PROJECT_SOURCE_DIR}/cmakes/glfw.cmake)

add_library(autd3_vk_helper INTERFACE)
add_library(autd3::extra::vk_helper ALIAS autd3_vk_helper)
target_include_directories(autd3_vk_helper INTERFACE ${Vulkan_INCLUDE_DIRS} ${PROJECT_SOURCE_DIR}/src/extra/helper)
target_link_libraries(autd3_vk_helper INTERFACE ${Vulkan_LIBRARIES} glfw)
