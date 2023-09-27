target_sources(test_autd3 PRIVATE
  amplitudes.cpp
  controller.cpp
  datagram.cpp
  firmware_info.cpp
  fpga_info.cpp
  special.cpp
  stm.cpp
)

add_subdirectory(geometry)
