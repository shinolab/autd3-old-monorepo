cmake_minimum_required(VERSION 3.21)

if(WIN32) 
  set(prefix "")
  set(extention dll)
elseif(APPLE)
  set(prefix lib)
  set(extention dylib)
else()
  set(prefix lib)
  set(extention so)
endif()

function(add_autd_dependencies target)
  target_include_directories(${target} PRIVATE ${PROJECT_SOURCE_DIR}/capi/base)
  target_link_libraries(${target} PRIVATE autd3capi)
  if(WIN32) 
    target_link_libraries(${target} PRIVATE winmm ws2_32)
  elseif(APPLE)
    target_link_libraries(${target} PRIVATE pcap)
  else()
    target_link_libraries(${target} PRIVATE rt m)
  endif()
  add_custom_command(TARGET ${target} POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi.${extention}
    ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  if(BUILD_GAIN_HOLO)
    target_link_libraries(${target} PRIVATE autd3capi-gain-holo)
    target_include_directories(${target} PRIVATE ${PROJECT_SOURCE_DIR}/capi/gain_holo)
    target_compile_definitions(${target} PRIVATE BUILD_GAIN_HOLO)
    add_custom_command(TARGET ${target} POST_BUILD
      COMMAND ${CMAKE_COMMAND} -E copy
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-gain-holo.${extention}
      ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  endif()
  if(BUILD_MODULATION_AUDIO_FILE)
    target_link_libraries(${target} PRIVATE autd3capi-modulation-audio-file)
    target_include_directories(${target} PRIVATE ${PROJECT_SOURCE_DIR}/capi/modulation_audio_file)
    target_compile_definitions(${target} PRIVATE BUILD_MODULATION_AUDIO_FILE)
    target_compile_definitions(${target} PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/examples/resources")
    add_custom_command(TARGET ${target} POST_BUILD
      COMMAND ${CMAKE_COMMAND} -E copy
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-modulation-audio-file.${extention}
      ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  endif()
endfunction()

if(BUILD_LINK_SOEM)
  add_executable(c_example_soem soem.c)
  add_autd_dependencies(c_example_soem)
  target_include_directories(c_example_soem PRIVATE ${PROJECT_SOURCE_DIR}/capi/link_soem)
  target_link_libraries(c_example_soem PRIVATE autd3capi-link-soem) 
  add_custom_command(TARGET c_example_soem POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-link-soem.${extention}
    ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  set_target_properties(c_example_soem PROPERTIES FOLDER "capi/example")
endif()

if(WIN32)
  if(BUILD_LINK_TWINCAT)
    add_executable(c_example_twincat twincat.c)
    add_autd_dependencies(c_example_twincat)
    target_include_directories(c_example_twincat PRIVATE ${PROJECT_SOURCE_DIR}/capi/link_twincat)
    target_link_libraries(c_example_twincat PRIVATE autd3capi-link-twincat) 
    add_custom_command(TARGET c_example_twincat POST_BUILD
      COMMAND ${CMAKE_COMMAND} -E copy
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-link-twincat.${extention}
      ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    set_target_properties(c_example_twincat PROPERTIES FOLDER "capi/example")
  endif()
endif()

if(BUILD_LINK_REMOTE_TWINCAT)
  add_executable(c_example_remote_twincat remote_twincat.c)
  add_autd_dependencies(c_example_remote_twincat)
  target_include_directories(c_example_remote_twincat PRIVATE ${PROJECT_SOURCE_DIR}/capi/link_remote_twincat)
  target_link_libraries(c_example_remote_twincat PRIVATE autd3capi-link-remote-twincat) 
  add_custom_command(TARGET c_example_remote_twincat POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-link-remote-twincat.${extention}
    ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  set_target_properties(c_example_remote_twincat PROPERTIES FOLDER "capi/example")
endif()

if(BUILD_LINK_DEBUG)
  add_executable(c_example_debug debug.c)
  add_autd_dependencies(c_example_debug)
  target_include_directories(c_example_debug PRIVATE ${PROJECT_SOURCE_DIR}/capi/link_debug)
  target_link_libraries(c_example_debug PRIVATE autd3capi-link-debug) 
  add_custom_command(TARGET c_example_debug POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-link-debug.${extention}
    ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  set_target_properties(c_example_debug PROPERTIES FOLDER "capi/example")
endif()

if(BUILD_LINK_SIMULATOR)
  add_executable(c_example_simulator_client simulator_client.c)
  add_autd_dependencies(c_example_simulator_client)
  target_include_directories(c_example_simulator_client PRIVATE ${PROJECT_SOURCE_DIR}/capi/link_simulator)
  target_link_libraries(c_example_simulator_client PRIVATE autd3capi-link-simulator) 
  add_custom_command(TARGET c_example_simulator_client POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-link-simulator.${extention}
    ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  set_target_properties(c_example_simulator_client PROPERTIES FOLDER "capi/example")
endif()

if(BUILD_GEOMETRY_VIEWER)
  add_executable(c_example_geometry_viewer geometry_viewer.c)
  add_autd_dependencies(c_example_geometry_viewer)
  target_include_directories(c_example_geometry_viewer PRIVATE ${PROJECT_SOURCE_DIR}/capi/extra_geometry_viewer)
  target_link_libraries(c_example_geometry_viewer PRIVATE autd3capi-extra-geometry-viewer)
  target_compile_definitions(c_example_geometry_viewer PRIVATE AUTD3_GEOMETRY_VIEWER_MODEL_PATH="${PROJECT_SOURCE_DIR}/src/extra/geometry_viewer/models/AUTD.glb")
  target_compile_definitions(c_example_geometry_viewer PRIVATE AUTD3_GEOMETRY_VIEWER_FONT_PATH="${PROJECT_SOURCE_DIR}/examples/fonts/NotoSans-Regular.ttf")
  add_custom_command(TARGET c_example_geometry_viewer POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-extra-geometry-viewer.${extention}
    ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  set_target_properties(c_example_geometry_viewer PROPERTIES FOLDER "capi/example")
endif()

if(BUILD_SIMULATOR)
  add_executable(c_example_simulator_server simulator_server.c)
  add_autd_dependencies(c_example_simulator_server)
  target_include_directories(c_example_simulator_server PRIVATE ${PROJECT_SOURCE_DIR}/capi/extra_simulator)
  target_link_libraries(c_example_simulator_server PRIVATE autd3capi-extra-simulator)
  add_custom_command(TARGET c_example_simulator_server POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-extra-simulator.${extention}
    ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  set_target_properties(c_example_simulator_server PROPERTIES FOLDER "capi/example")
endif()
