cmake_minimum_required(VERSION 3.16)

if(NOT WIN32)
  find_package(Threads REQUIRED)
endif()

add_library(autd3capi SHARED
  c_api.cpp
  impl.hpp
  header.h
  autd3_c_api.h
  ${include_files}
)

target_link_libraries(autd3capi ${CMAKE_THREAD_LIBS_INIT})
target_link_libraries(autd3capi autd3)

if(WIN32)
  target_link_libraries(autd3capi ws2_32.lib)
  target_link_libraries(autd3capi winmm.lib)
elseif(APPLE)
  target_link_libraries(autd3capi pcap)
else()
  target_link_libraries(autd3capi rt)
  target_link_libraries(autd3capi -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

if (WIN32)
  set_target_properties(autd3capi PROPERTIES FOLDER "capi")
else()
  set_target_properties(autd3capi PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
