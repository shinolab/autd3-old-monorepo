include(FetchContent)
set(SPDLOG_BUILD_EXAMPLE OFF)
set(SPDLOG_BUILD_TESTS OFF)
set(SPDLOG_BUILD_BENCH OFF)
FetchContent_Declare(
  spdlog
  GIT_REPOSITORY https://github.com/gabime/spdlog.git
  GIT_TAG v1.11.0)
FetchContent_MakeAvailable(spdlog)
set_target_properties(spdlog PROPERTIES FOLDER "3rdparty")
