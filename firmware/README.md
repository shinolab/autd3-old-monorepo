# Usage (Windows 64bit only)

## Requirements

Please install following softwares.

* Vivado Lab edition or ML edition (https://www.xilinx.com/products/design-tools/vivado.html)
    * If you only want to update the firmware, we strongly recommend you to use Lab edition
        * Lab edition requires only about 6GB of disk space, while ML edition requires more than 60GB
    * Tested with Vivado 2022.2
* J-Link Software (https://www.segger.com/downloads/jlink/)
    * Tested with J-Link Software v7.58b (x64)

Also, the following cables are required

* FPGA: [XILINX Platform Cable](https://www.xilinx.com/products/boards-and-kits/hw-usb-ii-g.html)
* CPU board: [J-Link Plus](https://www.segger.com/products/debug-probes/j-link/models/j-link-plus/) & [J-Link 9-Pin Cortex-M Adapter](https://www.segger-pocjapan.com/j-link-9-pin-cortex-m-adapter)

# Update

Make sure that AUTD is connected via appropriate cables and power on. Then, run `autd_firmware_writer.ps1` from powershell.
