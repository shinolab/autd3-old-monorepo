cmake_minimum_required(VERSION 3.21)

if(NOT BUILD_LINK_REMOTE_TWINCAT)
  return()
endif()

add_library(link_remote_twincat
  remote_twincat.cpp
  ${PROJECT_SOURCE_DIR}/include/autd3/link/remote_twincat.hpp

  ${ADS_PATH}/AdsLib/AdsDef.cpp
  ${ADS_PATH}/AdsLib/AdsDevice.cpp
  ${ADS_PATH}/AdsLib/Log.cpp
  ${ADS_PATH}/AdsLib/Sockets.cpp
  ${ADS_PATH}/AdsLib/Frame.cpp
  ${ADS_PATH}/AdsLib/standalone/AdsLib.cpp
  ${ADS_PATH}/AdsLib/standalone/AmsConnection.cpp
  ${ADS_PATH}/AdsLib/standalone/AmsNetId.cpp
  ${ADS_PATH}/AdsLib/standalone/AmsPort.cpp
  ${ADS_PATH}/AdsLib/standalone/AmsRouter.cpp
  ${ADS_PATH}/AdsLib/standalone/NotificationDispatcher.cpp
)

target_link_libraries(link_remote_twincat PUBLIC autd3_common)
target_include_directories(link_remote_twincat PRIVATE ${ADS_PATH}/AdsLib ${ADS_PATH}/AdsLib/standalone)

if(WIN32)
  target_link_libraries(link_remote_twincat PUBLIC ws2_32)
  target_compile_definitions(link_remote_twincat PRIVATE NOMINMAX)
elseif(APPLE)
else()
endif()
enable_cxx_compiler_flag_if_supported(link_remote_twincat PRIVATE -Wno-mismatched-new-delete)

set_target_properties(link_remote_twincat
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

set_target_properties(link_remote_twincat PROPERTIES FOLDER "link")
set_target_properties(link_remote_twincat PROPERTIES POSITION_INDEPENDENT_CODE ON)
