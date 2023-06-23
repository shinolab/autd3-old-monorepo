cmake_minimum_required(VERSION 3.21)

project(autd3-sample)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

if(${CMAKE_VERSION} VERSION_GREATER_EQUAL "3.24.0") 
  cmake_policy(SET CMP0135 NEW)
endif()

include(FetchContent)
if(WIN32)
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v12.2.0/autd3-v12.2.0-win-x64.zip
  )
elseif(APPLE)
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v12.2.0/autd3-v12.2.0-macos-universal.tar.gz
  )
else()
  FetchContent_Declare(
    autd3
    URL https://github.com/shinolab/autd3/releases/download/v12.2.0/autd3-v12.2.0-linux-x64.tar.gz
  )
endif()
set(USE_SYSTEM_EIGEN OFF)
FetchContent_MakeAvailable(autd3)

add_executable(main main.cpp)

autd3_target_link_library(main PRIVATE)

if (CMAKE_GENERATOR MATCHES "Visual Studio")
    set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT main)
endif()
