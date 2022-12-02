target_sources(test_autd3 PRIVATE
  primitive.cpp
)

if(BUILD_GAIN_HOLO)
  target_sources(test_autd3 PRIVATE
    backend_base.hpp
    backend_eigen.cpp
  )
  target_link_libraries(test_autd3 gain_holo backend_eigen)
  if(BUILD_BACKEND_CUDA)
    target_sources(test_autd3 PRIVATE backend_cuda.cpp)
    target_link_libraries(test_autd3 backend_cuda)
  endif()
  if(BUILD_BACKEND_BLAS)
    target_sources(test_autd3 PRIVATE backend_blas.cpp)
    target_link_libraries(test_autd3 backend_blas)
  endif()
  if(BUILD_BACKEND_ARRAYFIRE)
    target_sources(test_autd3 PRIVATE backend_arrayfire.cpp)
    target_link_libraries(test_autd3 backend_arrayfire)
  endif()
endif()
