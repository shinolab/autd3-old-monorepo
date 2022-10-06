cmake_minimum_required(VERSION 3.16)

find_package(Threads REQUIRED)

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

if(BUILD_LINK_DEBUG)
  add_executable(example_debug debug.cpp ${hpp_files})
  target_link_libraries(example_debug autd3_example_common link_debug)
  set_target_properties(example_debug PROPERTIES FOLDER "example")
endif()

if(BUILD_EXAMPLES_EXTRA)
  add_executable(example_freq_config freq_config.cpp)
  target_link_libraries(example_freq_config autd3_example_common link_debug)
  set_target_properties(example_freq_config PROPERTIES FOLDER "example")
endif()

if(BUILD_GEOMETRY_VIEWER)
  add_executable(example_geometry_viewer geometry_viewer.cpp)
  target_link_libraries(example_geometry_viewer autd3 geometry_viewer)
  
  target_compile_definitions(example_geometry_viewer PRIVATE AUTD3_GEOMETRY_VIEWER_MODEL_PATH="${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/models/AUTD.glb")
  target_compile_definitions(example_geometry_viewer PRIVATE AUTD3_GEOMETRY_VIEWER_FONT_PATH="${PROJECT_SOURCE_DIR}/examples/fonts/NotoSans-Regular.ttf")

  set_target_properties(example_geometry_viewer PROPERTIES FOLDER "example")
endif()

if(BUILD_SIMULATOR)
  add_executable(example_simulator simulator.cpp ${hpp_files})
  target_link_libraries(example_simulator autd3_example_common link_simulator)
  
  target_compile_definitions(example_simulator PRIVATE AUTD3_SIMULATOR_FONT_PATH="${PROJECT_SOURCE_DIR}/examples/fonts/NotoSans-Regular.ttf")

  set_target_properties(example_simulator PROPERTIES FOLDER "example")
endif()
