cmake_minimum_required(VERSION 3.16)

add_library(simulator
  sound_sources.hpp
  simulator.cpp
)
target_include_directories(simulator PRIVATE ${PROJECT_SOURCE_DIR}/include ${Vulkan_INCLUDE_DIRS} ${STB_INCLUDE_PATH})
target_link_libraries(simulator PRIVATE autd3_common vk_helper glm::glm glfw imgui_vulkan)

if(WIN32)
  target_link_options(simulator INTERFACE /NODEFAULTLIB:libcmt.lib /NODEFAULTLIB:libcmtd.lib /NODEFAULTLIB:msvcrtd.lib)
endif()

set_target_properties(simulator PROPERTIES FOLDER "extra")
