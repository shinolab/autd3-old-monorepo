add_subdirectory(common)
add_subdirectory(operation)

target_sources(test_autd3 PRIVATE
  firmware_version.cpp
  utils.cpp
)
