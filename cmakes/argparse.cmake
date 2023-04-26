include(FetchContent)
set(ARGPARSE_INSTALL OFF)
FetchContent_Declare(
  argparse
  GIT_REPOSITORY https://github.com/p-ranav/argparse.git
  GIT_TAG v2.9)
FetchContent_MakeAvailable(argparse)
