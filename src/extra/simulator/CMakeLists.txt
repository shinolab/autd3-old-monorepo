cmake_minimum_required(VERSION 3.16)

add_library(simulator
  color.hpp
  coloring.hpp
  update_flag.hpp
  sound_sources.hpp
  vulkan_imgui.hpp
  vulkan_renderer.hpp
  trans_viewer.hpp
  slice_viewer.hpp
  field_compute.hpp
  ${PROJECT_SOURCE_DIR}/include/autd3/extra/simulator/simulator.hpp
  simulator.cpp
)
target_include_directories(simulator PRIVATE ${PROJECT_SOURCE_DIR}/include ${Vulkan_INCLUDE_DIRS} ${STB_INCLUDE_PATH})
target_link_libraries(simulator PRIVATE autd3_common vk_helper glm::glm glfw imgui_vulkan)

if(WIN32)
  target_link_options(simulator INTERFACE /NODEFAULTLIB:libcmt.lib /NODEFAULTLIB:libcmtd.lib /NODEFAULTLIB:msvcrtd.lib)
endif()

set_target_properties(simulator PROPERTIES FOLDER "extra")
