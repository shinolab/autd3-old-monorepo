cmake_minimum_required(VERSION 3.16)

# currently Windows only
if(WIN32)
  if(BUILD_LINK_SOEM)
    add_executable(c_example_soem soem.c)
    target_include_directories(c_example_soem PRIVATE ${PROJECT_SOURCE_DIR}/capi/base ${PROJECT_SOURCE_DIR}/capi/link_soem)
    target_link_libraries(c_example_soem PRIVATE autd3capi autd3capi-link-soem) 
    target_link_libraries(c_example_soem PRIVATE winmm ws2_32)
    add_custom_command(TARGET c_example_soem POST_BUILD
      COMMAND ${CMAKE_COMMAND} -E copy
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi.dll
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-link-soem.dll
      ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    if(BUILD_GAIN_HOLO)
      target_link_libraries(c_example_soem PRIVATE autd3capi-gain-holo)
      target_include_directories(c_example_soem PRIVATE ${PROJECT_SOURCE_DIR}/capi/gain_holo)
      target_compile_definitions(c_example_soem PRIVATE BUILD_GAIN_HOLO)
      add_custom_command(TARGET c_example_soem POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-gain-holo.dll
        ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    endif()
    if(BUILD_MODULATION_AUDIO_FILE)
      target_link_libraries(c_example_soem PRIVATE autd3capi-modulation-audio-file)
      target_include_directories(c_example_soem PRIVATE ${PROJECT_SOURCE_DIR}/capi/modulation_audio_file)
      target_compile_definitions(c_example_soem PRIVATE BUILD_MODULATION_AUDIO_FILE)
      target_compile_definitions(c_example_soem PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/examples/resources")
      add_custom_command(TARGET c_example_soem POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-modulation-audio-file.dll
        ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
     endif()
    set_target_properties(c_example_soem PROPERTIES FOLDER "capi/example")
  endif()

  if(BUILD_LINK_EMULATOR)
    add_executable(c_example_emulator emulator.c)
    target_include_directories(c_example_emulator PRIVATE ${PROJECT_SOURCE_DIR}/capi/base ${PROJECT_SOURCE_DIR}/capi/link_emulator)
    target_link_libraries(c_example_emulator PRIVATE autd3capi autd3capi-link-emulator) 
    target_link_libraries(c_example_emulator PRIVATE winmm ws2_32)
    add_custom_command(TARGET c_example_emulator POST_BUILD
      COMMAND ${CMAKE_COMMAND} -E copy
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi.dll
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-link-emulator.dll
      ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    if(BUILD_GAIN_HOLO)
      target_link_libraries(c_example_emulator PRIVATE autd3capi-gain-holo)
      target_include_directories(c_example_emulator PRIVATE ${PROJECT_SOURCE_DIR}/capi/gain_holo)
      target_compile_definitions(c_example_emulator PRIVATE BUILD_GAIN_HOLO)
      add_custom_command(TARGET c_example_emulator POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-gain-holo.dll
        ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    endif()
    if(BUILD_MODULATION_AUDIO_FILE)
      target_link_libraries(c_example_emulator PRIVATE autd3capi-modulation-audio-file)
      target_include_directories(c_example_emulator PRIVATE ${PROJECT_SOURCE_DIR}/capi/modulation_audio_file)
      target_compile_definitions(c_example_emulator PRIVATE BUILD_MODULATION_AUDIO_FILE)
      target_compile_definitions(c_example_emulator PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/examples/resources")
      add_custom_command(TARGET c_example_emulator POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-modulation-audio-file.dll
        ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
     endif()
    set_target_properties(c_example_emulator PROPERTIES FOLDER "capi/example")
  endif()

  if(BUILD_LINK_TWINCAT)
    add_executable(c_example_twincat twincat.c)
    target_include_directories(c_example_twincat PRIVATE ${PROJECT_SOURCE_DIR}/capi/base ${PROJECT_SOURCE_DIR}/capi/link_twincat)
    target_link_libraries(c_example_twincat PRIVATE autd3capi autd3capi-link-twincat) 
    target_link_libraries(c_example_twincat PRIVATE winmm ws2_32)
    add_custom_command(TARGET c_example_twincat POST_BUILD
      COMMAND ${CMAKE_COMMAND} -E copy
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi.dll
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-link-twincat.dll
      ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    if(BUILD_GAIN_HOLO)
      target_link_libraries(c_example_twincat PRIVATE autd3capi-gain-holo)
      target_include_directories(c_example_twincat PRIVATE ${PROJECT_SOURCE_DIR}/capi/gain_holo)
      target_compile_definitions(c_example_twincat PRIVATE BUILD_GAIN_HOLO)
      add_custom_command(TARGET c_example_twincat POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-gain-holo.dll
        ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    endif()
    if(BUILD_MODULATION_AUDIO_FILE)
      target_link_libraries(c_example_twincat PRIVATE autd3capi-modulation-audio-file)
      target_include_directories(c_example_twincat PRIVATE ${PROJECT_SOURCE_DIR}/capi/modulation_audio_file)
      target_compile_definitions(c_example_twincat PRIVATE BUILD_MODULATION_AUDIO_FILE)
      target_compile_definitions(c_example_twincat PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/examples/resources")
      add_custom_command(TARGET c_example_twincat POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-modulation-audio-file.dll
        ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
     endif()
    set_target_properties(c_example_twincat PROPERTIES FOLDER "capi/example")
  endif()

  if(BUILD_LINK_REMOTE_TWINCAT)
    add_executable(c_example_remote_twincat remote_twincat.c)
    target_include_directories(c_example_remote_twincat PRIVATE ${PROJECT_SOURCE_DIR}/capi/base ${PROJECT_SOURCE_DIR}/capi/link_remote_twincat)
    target_link_libraries(c_example_remote_twincat PRIVATE autd3capi autd3capi-link-remote-twincat) 
    target_link_libraries(c_example_remote_twincat PRIVATE winmm ws2_32)
    add_custom_command(TARGET c_example_remote_twincat POST_BUILD
      COMMAND ${CMAKE_COMMAND} -E copy
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi.dll
      ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-link-remote-twincat.dll
      ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    if(BUILD_GAIN_HOLO)
      target_link_libraries(c_example_remote_twincat PRIVATE autd3capi-gain-holo)
      target_include_directories(c_example_remote_twincat PRIVATE ${PROJECT_SOURCE_DIR}/capi/gain_holo)
      target_compile_definitions(c_example_remote_twincat PRIVATE BUILD_GAIN_HOLO)
      add_custom_command(TARGET c_example_remote_twincat POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-gain-holo.dll
        ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
    endif()
    if(BUILD_MODULATION_AUDIO_FILE)
      target_link_libraries(c_example_remote_twincat PRIVATE autd3capi-modulation-audio-file)
      target_include_directories(c_example_remote_twincat PRIVATE ${PROJECT_SOURCE_DIR}/capi/modulation_audio_file)
      target_compile_definitions(c_example_remote_twincat PRIVATE BUILD_MODULATION_AUDIO_FILE)
      target_compile_definitions(c_example_remote_twincat PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/examples/resources")
      add_custom_command(TARGET c_example_remote_twincat POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        ${CMAKE_BINARY_DIR}/bin/$<CONFIG>/autd3capi-modulation-audio-file.dll
        ${CMAKE_CURRENT_BINARY_DIR}/$<CONFIG>)
     endif()
    set_target_properties(c_example_remote_twincat PROPERTIES FOLDER "capi/example")
  endif()
endif()
