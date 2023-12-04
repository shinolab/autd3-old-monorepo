<h1 align="center">
AUTD3: Airborne Ultrasound Tactile Display 3
</h1>

<div align="center">

![build](https://github.com/shinolab/autd3/workflows/build/badge.svg)
![build](https://github.com/shinolab/autd3/workflows/build-cpp/badge.svg)
![build](https://github.com/shinolab/autd3/workflows/build-cs/badge.svg)
![build](https://github.com/shinolab/autd3/workflows/build-python/badge.svg)

</div>

<div align="center">

[![codecov-rust](https://img.shields.io/codecov/c/gh/shinolab/autd3?flag=rust&logo=codecov&label=coverage-rust)](https://codecov.io/gh/shinolab/autd3)
[![codecov-cpp](https://img.shields.io/codecov/c/gh/shinolab/autd3?flag=cpp&logo=codecov&label=coverage-cpp)](https://codecov.io/gh/shinolab/autd3)
[![codecov-cs](https://img.shields.io/codecov/c/gh/shinolab/autd3?flag=cs&logo=codecov&label=coverage-cs)](https://codecov.io/gh/shinolab/autd3)
[![codecov-python](https://img.shields.io/codecov/c/gh/shinolab/autd3?flag=python&logo=codecov&label=coverage-python)](https://codecov.io/gh/shinolab/autd3)

</div>

<div align="center">

[![release](https://img.shields.io/github/v/release/shinolab/autd3)](https://github.com/shinolab/autd3/releases/latest)
[![Crate.io version](https://img.shields.io/crates/v/autd3)](https://crates.io/crates/autd3)
[![NuGet stable version](https://img.shields.io/nuget/v/autd3sharp)](https://nuget.org/packages/AUTD3Sharp)
[![autd3-unity](https://img.shields.io/npm/v/com.shinolab.autd3?label=autd3-unity)](https://www.npmjs.com/package/com.shinolab.autd3)
[![PyPI version](https://img.shields.io/pypi/v/pyautd3)](https://pypi.org/project/pyautd3/)

</div>

<p align="center">
Airborne Ultrasound Tactile Display (AUTD) is a midair haptic device that can remotely produce tactile sensation on a human skin surface without wearing devices.
Please see <a href="https://hapislab.org/en/airborne-ultrasound-tactile-display">our laboratory homepage</a> for more details on AUTD.
This repository contains a client library to drive AUTD version 3 devices.
This cross-platform library supports Windows, macOS, and Linux (including Single Board Computer such as Raspberry Pi).
</p>

> [!WARNING]  
> From v17.0.0, the software is completely incompatible with v2.x and v3.x firmware.
> Before using this library, write the latest firmware in `firmware`. For more information, please see [README](./firmware/README.md).

## Document

* [日本語/Japanese](https://shinolab.github.io/autd3/book/jp)
* [English](https://shinolab.github.io/autd3/book/en)

## Example

* See [examples](./src/examples)

    * If you are using Linux/macOS, you may need to run as root.

## For other programming languages

* [C++](./cpp)
* [C#](./dotnet/cs)
* [F#](./dotnet/fs)
* [Python](./python)

## Citing

* If you use this SDK in your research, please consider including the following citation in your publications:

   * [S. Suzuki, S. Inoue, M. Fujiwara, Y. Makino, and H. Shinoda, "AUTD3: Scalable Airborne Ultrasound Tactile Display," in IEEE Transactions on Haptics, DOI: 10.1109/TOH.2021.3069976.](https://ieeexplore.ieee.org/document/9392322)
   * S. Inoue, Y. Makino and H. Shinoda "Scalable Architecture for Airborne Ultrasound Tactile Display," Asia Haptics 2016

## LICENSE

* See [LICENSE](./LICENSE)

# Author

Shun Suzuki, 2022-2023
