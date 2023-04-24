include(FetchContent)
set(FX_GLTF_BUILD_TESTS OFF)
set(FX_GLTF_INSTALL OFF)
set(FX_GLTF_USE_INSTALLED_DEPS OFF)
FetchContent_Declare(
  fx-gltf
  GIT_REPOSITORY https://github.com/jessey-git/fx-gltf.git
  GIT_TAG v2.0.0)
FetchContent_MakeAvailable(fx-gltf)
