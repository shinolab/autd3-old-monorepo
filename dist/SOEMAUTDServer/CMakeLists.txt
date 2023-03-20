cmake_minimum_required(VERSION 3.21)

if(NOT EXPORT_SOEM_AUTD_SERVER)
  return()
endif()

find_package(Threads REQUIRED)

file(GLOB_RECURSE SOEM_SRC_FILES ${SOEM_PATH}/soem/*.c ${SOEM_PATH}/osal/${OS}/*.c ${SOEM_PATH}/oshw/${OS}/*.c)

add_executable(SOEMAUTDServer
  soem_server.cpp
  interface.hpp
  local_interface.hpp
  tcp_interface.hpp
  ${SOEM_SRC_FILES}
  ${PROJECT_SOURCE_DIR}/src/link/soem/ecat.hpp
  ${PROJECT_SOURCE_DIR}/src/link/soem/error_handler.hpp
  ${PROJECT_SOURCE_DIR}/src/link/soem/soem_handler.hpp
)
target_link_libraries(SOEMAUTDServer PRIVATE autd3_common)

target_include_directories(SOEMAUTDServer PRIVATE ${PROJECT_SOURCE_DIR}/dist/SOEMAUTDServer ${PROJECT_SOURCE_DIR}/src/link/soem ${PROJECT_SOURCE_DIR}/src/smem)
target_compile_definitions(SOEMAUTDServer PRIVATE SMEM_HEADER_ONLY)
target_include_directories(SOEMAUTDServer PRIVATE
  ${PROJECT_SOURCE_DIR}/include
  ${SOEM_PATH}/soem
  ${SOEM_PATH}/osal
  ${SOEM_PATH}/osal/${OS}
  ${SOEM_PATH}/oshw/${OS}
)
if(WIN32)
  target_include_directories(SOEMAUTDServer PRIVATE 
    ${SOEM_PATH}/oshw/${OS}/wpcap/Include
    ${SOEM_PATH}/oshw/${OS}/wpcap/Include/pcap
  )
  target_compile_definitions(SOEMAUTDServer PRIVATE __STDC_LIMIT_MACROS NOMINMAX _CRT_SECURE_NO_WARNINGS)
endif()

enable_cxx_compiler_flag_if_supported(SOEMAUTDServer PRIVATE /wd4996)

target_link_libraries(SOEMAUTDServer PRIVATE ${CMAKE_THREAD_LIBS_INIT})
if(WIN32)
  if(AUTD3_BUILD_ARM)
    target_link_libraries(SOEMAUTDServer PRIVATE ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/ARM64/Packet.lib ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/ARM64/wpcap.lib)
  elseif("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "Win32")
    target_link_libraries(SOEMAUTDServer PRIVATE ${SOEM_PATH}/oshw/${OS}/wpcap/Lib/Packet.lib ${SOEM_PATH}/oshw/${OS}/wpcap/Lib/wpcap.lib)
  else()
    target_link_libraries(SOEMAUTDServer PRIVATE ${SOEM_PATH}/oshw/${OS}/wpcap/Lib/x64/Packet.lib ${SOEM_PATH}/oshw/${OS}/wpcap/Lib/x64/wpcap.lib)
  endif()
  target_link_libraries(SOEMAUTDServer PRIVATE ws2_32 winmm)
elseif(APPLE)
  target_link_libraries(SOEMAUTDServer PRIVATE pcap)
else()
  target_link_libraries(SOEMAUTDServer PRIVATE rt)
endif()

set_target_properties(SOEMAUTDServer PROPERTIES FOLDER "publish")
