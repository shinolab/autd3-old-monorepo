cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_SIMULATOR)
  return()
endif()

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
  ${PROJECT_SOURCE_DIR}/include/autd3/extra/simulator.hpp
  simulator.cpp
)
target_include_directories(simulator PRIVATE ${PROJECT_SOURCE_DIR}/include ${STB_INCLUDE_PATH} ${Vulkan_INCLUDE_DIRS} ${PROJECT_SOURCE_DIR}/src/extra/simulator ${PROJECT_SOURCE_DIR}/src/extra/tools)
target_link_libraries(simulator PRIVATE autd3_common vk_helper glm::glm glfw imgui_vulkan smem)
target_link_libraries(simulator PUBLIC nlohmann_json tinycolormap)

set_target_properties(simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(simulator PROPERTIES FOLDER "extra")
set_target_properties(simulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
