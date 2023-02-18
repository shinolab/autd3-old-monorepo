cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_EMEM)
  return()
endif()

add_library(link_emem
  ${PROJECT_SOURCE_DIR}/include/autd3/link/emem.hpp
  emem.cpp
  buffer.hpp
  consts.hpp
  result.hpp
  ethercat_driver.hpp
  idx_stack.hpp
  master.hpp
  network_driver.hpp
  utils.hpp
  ethercat/command.hpp
  ethercat/datagram_addr.hpp
  ethercat/datagram.hpp
  ethercat/fmmu.hpp
  ethercat/header.hpp
  ethercat/slave.hpp
  ethercat/sync_manager.hpp
  pcap/pcap_interface.hpp
  pcap/adapter.hpp
)
target_link_libraries(link_emem PUBLIC autd3_common)

if(WIN32)
  target_include_directories(link_emem PRIVATE 
    pcap/Include
    pcap/Include/pcap
  )
  target_compile_definitions(link_emem PRIVATE __STDC_LIMIT_MACROS NOMINMAX _CRT_SECURE_NO_WARNINGS)
endif()

target_link_libraries(link_emem INTERFACE ${CMAKE_THREAD_LIBS_INIT})
if(WIN32)
  if(AUTD3_BUILD_ARM)
    target_link_libraries(link_emem PUBLIC ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/ARM64/Packet.lib ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/ARM64/wpcap.lib)
  else()
    target_link_libraries(link_emem PUBLIC ${SOEM_PATH}/oshw/${OS}/wpcap/Lib/x64/Packet.lib ${SOEM_PATH}/oshw/${OS}/wpcap/Lib/x64/wpcap.lib)
  endif()
  target_link_libraries(link_emem PUBLIC ws2_32 winmm)
elseif(APPLE)
  target_link_libraries(link_emem INTERFACE pcap)
else()
  target_link_libraries(link_emem INTERFACE rt)
endif()

set_target_properties(link_emem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_emem PROPERTIES FOLDER "link")
set_target_properties(link_emem PROPERTIES POSITION_INDEPENDENT_CODE ON)
