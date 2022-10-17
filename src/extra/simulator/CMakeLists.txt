cmake_minimum_required(VERSION 3.21)

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
target_include_directories(simulator PRIVATE ${PROJECT_SOURCE_DIR}/include ${STB_INCLUDE_PATH} ${Vulkan_INCLUDE_DIRS} ${PROJECT_SOURCE_DIR}/src/extra/simulator/shaders ${PROJECT_SOURCE_DIR}/src/extra/simulator/textures)
target_link_libraries(simulator PRIVATE autd3_common vk_helper glm::glm glfw imgui_vulkan)
target_link_libraries(simulator PUBLIC nlohmann_json tinycolormap)
if (WIN32)
  target_link_libraries(simulator PUBLIC ws2_32)
endif()

set_target_properties(simulator
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(simulator PROPERTIES FOLDER "extra")
set_target_properties(simulator PROPERTIES POSITION_INDEPENDENT_CODE ON)
