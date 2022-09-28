cmake_minimum_required(VERSION 3.16)

if(NOT WIN32)
  find_package(Threads REQUIRED)
endif()

file(GLOB hpp_files tests/*.hpp runner.hpp)

add_library(autd3_example_common INTERFACE)
target_link_libraries(autd3_example_common INTERFACE autd3)
if(BUILD_GAIN_HOLO)
  target_link_libraries(autd3_example_common INTERFACE gain_holo)
  target_compile_definitions(autd3_example_common INTERFACE BUILD_GAIN_HOLO)
endif()
if(BUILD_MODULATION_AUDIO_FILE)
  target_link_libraries(autd3_example_common INTERFACE modulation_audio_file)
  target_compile_definitions(autd3_example_common INTERFACE BUILD_MODULATION_AUDIO_FILE)
  target_compile_definitions(autd3_example_common INTERFACE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/examples/resources")
endif()
if(NOT WIN32)
  target_link_libraries(autd3_example_common INTERFACE ${CMAKE_THREAD_LIBS_INIT})
endif()

if(BUILD_LINK_SOEM)
  add_executable(example_soem soem.cpp ${hpp_files})
  target_link_libraries(example_soem autd3_example_common link_soem)
  set_target_properties(example_soem PROPERTIES FOLDER "example")
endif()

if(BUILD_LINK_TWINCAT)
  if(WIN32)
    add_executable(example_twincat twincat.cpp ${hpp_files})
    target_link_libraries(example_twincat autd3_example_common link_twincat)
    set_target_properties(example_twincat PROPERTIES FOLDER "example")
  endif()
endif()

if(BUILD_LINK_REMOTE_TWINCAT)
  add_executable(example_remote_twincat remote_twincat.cpp ${hpp_files})
  target_link_libraries(example_remote_twincat autd3_example_common link_remote_twincat)
  set_target_properties(example_remote_twincat PROPERTIES FOLDER "example")
endif()

if(BUILD_LINK_EMULATOR)
  add_executable(example_emulator emulator.cpp ${hpp_files})
  target_link_libraries(example_emulator autd3_example_common link_emulator)
  set_target_properties(example_emulator PROPERTIES FOLDER "example")
endif()

if(BUILD_LINK_DEBUG)
  add_executable(example_debug debug.cpp ${hpp_files})
  target_link_libraries(example_debug autd3_example_common link_debug)
  set_target_properties(example_debug PROPERTIES FOLDER "example")
endif()

if(BUILD_EXAMPLES_EXTRA)
  add_executable(freqency_configuration freq_config.cpp)
  target_link_libraries(freqency_configuration autd3_example_common link_debug)
  set_target_properties(freqency_configuration PROPERTIES FOLDER "example")
endif()

if(BUILD_GEOMETRY_VIEWER)
  add_executable(geometry_viewer_example geometry_viewer_example.cpp)
  target_link_libraries(geometry_viewer_example autd3 geometry-viewer)
  
  add_custom_command(TARGET geometry_viewer_example PRE_BUILD COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_BINARY_DIR}/shaders)
  add_custom_command(TARGET geometry_viewer_example PRE_BUILD COMMAND ${Vulkan_GLSLC_EXECUTABLE} ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/shaders/simple.frag -o ${CMAKE_CURRENT_BINARY_DIR}/shaders/frag.spv)
  add_custom_command(TARGET geometry_viewer_example PRE_BUILD COMMAND ${Vulkan_GLSLC_EXECUTABLE} ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/shaders/simple.vert -o ${CMAKE_CURRENT_BINARY_DIR}/shaders/vert.spv)

  add_custom_command(TARGET geometry_viewer_example PRE_BUILD COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_BINARY_DIR}/fonts)
  add_custom_command(TARGET geometry_viewer_example
      COMMAND ${CMAKE_COMMAND} -E copy_if_different
      ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/fonts/NotoSans-Regular.ttf
      ${CMAKE_CURRENT_BINARY_DIR}/fonts/NotoSans-Regular.ttf)

  add_custom_command(TARGET geometry_viewer_example PRE_BUILD COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_BINARY_DIR}/models)
  add_custom_command(TARGET geometry_viewer_example
      COMMAND ${CMAKE_COMMAND} -E copy_if_different
      ${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/models/AUTD.glb
      ${CMAKE_CURRENT_BINARY_DIR}/models/AUTD.glb)
  
  set_target_properties(geometry_viewer_example PROPERTIES FOLDER "example")
endif()
