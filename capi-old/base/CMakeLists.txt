cmake_minimum_required(VERSION 3.21)

find_package(Threads REQUIRED)

add_library(autd3capi SHARED c_api.cpp)

target_link_libraries(autd3capi PRIVATE ${CMAKE_THREAD_LIBS_INIT})
target_link_libraries(autd3capi PRIVATE autd3::autd3)

if(WIN32)
  target_link_libraries(autd3capi PRIVATE winmm)
elseif(APPLE)
else()
  target_link_libraries(autd3capi PRIVATE -static-libgcc -static-libstdc++)
endif()

set_target_properties(autd3capi
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin/$<CONFIG>/"
)

set_target_properties(autd3capi PROPERTIES FOLDER "capi")
set_target_properties(autd3capi PROPERTIES POSITION_INDEPENDENT_CODE ON)
