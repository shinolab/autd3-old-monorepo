<h1 align="center">
AUTD3: Airborne Ultrasound Tactile Display 3
</h1>

<div align="center">

![build](https://github.com/shinolab/autd3/workflows/build/badge.svg)
![build](https://github.com/shinolab/autd3/workflows/build-cs/badge.svg)
![build](https://github.com/shinolab/autd3/workflows/build-python/badge.svg)
![build](https://github.com/shinolab/autd3/workflows/build-julia/badge.svg)
![build](https://github.com/shinolab/autd3/workflows/build-rust/badge.svg)

</div>

<div align="center">

[![release](https://img.shields.io/github/v/release/shinolab/autd3)](https://github.com/shinolab/autd3/releases/latest)
[![Crate.io version](https://img.shields.io/crates/v/autd3)](https://crates.io/crates/autd3)
[![NuGet stable version](https://img.shields.io/nuget/v/autd3sharp)](https://nuget.org/packages/AUTD3Sharp)
[![autd3-unity](https://img.shields.io/npm/v/com.shinolab.autd3?label=autd3-unity)](https://www.npmjs.com/package/com.shinolab.autd3)
[![PyPI version](https://img.shields.io/pypi/v/pyautd3)](https://pypi.org/project/pyautd3/)
[![Julia release](https://img.shields.io/github/v/release/shinolab/autd3?label=Julia)](https://github.com/shinolab/autd3/releases/latest)

</div>

<p align="center">
Airborne Ultrasound Tactile Display (AUTD) is a midair haptic device that can remotely produce tactile sensation on a human skin surface without wearing devices.
Please see <a href="https://hapislab.org/en/airborne-ultrasound-tactile-display">our laboratory homepage</a> for more details on AUTD.
This repository contains a client library to drive AUTD version 3 devices.
This cross-platform library supports Windows, macOS, and Linux (including Single Board Computer such as Raspberry Pi).
</p>

## Document

* [日本語/Japanese](https://shinolab.github.io/autd3/book/jp)
* [English](https://shinolab.github.io/autd3/book/en)
    * [API References](https://shinolab.github.io/autd3/api/)

## CAUTION

* Before using, write the latest firmware in `dist/firmware`. 
    * For more information, please see [README](/dist/firmware/README.md).

## Requirements

* If you use `link::SOEM` on Windows, install [Npcap](https://nmap.org/npcap/) with WinPcap API-compatible mode.
* If you use `link::TwinCAT` or `link::RemoteTwinCAT`, please see [how to install TwinCAT](https://shinolab.github.io/autd3/book/en/Users_Manual/link.html#how-to-install-twincat).

## Build

* Pre-built binaries and header files are on the [GitHub Release page](https://github.com/shinolab/autd3/releases). 
* Or, if you want to build from the source, install CMake version 3.21 or higher and follow the instructions below.
    ```
    git clone https://github.com/shinolab/autd3.git
    cd autd3
    mkdir build && cd build
    cmake ..
    cmake --build . --config Release
    ```

    * Some projects are disabled by default. Please enable them by switching their flags on

        * For example, if you want to use TwinCATLink:
            ```
            cmake .. -DBUILD_LINK_TWINCAT=ON
            ```

## Example

* See [examples](./examples)

* If you are using Linux/macOS, you may need to run as root.

## CMake options list

| Option                      | default | description                          |
| -------------------------   | ------- | ------------------------------------ |
| BUILD_ALL                   | OFF     | build all basic options              |
| BUILD_DOC                   | OFF     | build doxygen documents              |
| BUILD_GAIN_HOLO             | ON      | build Holo gain                      |
| BUILD_BACKEND_CUDA          | OFF     | build CUDABackend for Holo gain      |
| BUILD_BACKEND_ARRAYFIRE     | OFF     | build ArrayFireBackend for Holo gain |
| BUILD_BACKEND_BLAS          | OFF     | build BLASBackend for Holo gain      |
| BLAS_LIB_DIR                | OFF     | BLAS lib dir                         |
| BLAS_DEPEND_LIB_DIR         | OFF     | BLAS depends lib dir                 |
| BLAS_INCLUDE_DIR            | OFF     | BLAS include dir                     |
| USE_MKL                     | OFF     | ON if use intel MKL                  |
| BUILD_MODULATION_AUDIO_FILE | OFF     | build Wav and RawPCM modulation      |
| BUILD_LINK_SOEM             | ON      | build SOEM link                      |
| BUILD_LINK_TWINCAT          | OFF     | build TwinCAT link                   |
| BUILD_LINK_REMOTE_TWINCAT   | OFF     | build RemoteTwinCAT link             |
| BUILD_LINK_SIMULATOR        | OFF     | build Simulator link                 |
| BUILD_LINK_BUNDLE           | OFF     | build Bundle link                    |
| BUILD_LINK_DEBUG            | ON      | build Debug link                     |
| BUILD_GEOMETRY_VIEWER       | OFF     | build GeometryViewer                 |
| BUILD_SIMULATOR             | OFF     | build Simulator                      |
| BUILD_EXAMPLES              | ON      | build examples                       |
| BUILD_CAPI                  | OFF     | build C-API                          |
| ENABLE_LINT                 | OFF     | enable cpp lint                      |
| BUILD_TEST                  | OFF     | build tests                          |


## For other programming languages

* [Rust](./rust)
* [C#](./cs)
* [F#](./fs)
* [Python](./python)
* [Nim](./nim)
* [Julia](./julia)
* [MATLAB](./matlab)

## Citing

* If you use this SDK in your research, please consider including the following citation in your publications:

   * [S. Suzuki, S. Inoue, M. Fujiwara, Y. Makino, and H. Shinoda, "AUTD3: Scalable Airborne Ultrasound Tactile Display," in IEEE Transactions on Haptics, DOI: 10.1109/TOH.2021.3069976.](https://ieeexplore.ieee.org/document/9392322)
   * S. Inoue, Y. Makino and H. Shinoda "Scalable Architecture for Airborne Ultrasound Tactile Display," Asia Haptics 2016

## LICENSE

* See [LICENSE](./LICENSE)

# Author

Shun Suzuki, 2022
