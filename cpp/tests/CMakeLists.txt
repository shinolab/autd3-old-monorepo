cmake_minimum_required(VERSION 3.21)

if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from there. You may need to remove CMakeCache.txt. ")
endif()

project(autd3-test)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(ENABLE_BACKEND_CUDA "ENABLE_BACKEND_CUDA" OFF)

set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)

if(POLICY CMP0135)
  cmake_policy(SET CMP0135 NEW)
endif()
include(${PROJECT_SOURCE_DIR}/gtest.cmake)

enable_testing()

include(FetchContent)
FetchContent_Declare(
  autd3
  SOURCE_DIR ${PROJECT_SOURCE_DIR}/..
)
FetchContent_MakeAvailable(autd3)

add_executable(test_autd3 main.cpp)

target_include_directories(test_autd3 PRIVATE .)
target_link_libraries(test_autd3 PRIVATE gtest_main)
autd3_target_link_library(test_autd3 PRIVATE)

add_subdirectory(${PROJECT_SOURCE_DIR}/gain)
add_subdirectory(${PROJECT_SOURCE_DIR}/internal)
add_subdirectory(${PROJECT_SOURCE_DIR}/link)
add_subdirectory(${PROJECT_SOURCE_DIR}/modulation)

add_test(NAME test_autd3 COMMAND test_autd3)

include(GoogleTest)
gtest_discover_tests(test_autd3)

if(WIN32)
  target_link_options(test_autd3 PRIVATE /NODEFAULTLIB:libcmt.lib /NODEFAULTLIB:libcmtd.lib /NODEFAULTLIB:msvcrtd.lib)
endif()

if(ENABLE_BACKEND_CUDA)
  target_compile_definitions(test_autd3 PRIVATE ENABLE_BACKEND_CUDA)
endif()

target_compile_definitions(test_autd3 PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/../examples/resources")
