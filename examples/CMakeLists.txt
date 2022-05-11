cmake_minimum_required(VERSION 3.16)

add_executable(example main.cpp ${hpp_files})
target_link_libraries(example PRIVATE autd3)
target_link_libraries(example PRIVATE link_emulator)
target_link_libraries(example PRIVATE GSL)
set_target_properties(example PROPERTIES FOLDER "example")
