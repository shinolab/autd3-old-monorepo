cmake_minimum_required(VERSION 3.21)

project(autd3_sample)

set (CMAKE_CXX_STANDARD 17)

include(FetchContent)
FetchContent_Declare(
  autd3
  GIT_REPOSITORY https://github.com/shinolab/autd3.git
  GIT_TAG v9.0.0
  GIT_SUBMODULES ""
  GIT_PROGRESS TRUE
  GIT_SHALLOW TRUE
)
set(BUILD_LINK_SOEM ON)
FetchContent_MakeAvailable(autd3)

add_executable(main main.cpp)

target_link_libraries(main autd3::autd3 autd3::link::soem)

if (CMAKE_GENERATOR MATCHES "Visual Studio")
    set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT main)
endif()
