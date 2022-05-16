cmake_minimum_required(VERSION 3.16)

if(NOT WIN32)
  find_package(Threads REQUIRED)
endif()

file(GLOB include_files ${PROJECT_SOURCE_DIR}/include/*.hpp *.hpp)

add_library(autd3capi-legacy SHARED
  c_api.cpp
  ../base/impl.hpp
  ../base/autd3_c_api.h
  ${include_files}
)

target_link_libraries(autd3capi-legacy ${CMAKE_THREAD_LIBS_INIT})
target_link_libraries(autd3capi-legacy autd3)

if(WIN32)
  target_link_libraries(autd3capi-legacy ws2_32.lib)
  target_link_libraries(autd3capi-legacy winmm.lib)
elseif(APPLE)
  target_link_libraries(autd3capi-legacy pcap)
else()
  target_link_libraries(autd3capi-legacy rt)
  target_link_libraries(autd3capi-legacy -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi-legacy
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)

if (WIN32)
  set_target_properties(autd3capi-legacy PROPERTIES FOLDER "capi")
else()
  set_target_properties(autd3capi-legacy PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
