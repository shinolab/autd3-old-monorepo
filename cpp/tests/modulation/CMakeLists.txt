target_sources(test_autd3 PRIVATE
  cache.cpp
  fourier.cpp
  modulation.cpp
  radiation_pressure.cpp
  sine_legacy.cpp
  sine.cpp
  square.cpp
  static.cpp
  transform.cpp
)

add_subdirectory(audio_file)
