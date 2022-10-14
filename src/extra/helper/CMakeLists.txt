add_library(vk_helper INTERFACE)
target_include_directories(vk_helper INTERFACE ${Vulkan_INCLUDE_DIRS} ${PROJECT_SOURCE_DIR}/src/extra/helper)
target_link_libraries(vk_helper INTERFACE ${Vulkan_LIBRARIES} glfw)
