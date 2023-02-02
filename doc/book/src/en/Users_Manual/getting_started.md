# Getting Started

This section describe the actual procedure for running AUTD3.
Windows 11 64bit is used in this section, however, it can be run on other operating systems such as Linux and macOS.

## Install

First, install the necessary tools.
The tools and versions used in this section are as follows. 

- Visual Studio Community 2022 17.3.5
- CMake 3.24.2
- git 2.38.0.windows.1[^fn_git]
- npcap 1.71[^fn_npcap] (only for Windows)

Follow the official instructions to install each of them.
For Visual Studio Community 2022, install "Desktop development with C++".

Make sure you setup PATH to use `git` and `cmake` from a terminal. 

## Setup Device

Next, set up the device. 
We will use only one AUTD3 device here.
Connect the ethernet port of the PC to the `EtherCAT In` of the AUTD3 device with an Ethernet cable (see [Concept](concept.md)). Next, connect the $\SI{24}{V}$ power supply.

### Firmware update

If the firmware is out of date, the operation is not guaranteed. The version of firmware in this document is assumed to be 2.8.

To update the firmware, [Vivado](https://www.xilinx.com/products/design-tools/vivado.html) and [J-Link Software](https://www.segger.com/downloads/jlink/) on Windows 10/11 64-bit PC are required.

> NOTE: If you only want to update the firmware, we strongly recommend you to use Vivado Lab edition.

First, connect the AUTD3 device and the PC to [XILINX Platform Cable](https://www.xilinx.com/products/boards-and-kits/hw-usb-ii-g.html), and [J-Link Plus](https://www.segger.com/products/debug-probes/j-link/models/j-link-plus/) with [J-Link 9-Pin Cortex-M Adapter](https://www.segger-pocjapan.com/j-link-9-pin-cortex-m-adapter).
Next, connect AUTD3 to the power supply and power it on.
Next, run `dist/firmware/autd_firmware_writer.ps1` from PowerShell and follow the instructions.
The update will take a few minutes.

## Building first program

First, open a terminal and prepare a directory for the sample.

```
mkdir autd3_sample
cd autd3_sample
```

Then, make `CMakeLists.txt` and `main.cpp` files.

```
└─autd3_sample
        CMakeLists.txt
        main.cpp
```

Next, download the [latest binary of the SDK](https://github.com/shinolab/autd3/releases).
Unzip the downloaded file and copy the `include` and `lib` folders to the `autd3_sample` folder.

```
└─autd3_sample
    │  CMakeLists.txt
    │  main.cpp
    ├─include
    └─lib
```

Next, download Eigen3 library.
Here, to simplify the process, we use git and add Eigen3 as a submodule.

```
git init
git submodule add https://gitlab.com/libeigen/eigen.git eigen
cd eigen
git checkout 3.4.0
cd ..
```

Alternatively, you can download [Eigen3](https://gitlab.com/libeigen/eigen) directly and put it under the `autd3_sample` folder.
The Eigen3 version used in the SDK is 3.4.0.

At this point, the directory structure is as follows.

```
└─autd3_sample
    │  CMakeLists.txt
    │  main.cpp
    ├─include
    ├─lib
    └─eigen
        ├─bench
        ├─blas
        ...
```

Next, make `CMakeLists.txt` as follows,

```
{{#include ../../../samples/cpp/CMakeLists.txt}}
```

And, make `main.cpp` as follows.
This is the source code for generating a focus with $\SI{150}{Hz}$ AM modulation. 

```cpp
{{#include ../../../samples/cpp/main.cpp}}
```

Next, build with CMake.

```
mkdir build
cd build
cmake ..
```

Now, `autd3_sample.sln` should be generated under the build directory.
Open it and execute the main project.
**Note that you must change the build configuration of Visual Studio from Debug to Release when executing the main project.**
Also, if you use Linux/macOS, root privileges may be required to run the main project.

[Online API Documentation](https://shinolab.github.io/autd3/api/index.html) is also available.

[^fn_git]: Not required to run, but used to simplify the work.

[^fn_npcap]: Used to use SOEM link. Not necessary for other links.
