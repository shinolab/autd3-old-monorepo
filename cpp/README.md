# autd3-cpp

[autd3 library](https://github.com/shinolab/autd3) for C++17.

version: 14.2.1

## Install

- This library can be installed with CMake. e.g.,
    ```
    include(FetchContent)
    if(WIN32)
    FetchContent_Declare(
        autd3
        URL https://github.com/shinolab/autd3/releases/download/v14.2.1/autd3-v14.2.1-win-x64.zip
    )
    elseif(APPLE)
    FetchContent_Declare(
        autd3
        URL https://github.com/shinolab/autd3/releases/download/v14.2.1/autd3-v14.2.1-macos-universal.tar.gz
    )
    else()
    FetchContent_Declare(
        autd3
        URL https://github.com/shinolab/autd3/releases/download/v14.2.1/autd3-v14.2.1-linux-x64.tar.gz
    )
    endif()
    FetchContent_MakeAvailable(autd3)

    autd3_target_link_library(<target> <PRIVATE|PUBLIC|INTERFACE>)
    ```

    - Or, you can add `autd3::autd3` library by `target_link_libraries`. In this case, however, Windows user must copy dlls to a location in the PATH or the executable's folder to find dlls.

## Build example

- Windows

    ```
    git clone https://github.com/shinolab/autd3.git
    cd autd3/cpp
    pwsh build.ps1
    cd examples
    mkdir build
    cd build
    cmake .. -DAUTD_LOCAL_TEST=ON
    cmake --build . --config Release
    ```

- Linux/macOS

    ```
    git clone https://github.com/shinolab/autd3.git
    cd autd3/cpp
    chmod +x ./build.sh
    ./build.sh
    cd examples
    mkdir build
    cd build
    cmake .. -DAUTD_LOCAL_TEST=ON
    cmake --build . --config Release
    ```

## LICENSE

MIT

# Author

Shun Suzuki, 2022-2023
