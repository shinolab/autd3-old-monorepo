cmake_minimum_required(VERSION 3.16)

file(GLOB_RECURSE include_files ${PROJECT_SOURCE_DIR}/include/*.hpp ${PROJECT_SOURCE_DIR}/include/autd3/*.hpp)
file(GLOB_RECURSE core_include_files ${PROJECT_SOURCE_DIR}/include/autd3/core/*.hpp)
file(GLOB_RECURSE driver_include_files ${PROJECT_SOURCE_DIR}/include/autd3/driver/*.hpp)

set(SOURCE_LIST
  controller.cpp
  ${include_files}
  ${core_include_files}
  ${driver_include_files}
)

add_library(autd3 ${SOURCE_LIST})
target_include_directories(autd3 PUBLIC ${PROJECT_SOURCE_DIR}/include ${EIGEN_PATH} ${BOOST_LIBRALIES})
target_link_libraries(autd3 PUBLIC GSL)
if(WIN32)
  target_link_libraries(autd3 ${AUTD_LINK_LIBRARIES_KEYWORD} winmm)
endif()

set_target_properties(autd3
  PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
  RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

if (NOT WIN32)
  set_target_properties(autd3 PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()
