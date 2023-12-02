# Tutorial

## Installation of dependencies

This tutorial uses SOEM.
If you are using Windows, install [Npcap](https://npcap.com/) in **WinPcap API-compatible Mode**.

## Setup devices

First, set up the devices.
Here, we assume that only one AUTD3 is used.
Connect the PC's Ethernet port and the AUTD3 device's EtherCAT In ([Concept](concept.md)) with an Ethernet cable.
Next, connect the $\SI{24}{V}$ power supply.

### Firmware update

If the firmware is old, normal operation is not guaranteed.
The firmware version in this document is assumed to be v4.0.x.

To update the firmware, you need a Windows 10/11 64bit PC with [Vivado](https://www.xilinx.com/products/design-tools/vivado.html) and [J-Link Software](https://www.segger.com/downloads/jlink/) installed.

> Note: If you only want to update the firmware, we strongly recommend using "Vivado Lab Edition".
> The "Vivado ML Edition" requires more than 60 GB of disk space to install, while the "Vivado Lab Edition" requires only about 6 GB.

First, connect the AUTD3 device and the PC with [XILINX Platform Cable](https://www.xilinx.com/products/boards-and-kits/hw-usb-ii-g.html) and [J-Link Plus](https://www.segger.com/products/debug-probes/j-link/models/j-link-plus/) with [J-Link 9-Pin Cortex-M Adapter](https://www.segger-pocjapan.com/j-link-9-pin-cortex-m-adapter), and turn on the power of the AUTD3.

Then, run `firmware/autd_firmware_writer.ps1` in [SDK](https://github.com/shinolab/autd3) on powershell.

## Language-specific tutorials

- [Rust](./getting_started/rust.md)
- [C++](./getting_started/cpp.md)
- [C#](./getting_started/cs.md)
    - [Unity](./getting_started/unity.md)
- [Python](./getting_started/python.md)
