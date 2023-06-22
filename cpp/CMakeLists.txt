cmake_minimum_required(VERSION 3.21)

if(${CMAKE_SOURCE_DIR} STREQUAL ${CMAKE_BINARY_DIR})
  message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from there. You may need to remove CMakeCache.txt. ")
endif()

project(autd3 VERSION 12.1.0)

message(STATUS "Build AUTD3: ${PROJECT_VERSION}")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_POLICY_DEFAULT_CMP0077 NEW)

option(USE_SYSTEM_EIGEN OFF)
option(AUTD3_BACKEND_CUDA OFF)

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
target_link_libraries(autd3 INTERFACE Eigen3::Eigen)
target_compile_definitions(autd3 INTERFACE EIGEN_MPL2_ONLY)

#file(GLOB_RECURSE autd3_src ${PROJECT_SOURCE_DIR}/include/**/*.hpp)
#add_library(autd3 ${autd3_src})
#set_target_properties(autd3 PROPERTIES LINKER_LANGUAGE CXX)
#target_include_directories(autd3 PUBLIC ${PROJECT_SOURCE_DIR}/include)
#target_link_libraries(autd3 PUBLIC Eigen3::Eigen)
#target_compile_definitions(autd3 PUBLIC EIGEN_MPL2_ONLY)

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
  target_link_libraries(autd3 INTERFACE winmm ws2_32 bcrypt userenv uxtheme dwmapi Imm32 NtDll)
  target_link_directories(autd3 INTERFACE ${PROJECT_SOURCE_DIR}/lib/wpcap)
  target_link_libraries(autd3 INTERFACE Packet wpcap)
  target_link_directories(autd3 INTERFACE ${PROJECT_SOURCE_DIR}/lib)
else()
  target_link_directories(autd3 INTERFACE ${PROJECT_SOURCE_DIR}/bin)
endif()

target_link_libraries(autd3 INTERFACE autd3capi)
target_link_libraries(autd3 INTERFACE autd3capi_gain_holo)
target_link_libraries(autd3 INTERFACE autd3capi_modulation_audio_file)
target_link_libraries(autd3 INTERFACE autd3capi_link_soem)
target_link_libraries(autd3 INTERFACE autd3capi_link_twincat)
target_link_libraries(autd3 INTERFACE autd3capi_link_simulator)
target_link_libraries(autd3 INTERFACE autd3capi_geometry_viewer)
target_link_libraries(autd3 INTERFACE autd3capi_simulator)

if(AUTD3_BACKEND_CUDA)
  if(APPLE)
  else()
    find_package(CUDA REQUIRED)
    find_package(CUDAToolkit REQUIRED)
    message("CUDA version: " ${CUDA_VERSION})
    message("CUDA toolkit version: " ${CUDAToolkit_VERSION})

    set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} --expt-relaxed-constexpr -Xcudafe "--diag_suppress=esa_on_defaulted_function_ignored")
    if(WIN32)
      if(MSVC_TOOLSET_VERSION VERSION_EQUAL 143)
        set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -allow-unsupported-compiler)
      endif()
      if(BUILD_WITH_STATIC_CRT)
        set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -Xcompiler /MT)
      else()
        set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -Xcompiler /MD)
      endif()
    else()
      set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS} -Xcompiler -fPIC)
    endif()

    target_link_libraries(autd3 INTERFACE ${CUDA_LIBRARIES})
    target_link_libraries(autd3 INTERFACE ${CUDA_CUBLAS_LIBRARIES})
    target_link_libraries(autd3 INTERFACE ${CUDA_cusolver_LIBRARY})

    target_link_libraries(autd3 INTERFACE autd3capi_backend_cuda)
  endif()
endif()

add_library(autd3::autd3 ALIAS autd3)
