cmake_minimum_required(VERSION 3.16)

add_executable(example main.cpp ${hpp_files})
target_link_libraries(example autd3)
set_target_properties(example PROPERTIES FOLDER "example")
