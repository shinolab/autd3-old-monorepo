cmake_minimum_required(VERSION 3.16)

if(NOT WIN32)
  find_package(Threads REQUIRED)
endif()

file(GLOB_RECURSE SOEM_SRC_FILES ${SOEM_PATH}/soem/*.c ${SOEM_PATH}/osal/${OS}/*.c ${SOEM_PATH}/oshw/${OS}/*.c)

add_library(link_soem
  link_soem_impl.cpp
  link_soem_impl.hpp
  ${SOEM_SRC_FILES}
  ${PROJECT_SOURCE_DIR}/include/autd3/link/soem.hpp
)

target_link_libraries(link_soem PUBLIC autd3_common)

target_include_directories(link_soem PRIVATE
  ${PROJECT_SOURCE_DIR}/include
  ${SOEM_PATH}/soem
  ${SOEM_PATH}/osal
  ${SOEM_PATH}/osal/${OS}
  ${SOEM_PATH}/oshw/${OS}
)
if(WIN32)
  target_include_directories(link_soem PRIVATE 
    ${SOEM_PATH}/oshw/${OS}/wpcap/Include
    ${SOEM_PATH}/oshw/${OS}/wpcap/Include/pcap
  )
  target_compile_definitions(link_soem PRIVATE __STDC_LIMIT_MACROS NOMINMAX _CRT_SECURE_NO_WARNINGS)
  if(MSVC)
    target_compile_options(link_soem PRIVATE /wd4996)
  endif()
endif()

target_link_libraries(link_soem INTERFACE ${CMAKE_THREAD_LIBS_INIT})
if(WIN32)
  if("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "Win32")
    target_link_directories(link_soem INTERFACE ${PROJECT_SOURCE_DIR}/src/link/soem/Lib)
  else()
    target_link_directories(link_soem INTERFACE ${PROJECT_SOURCE_DIR}/src/link/soem/Lib/x64)
  endif()
  target_link_libraries(link_soem INTERFACE Packet.lib wpcap.lib)
  target_link_libraries(link_soem INTERFACE ws2_32 winmm)
elseif(APPLE)
  target_link_libraries(link_soem INTERFACE pcap)
else()
  target_link_libraries(link_soem INTERFACE rt -static-libgcc -static-libstdc++)
endif()

set_target_properties(link_soem
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

if(WIN32)
  set_target_properties(link_soem PROPERTIES FOLDER "link")
else()
  set_target_properties(link_soem PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
