cmake_minimum_required(VERSION 3.21)

if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from there. You may need to remove CMakeCache.txt. ")
endif()

project(autd3-examples)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(AUTD_LOCAL_TEST "For local test build" OFF)

include(FetchContent)
if(AUTD_LOCAL_TEST)
  FetchContent_Declare(
    autd3
    SOURCE_DIR ${PROJECT_SOURCE_DIR}/..
  )
elseif(WIN32)
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v10.0.0/autd3-v10.0.0-win-x64.zip
    SOURCE_DIR ${PROJECT_SOURCE_DIR}/..
  )
elseif(APPLE)
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v10.0.0/autd3-v10.0.0-macos-universal.tar.gz
  )
else()
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v10.0.0/autd3-v10.0.0-linux-x64.tar.gz
  )
endif()
FetchContent_MakeAvailable(autd3)

add_executable(soem soem.cpp)
target_link_libraries(soem PRIVATE autd3)
target_compile_definitions(soem PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/resources")

add_executable(twincat twincat.cpp)
target_link_libraries(twincat PRIVATE autd3)
target_compile_definitions(twincat PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/resources")

add_executable(remote_twincat remote_twincat.cpp)
target_link_libraries(remote_twincat PRIVATE autd3)
target_compile_definitions(remote_twincat PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/resources")

add_executable(remote_soem remote_soem.cpp)
target_link_libraries(remote_soem PRIVATE autd3)
target_compile_definitions(remote_soem PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/resources")

add_executable(geometry_viewer geometry_viewer.cpp)
target_link_libraries(geometry_viewer PRIVATE autd3)

add_executable(simulator_server simulator_server.cpp)
target_link_libraries(simulator_server PRIVATE autd3)

add_executable(simulator_client simulator_client.cpp)
target_link_libraries(simulator_client PRIVATE autd3)
target_compile_definitions(simulator_client PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/resources")

add_executable(debug debug.cpp)
target_link_libraries(debug PRIVATE autd3)
target_compile_definitions(debug PRIVATE AUTD3_RESOURCE_PATH="${PROJECT_SOURCE_DIR}/resources")

add_executable(freq_config freq_config.cpp)
target_link_libraries(freq_config PRIVATE autd3)
