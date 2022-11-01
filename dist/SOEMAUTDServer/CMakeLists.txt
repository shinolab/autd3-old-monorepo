cmake_minimum_required(VERSION 3.21)

if(EXPORT_SOEM_AUTD_SERVER)
  add_executable(SOEMAUTDServer soem_server.cpp)
  target_include_directories(SOEMAUTDServer PRIVATE ${PROJECT_SOURCE_DIR}/include)
  target_link_libraries(SOEMAUTDServer link_soem)
  set_target_properties(SOEMAUTDServer PROPERTIES FOLDER "publish")
endif()
