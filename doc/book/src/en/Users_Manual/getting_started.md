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

If the firmware is out of date, the operation is not guaranteed. The version of firmware in this document is assumed to be 2.4.

To update the firmware, [Vivado](https://www.xilinx.com/products/design-tools/vivado.html) and [J-Link Software](https://www.segger.com/downloads/jlink/) on Windows 10/11 64-bit PC are required.

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
        ├─ci
        ├─cmake
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

## Explanation

To use the SDK, include the `autd3.hpp` header.
The `autd3/link/soem.hpp` header is also required to use `link::SOEM`.

```cpp
#include "autd3.hpp"
#include "autd3/link/soem.hpp"
```

Next, create a `Controller`.

```cpp
autd3::Controller autd;
````

Then, specify the device placement.

```cpp
autd.geometry().add_device(autd3::Vector3::Zero(), autd3::Vector3::Zero());
```

The first argument of `add_device` is the position, the second is the rotation. 
The position is the origin of the device in the global coordinate system.
The rotation is specified in ZYZ Euler angles or quaternions. 
In this example, neither rotation nor translation is assumed.

Next, create a `Link` to connect to the device.

```cpp
  auto link = autd3::link::SOEM().high_precision(true).build();
  autd.open(std::move(link));
```

Next, `check_trials` is set to 50. 
You do not need to change this value, but setting it increases reliability. 
For SOEM link, it is recommended to set `check_trials` to about 50.

```cpp
autd.check_trials = 50;
```

Next, initialize the AUTD device.
You may not need to call `clear()` since the devices are initialized at power-up.

```cpp
autd.clear();
````

Next, synchronize the AUTD devices.

```cpp
autd.synchronize();
```

**Even if only one device is used, this function must be called once after initialization.**

Next, we check the firmware version. 

```cpp
const auto firm_infos = autd.firmware_infos();
std::copy(firm_infos.begin(), firm_infos.end(), std::ostream_iterator<autd3::FirmwareInfo>(std::cout,"\n"));
```

Note that if a version other than v2.4 is displayed here, it is not guaranteed to work well.

Next, setup silencer.

```cpp
autd3::SilencerConfig config;
autd.send(config);
````

This is set by default, so you don't really need to call it.
If you want to turn silencer off, use `SilencerConfig::none()`.
The silencer is used to quiet down the transducers by passing the phase/amplitude parameters through a low-pass filter.

Then, we create a `Gain` representing a single focus and a `Modulation` applying a sin wave modulation of $\SI{150}{Hz}$ and sends them to the device.

```cpp
const auto focus = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
autd3::gain::Focus g(focus);
autd3::modulation::Sine m(150);

autd.send(m, g);
```
, where `focus` denotes $\SI{150}{mm}$ directly above the center of the device.

Finally, disconnect the device.

```cpp
autd.close();
```

In the next page, the basic functions will be described. 
[Online API Documentation](https://shinolab.github.io/autd3/api/index.html) is also available.

[^fn_git]: Not required to run, but used to simplify the work.

[^fn_npcap]: Used to use SOEM link. Not necessary for other links.
