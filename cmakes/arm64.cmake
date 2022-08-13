set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR aarch64)

set(CMAKE_CROSSCOMPILING TRUE)
if (NOT DEFINED TARGET_ABI)
  set(TARGET_ABI "linux-gnu")
endif()
set(CMAKE_C_COMPILER /usr/bin/aarch64-${TARGET_ABI}-gcc)
set(CMAKE_CXX_COMPILER /usr/bin/aarch64-${TARGET_ABI}-g++)
set(CMAKE_LINKER ${TARGET_ABI}-gcc)
SET(CMAKE_FIND_ROOT_PATH "/usr/aarch64-${TARGET_ABI}")
set(CMAKE_FIND_ROOT_PATH_MODE_PROGRAM NEVER)
set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
