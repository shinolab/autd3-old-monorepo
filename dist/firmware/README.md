# Usage (Windows 64bit only)

Make sure that AUTD is connected via appropriate cables and power on.
* FPGA: [XILINX Platform Cable](https://www.xilinx.com/products/boards-and-kits/hw-usb-ii-g.html)
* CPU board: [J-Link Plus](https://www.segger.com/products/debug-probes/j-link/models/j-link-plus/) & [J-Link 9-Pin Cortex-M Adapter](https://www.segger-pocjapan.com/j-link-9-pin-cortex-m-adapter)

Then, run `autd_firmware_writer.ps1` from powershell (this script can only run with Vivado 2019 or later)

## Requirements

Please install following softwares.

* Vivado Design Suite (https://www.xilinx.com/products/design-tools/vivado.html)
    * Tested with Vivado 2022.1
* J-Link Software (https://www.segger.com/downloads/jlink/)
    * Tested with J-Link Software v7.58b (x64)
