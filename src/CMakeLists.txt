cmake_minimum_required(VERSION 3.16)

add_library(autd3 INTERFACE)
target_include_directories(autd3 INTERFACE ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH})
