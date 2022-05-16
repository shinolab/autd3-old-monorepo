cmake_minimum_required(VERSION 3.16)

if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from there. You may need to remove CMakeCache.txt. ")
endif()

project(autd3)

set(VERSION_MAJOR 2 CACHE STRING "Project major version number.")
set(VERSION_MINOR 0 CACHE STRING "Project minor version number.")
set(VERSION_PATCH 0 CACHE STRING "Project patch version number.")
mark_as_advanced(VERSION_MAJOR VERSION_MINOR VERSION_PATCH)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(BUILD_ALL "BUILD_ALL" OFF)
option(BUILD_DOC "BUILD_DOC" OFF)

option(BUILD_GAIN_HOLO "BUILD_GAIN_HOLO" ON)
option(BUILD_BACKEND_CUDA "BUILD_BACKEND_CUDA" OFF)

option(BUILD_MODULATION_AUDIO_FILE "BUILD_MODULATION_AUDIO_FILE" OFF)

option(BUILD_LINK_SOEM "BUILD_LINK_SOEM" ON)
option(BUILD_LINK_TWINCAT "BUILD_LINK_TWINCAT" OFF)
option(BUILD_LINK_REMOTE_TWINCAT "BUILD_LINK_REMOTE_TWINCAT" OFF)
option(BUILD_LINK_EMULATOR "BUILD_LINK_EMULATOR" OFF)

option(BUILD_EXAMPLES "BUILD_EXAMPLES" ON)

option(ENABLE_LINT "ENABLE_LINT" OFF)

option(BUILD_TEST "BUILD_TEST" OFF)
option(gtest_disable_pthreads  "gtest_disable_pthreads" ON)

option(BUILD_WITH_STATIC_CRT "BUILD_WITH_STATIC_CRT" OFF)

if(BUILD_ALL)
  set(BUILD_GAIN_HOLO ON)
  set(BUILD_MODULATION_AUDIO_FILE ON)
  set(BUILD_LINK_SOEM ON)
  set(BUILD_LINK_TWINCAT ON)
  set(BUILD_LINK_REMOTE_TWINCAT ON)
  set(BUILD_LINK_EMULATOR ON)
  set(BUILD_EXAMPLES ON)
  set(BUILD_CAPI ON)
endif()

if(BUILD_DOC)
  find_package(Doxygen)
  if(DOXYGEN_FOUND)
    message(STATUS "Doxygen found.")
    add_subdirectory(doc)
  else()
    message(STATUS "Doxygen is needed to build the documentation. Skip building doc...")
  endif()
endif()

if(WIN32)
  set(OS win32)
  include(cmakes/win_vs.cmake)
elseif(APPLE)
  set(OS macosx)
  include(cmakes/mac.cmake)
else()
  set(OS linux)
  if(DEFINED BUILD_TARGET)
    if (${BUILD_TARGET} STREQUAL ARM32 OR ${BUILD_TARGET} STREQUAL ARM)
      include(cmakes/arm32.cmake)
    elseif(${BUILD_TARGET} STREQUAL ARM64 OR ${BUILD_TARGET} STREQUAL AARCH64)
      include(cmakes/arm64.cmake)
    elseif(${BUILD_TARGET} STREQUAL AMD64 OR ${BUILD_TARGET} STREQUAL x86_64 OR ${BUILD_TARGET} STREQUAL x64)
      include(cmakes/linux.cmake)
    endif()
  else()
    include(cmakes/linux.cmake)
  endif()
endif()

find_package(Git)
if(NOT Git_FOUND)
  message(WARNING "Git not found.")
endif()

if(ENABLE_LINT)
  if(Git_FOUND)
    execute_process(COMMAND git -C ${PROJECT_SOURCE_DIR} submodule update --init --recursive -- ${PROJECT_SOURCE_DIR}/3rdparty/cpplint)
  endif()
  file(GLOB_RECURSE lint_files src/*.cpp src/*.hpp include/*.hpp capi/*.cpp capi/*.hpp examples/*.cpp examples/*.hpp)
  ADD_CUSTOM_TARGET(cpplint
    python
    ${PROJECT_SOURCE_DIR}/3rdparty/cpplint/cpplint.py
    --filter=-runtime/references,-build/c++11,-whitespace/comments,-readability/braces,-build/include_subdir,-build/include_order
    --linelength=1000
    ${lint_files}
  )
endif()

if(Git_FOUND)
  execute_process(COMMAND git -C ${PROJECT_SOURCE_DIR} submodule update --init --recursive -- ${PROJECT_SOURCE_DIR}/3rdparty/eigen)
  set(EIGEN_PATH ${PROJECT_SOURCE_DIR}/3rdparty/eigen)
endif()
add_compile_definitions(EIGEN_MPL2_ONLY)

add_subdirectory(src)

if(BUILD_GAIN_HOLO)
  add_subdirectory(src/gain/holo)
endif()

if(BUILD_MODULATION_AUDIO_FILE)
  add_subdirectory(src/modulation/audio_file)
endif()

if(BUILD_LINK_SOEM)
  if(Git_FOUND)
    execute_process(COMMAND git -C ${PROJECT_SOURCE_DIR} submodule update --init --recursive -- ${PROJECT_SOURCE_DIR}/3rdparty/SOEM)
    set(SOEM_PATH ${PROJECT_SOURCE_DIR}/3rdparty/SOEM)
  endif()
  add_subdirectory(src/link/soem)
endif()

if(BUILD_LINK_TWINCAT)
  add_subdirectory(src/link/twincat)
endif()

if(BUILD_LINK_REMOTE_TWINCAT)
  if(Git_FOUND)
    execute_process(COMMAND git -C ${PROJECT_SOURCE_DIR} submodule update --init --recursive -- ${PROJECT_SOURCE_DIR}/3rdparty/ADS)
  endif()
  set(ADS_PATH ${PROJECT_SOURCE_DIR}/3rdparty/ADS)
  add_subdirectory(src/link/remote_twincat)
endif()

if(BUILD_LINK_EMULATOR)
  add_subdirectory(src/link/emulator)
endif()

if(BUILD_EXAMPLES)
  add_subdirectory(examples)
endif()

if(BUILD_TEST)
  set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)
  include(${PROJECT_SOURCE_DIR}/cmakes/gtest.cmake)
  add_subdirectory(tests)
endif()
