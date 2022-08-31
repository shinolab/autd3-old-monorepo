cmake_minimum_required(VERSION 3.16)

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
    target_link_libraries(${target} PRIVATE rt)
    target_link_libraries(${target} PRIVATE -static-libgcc -static-libstdc++)
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

if(BUILD_LINK_EMULATOR)
  add_executable(c_example_emulator emulator.c)
  add_autd_dependencies(c_example_emulator)
  target_include_directories(c_example_emulator PRIVATE ${PROJECT_SOURCE_DIR}/capi/link_emulator)
  target_link_libraries(c_example_emulator PRIVATE autd3capi autd3capi-link-emulator) 
  add_custom_command(TARGET c_example_emulator POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/${prefix}autd3capi-link-emulator.${extention}
    ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
  set_target_properties(c_example_emulator PROPERTIES FOLDER "capi/example")
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
