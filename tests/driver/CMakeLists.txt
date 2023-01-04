add_subdirectory(common)
add_subdirectory(driver)

target_sources(test_autd3 PRIVATE
  firmware_version.cpp
  utils.cpp
)
