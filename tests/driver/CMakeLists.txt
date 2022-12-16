add_subdirectory(common)
add_subdirectory(v2_2)
add_subdirectory(v2_3)
add_subdirectory(v2_4)
add_subdirectory(v2_5)
add_subdirectory(v2_6)
add_subdirectory(v2_7)

target_sources(test_autd3 PRIVATE
  firmware_version.cpp
  utils.cpp
)
