cmake_minimum_required(VERSION 3.21)

if(NOT EXPORT_SOEM_AUTD_SERVER)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/argparse.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/soem.cmake)

find_package(Threads REQUIRED)

add_executable(SOEMAUTDServer soem_server.cpp)
target_link_libraries(SOEMAUTDServer PRIVATE autd3::common soem argparse::argparse)

target_include_directories(SOEMAUTDServer PRIVATE ${PROJECT_SOURCE_DIR}/dist/SOEMAUTDServer ${PROJECT_SOURCE_DIR}/src/link/soem ${PROJECT_SOURCE_DIR}/src/smem)
target_compile_definitions(SOEMAUTDServer PRIVATE SMEM_HEADER_ONLY)
target_include_directories(SOEMAUTDServer PRIVATE
  ${PROJECT_SOURCE_DIR}/include
)
if(WIN32)
  target_include_directories(SOEMAUTDServer PRIVATE 
    ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Include
    ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Include/pcap
  )
  target_compile_definitions(SOEMAUTDServer PRIVATE __STDC_LIMIT_MACROS NOMINMAX _CRT_SECURE_NO_WARNINGS)
endif()

enable_cxx_compiler_flag_if_supported(SOEMAUTDServer PRIVATE /wd4996)

target_link_libraries(SOEMAUTDServer PRIVATE ${CMAKE_THREAD_LIBS_INIT})
if(WIN32)
  if(AUTD3_BUILD_ARM)
    target_link_directories(SOEMAUTDServer PRIVATE ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/ARM64)
  elseif("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "Win32")
    target_link_directories(SOEMAUTDServer PRIVATE ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Lib)
  else()
    target_link_directories(SOEMAUTDServer PRIVATE ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Lib/x64)
  endif()
  target_link_libraries(SOEMAUTDServer PRIVATE Packet.lib wpcap.lib)
elseif(APPLE)
else()
endif()

set_target_properties(SOEMAUTDServer PROPERTIES FOLDER "publish")
