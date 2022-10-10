cmake_minimum_required(VERSION 3.16)

if(EXPORT_AUTD_SIMULATOR)
  add_executable(autd_simulator autd_simulator.cpp)
  target_include_directories(autd_simulator PRIVATE ${PROJECT_SOURCE_DIR}/include)
  target_link_libraries(autd_simulator simulator)
  set_target_properties(autd_simulator PROPERTIES FOLDER "publish")
endif()
