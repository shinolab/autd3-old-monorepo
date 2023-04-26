cmake_minimum_required(VERSION 3.21)

if(NOT EXPORT_AUTD_SIMULATOR)
  return()
endif()

include(${PROJECT_SOURCE_DIR}/cmakes/json.cmake)
include(${PROJECT_SOURCE_DIR}/cmakes/tinycolormap.cmake)

add_executable(autd_simulator autd_simulator.cpp)
target_include_directories(autd_simulator PRIVATE ${PROJECT_SOURCE_DIR}/include)
target_link_libraries(autd_simulator PRIVATE autd3::common autd3::extra::simulator nlohmann_json::nlohmann_json tinycolormap)
set_target_properties(autd_simulator PROPERTIES FOLDER "publish")
