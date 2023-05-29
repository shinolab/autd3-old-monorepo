cmake_minimum_required(VERSION 3.21)

if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from there. You may need to remove CMakeCache.txt. ")
endif()

project(autd3 VERSION 10.0.0)

message(STATUS "Build AUTD3: ${VERSION_MAJOR}.${VERSION_MINOR}.${VERSION_PATCH}")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)

option(USE_SYSTEM_EIGEN OFF)

if(USE_SYSTEM_EIGEN)
  find_package(Eigen REQUIRED)
else()
  include(FetchContent)
  set(EIGEN_BUILD_DOC OFF)
  set(BUILD_TESTING OFF)
  set(EIGEN_BUILD_PKGCONFIG OFF)
  FetchContent_Declare(
    eigen
    GIT_REPOSITORY https://gitlab.com/libeigen/eigen.git
    GIT_TAG 3.4.0)
  FetchContent_MakeAvailable(eigen)
endif()

add_library(autd3 INTERFACE)
target_include_directories(autd3 INTERFACE ${PROJECT_SOURCE_DIR}/include)
target_link_directories(autd3 INTERFACE ${PROJECT_SOURCE_DIR}/lib)
target_link_libraries(autd3 INTERFACE Eigen3::Eigen)
target_compile_definitions(autd3 INTERFACE EIGEN_MPL2_ONLY)

include(CheckCXXCompilerFlag)
function(enable_cxx_compiler_flag_if_supported target scope flag)
  check_cxx_compiler_flag("${flag}" flag_supported)
  if(flag_supported)
    target_compile_options(${target} ${scope} ${flag})
    endif()
  unset(flag_supported CACHE)
endfunction()

enable_cxx_compiler_flag_if_supported(autd3 INTERFACE /wd4819)

if(WIN32)
  target_link_libraries(autd3 INTERFACE winmm ws2_32 bcrypt userenv uxtheme dwmapi Imm32)
  target_link_directories(autd3 INTERFACE ${PROJECT_SOURCE_DIR}/lib/wpcap)
  target_link_libraries(autd3 INTERFACE Packet wpcap)
endif()

target_link_libraries(autd3 INTERFACE autd3capi)
target_link_libraries(autd3 INTERFACE autd3capi_gain_holo)
target_link_libraries(autd3 INTERFACE autd3capi_modulation_audio_file)
target_link_libraries(autd3 INTERFACE autd3capi_link_soem)
target_link_libraries(autd3 INTERFACE autd3capi_link_twincat)
target_link_libraries(autd3 INTERFACE autd3capi_link_simulator)
target_link_libraries(autd3 INTERFACE autd3capi_geometry_viewer)
target_link_libraries(autd3 INTERFACE autd3capi_simulator)

add_library(autd3::autd3 ALIAS autd3)
