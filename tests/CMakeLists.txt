cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_TEST)
  return()
endif()

cmake_policy(SET CMP0077 NEW)
if(POLICY CMP0135)
  cmake_policy(SET CMP0135 NEW)
endif()
include(${PROJECT_SOURCE_DIR}/cmakes/gtest.cmake)

enable_testing()

add_executable(test_autd3)
target_include_directories(test_autd3 PRIVATE .)
target_link_libraries(test_autd3 gtest_main)
target_link_libraries(test_autd3 autd3::autd3)

add_subdirectory(${PROJECT_SOURCE_DIR}/tests/driver)
add_subdirectory(${PROJECT_SOURCE_DIR}/tests/core)
add_subdirectory(${PROJECT_SOURCE_DIR}/tests/gain)
add_subdirectory(${PROJECT_SOURCE_DIR}/tests/modulation)

target_sources(test_autd3 PRIVATE
  test.cpp
  autd3_device_test.cpp
)

add_test(NAME test_autd3 COMMAND test_autd3)

include(GoogleTest)
gtest_discover_tests(test_autd3)

if(WIN32)
  target_link_options(test_autd3 PRIVATE /NODEFAULTLIB:libcmt.lib /NODEFAULTLIB:libcmtd.lib /NODEFAULTLIB:msvcrtd.lib)
endif()
set_target_properties(test_autd3 PROPERTIES FOLDER "test")
set_target_properties(gtest PROPERTIES FOLDER "test/gtest")
set_target_properties(gtest_main PROPERTIES FOLDER "test/gtest")
