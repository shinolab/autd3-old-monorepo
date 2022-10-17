cmake_minimum_required(VERSION 3.21)

add_library(autd3 INTERFACE)
target_link_libraries(autd3 INTERFACE autd3_common)
