include(FetchContent)
set(GLFW_BUILD_EXAMPLES OFF)
set(GLFW_BUILD_TESTS OFF)
set(GLFW_BUILD_DOCS OFF)
set(GLFW_INSTALL OFF)
FetchContent_Declare(
  glfw
  GIT_REPOSITORY https://github.com/glfw/glfw.git
  GIT_TAG 3.3.8)
FetchContent_MakeAvailable(glfw)

set_target_properties(glfw PROPERTIES FOLDER "3rdparty")
set_target_properties(update_mappings PROPERTIES FOLDER "3rdparty")
