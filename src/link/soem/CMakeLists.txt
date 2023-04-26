cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_SOEM)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/soem.cmake)

add_library(autd3_link_soem link_soem_impl.cpp)
add_library(autd3::link::soem ALIAS autd3_link_soem)

target_link_libraries(autd3_link_soem PRIVATE autd3::common)
target_link_libraries(autd3_link_soem PRIVATE soem)

if(WIN32)
  target_include_directories(autd3_link_soem PRIVATE 
    ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Include
    ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Include/pcap
  )
  target_compile_definitions(autd3_link_soem PRIVATE NOMINMAX _CRT_SECURE_NO_WARNINGS)
  target_compile_definitions(autd3_link_soem PRIVATE __STDC_LIMIT_MACROS)
endif()
enable_cxx_compiler_flag_if_supported(autd3_link_soem PRIVATE /wd4996)

if(WIN32)
  if(AUTD3_BUILD_ARM)
    target_link_directories(autd3_link_soem INTERFACE ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/ARM64)
  elseif("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "Win32")
    target_link_directories(autd3_link_soem INTERFACE ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Lib)
  else()
    target_link_directories(autd3_link_soem INTERFACE ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Lib/x64)
  endif()
  target_link_libraries(autd3_link_soem INTERFACE Packet.lib wpcap.lib)
endif()

set_target_properties(autd3_link_soem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_link_soem PROPERTIES FOLDER "link")
set_target_properties(autd3_link_soem PROPERTIES POSITION_INDEPENDENT_CODE ON)
