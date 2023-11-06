cmake_minimum_required(VERSION 3.21)

if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from there. You may need to remove CMakeCache.txt. ")
endif()

project(autd3-examples)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(AUTD_LOCAL_TEST "For local test build" OFF)
option(ENABLE_LINK_VISUALIZER ENABLE_LINK_VISUALIZER OFF)
option(USE_PYTHON "Use python backend for visualizer" OFF)

if(${CMAKE_VERSION} VERSION_GREATER_EQUAL "3.24.0") 
  cmake_policy(SET CMP0135 NEW)
endif()

include(FetchContent)
if(AUTD_LOCAL_TEST)
  FetchContent_Declare(
    autd3
    SOURCE_DIR ${PROJECT_SOURCE_DIR}/..
  )
elseif(WIN32)
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v17.0.0/autd3-v17.0.0
  )
elseif(APPLE)
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v17.0.0/autd3-v17.0.0
  )
else()
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v17.0.0/autd3-v17.0.0
  )
endif()
FetchContent_MakeAvailable(autd3)

add_library(autd3-example INTERFACE)
target_compile_definitions(autd3-example INTERFACE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/resources")

add_executable(soem soem.cpp)
autd3_target_link_library(soem PRIVATE)
target_link_libraries(soem PRIVATE autd3-example)

add_executable(twincat twincat.cpp)
autd3_target_link_library(twincat PRIVATE)
target_link_libraries(twincat PRIVATE autd3-example)

add_executable(remote_twincat remote_twincat.cpp)
autd3_target_link_library(remote_twincat PRIVATE)
target_link_libraries(remote_twincat PRIVATE autd3-example)

add_executable(remote_soem remote_soem.cpp)
autd3_target_link_library(remote_soem PRIVATE)
target_link_libraries(remote_soem PRIVATE autd3-example)

add_executable(simulator simulator.cpp)
autd3_target_link_library(simulator PRIVATE)
target_link_libraries(simulator PRIVATE autd3-example)

add_executable(group_gain group_gain.cpp)
autd3_target_link_library(group_gain PRIVATE)
target_link_libraries(group_gain PRIVATE autd3-example)

if(ENABLE_LINK_VISUALIZER)
  add_executable(visualizer visualizer.cpp)
  autd3_target_link_library(visualizer PRIVATE)
  target_link_libraries(visualizer PRIVATE autd3-example)
  if(USE_PYTHON)
    target_compile_definitions(visualizer PRIVATE USE_PYTHON)
  endif()
endif()
