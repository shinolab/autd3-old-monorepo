cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_EMEM)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/soem.cmake)

add_library(autd3_link_emem emem.cpp)
add_library(autd3::link::emem ALIAS autd3_link_emem)
target_link_libraries(autd3_link_emem PRIVATE autd3::common)

if(WIN32)
  target_include_directories(autd3_link_emem PRIVATE 
    pcap/Include
    pcap/Include/pcap
  )
  target_compile_definitions(autd3_link_emem PRIVATE __STDC_LIMIT_MACROS NOMINMAX _CRT_SECURE_NO_WARNINGS)
endif()

target_link_libraries(autd3_link_emem INTERFACE ${CMAKE_THREAD_LIBS_INIT})
if(WIN32)
  if(AUTD3_BUILD_ARM)
    target_link_libraries(autd3_link_emem INTERFACE ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/ARM64/Packet.lib ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/ARM64/wpcap.lib)
  else()
    target_link_libraries(autd3_link_emem INTERFACE ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Lib/x64/Packet.lib ${soem_SOURCE_DIR}/oshw/${OS}/wpcap/Lib/x64/wpcap.lib)
  endif()
endif()

set_target_properties(autd3_link_emem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(autd3_link_emem PROPERTIES FOLDER "link")
set_target_properties(autd3_link_emem PROPERTIES POSITION_INDEPENDENT_CODE ON)
