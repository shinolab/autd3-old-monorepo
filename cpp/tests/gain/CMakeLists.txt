target_sources(test_autd3 PRIVATE
  bessel.cpp
  cache.cpp
  focus.cpp
  gain.cpp
  group.cpp
  null.cpp
  plane.cpp
  trans_test.cpp
  transform.cpp
  uniform.cpp
)

add_subdirectory(holo)
