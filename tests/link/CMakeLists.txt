file(GLOB test_files *.cpp)

target_sources(test_autd3 PRIVATE
  ${test_files}
)

if(BUILD_LINK_SOEM)
  target_compile_definitions(test_autd3 PRIVATE TEST_LINK_SOEM)
  target_link_libraries(test_autd3 link_soem)
endif()

if(BUILD_LINK_TWINCAT)
  target_compile_definitions(test_autd3 PRIVATE TEST_LINK_TWINCAT)
  target_link_libraries(test_autd3 link_twincat)
endif()

if(BUILD_LINK_REMOTE_TWINCAT)
  target_compile_definitions(test_autd3 PRIVATE TEST_LINK_REMOTE_TWINCAT)
  target_link_libraries(test_autd3 link_remote_twincat)
endif()

if(BUILD_LINK_EMULATOR)
  target_compile_definitions(test_autd3 PRIVATE TEST_LINK_EMULATOR)
  target_link_libraries(test_autd3 link_emulator)
endif()
