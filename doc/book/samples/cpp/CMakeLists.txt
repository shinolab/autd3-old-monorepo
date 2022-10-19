cmake_minimum_required(VERSION 3.21)

project(autd3_sample)

set (CMAKE_CXX_STANDARD 17)

add_executable(main main.cpp)

target_link_directories(main PRIVATE lib)
target_include_directories(main PRIVATE include eigen)
target_link_libraries(main link_soem)

if(WIN32)
  target_link_directories(main PRIVATE lib/wpcap)
  target_link_libraries(main Packet.lib wpcap.lib ws2_32.lib winmm.lib)
elseif(APPLE)
  target_link_libraries(main pcap)
else()
  target_link_libraries(main rt)
endif()
