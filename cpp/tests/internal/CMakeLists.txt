target_sources(test_autd3 PRIVATE
  controller.cpp
  datagram.cpp
  firmware_info.cpp
  fpga_info.cpp
  special.cpp
  stm.cpp
  emit_intensity.cpp
  sampling_config.cpp
  rotation.cpp
)

add_subdirectory(geometry)
