cmake_minimum_required(VERSION 3.21)

if(NOT (BUILD_GEOMETRY_VIEWER OR BUILD_SIMULATOR))
  return()
endif()

add_library(vk_helper INTERFACE)
target_include_directories(vk_helper INTERFACE ${Vulkan_INCLUDE_DIRS} ${PROJECT_SOURCE_DIR}/src/extra/helper)
target_link_libraries(vk_helper INTERFACE ${Vulkan_LIBRARIES} glfw)
