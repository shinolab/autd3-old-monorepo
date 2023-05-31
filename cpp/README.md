# autd3-cpp

[autd3 library](https://github.com/shinolab/autd3) for C++17.

version: 10.0.1

## Install

- This library can be installed with CMake. e.g.,
    ```
    include(FetchContent)
    if(WIN32)
    FetchContent_Declare(
        autd3
        URL https://github.com/shinolab/autd3/releases/download/v10.0.1/autd3-v10.0.1-win-x64.zip
    )
    elseif(APPLE)
    FetchContent_Declare(
        autd3
        URL https://github.com/shinolab/autd3/releases/download/v10.0.1/autd3-v10.0.1-macos-universal.tar.gz
    )
    else()
    FetchContent_Declare(
        autd3
        URL https://github.com/shinolab/autd3/releases/download/v10.0.1/autd3-v10.0.1-linux-x64.tar.gz
    )
    endif()
    FetchContent_MakeAvailable(autd3)

    target_link_libraries(<target> <PRIVATE|PUBLIC|INTERFACE> autd3)
    ```

## Build example

```
git clone https://github.com/shinolab/autd3.git
cd autd3/cpp/examples
mkdir build
cd build
cmake ..
cmake --build . --config Release
```

## LICENSE

MIT

# Author

Shun Suzuki, 2022-2023
