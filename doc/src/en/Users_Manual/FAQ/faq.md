# FAQ

[[_TOC_]]

## "No AUTD3 devices found"

- If you use `link::SOEM` on macOS or linux, you need root privileges.

   - On linux, you can bypass this by setting the following privileges with the `setcap` command:
   
      ```shell
      sudo setcap cap_net_raw,cap_net_admin=eip ./examples/example_soem
      ```

- (Windows) Install the latest npcap

- Virtual machines such as WSL are not supported.
   - VirtualBox and other virtual machines may work, but the behavior will be unstable.

## "One ore more slaves are not responding"

- Update the driver
   - If you are using Realtek on Windows, please download latest `Win10 Auto Installation Program (NDIS)` driver from [official site](https://www.realtek.com/ja/component/zoo/category/network-interface-controllers-10-100-1000m-gigabit-ethernet-pci-express-software), and install it.
        - Even if you use Windows 11, you must use NDIS version.

- (Windows) Install the latest npcap.

- Increase the values of `send_cycle` and `sync0_cycle`.

## Frequent send failures when using `link::SOEM`

- This problem occurs when using the onboard ethernet interface, and one of the following situations

   * Using RealSense, Azure Kinect, webcam, etc.
      * Basically, the problem occurs when the camera is activated.
   * Playing a video or audio file.
      * Or, open a video site (e.g. Youtube) with an browser.
   * Using Unity
   * Playing animation in Blender
      * Other operations (modeling, etc.) are fine.

- As a workaround, try one of the following
  1. Use `link::TwinCAT`, `link::RemoteTwinCAT`, or `link::RemoteSOEM`
  1. Use a USB to Ethernet adapter
     - It has been confirmed that at least the adapter using the "ASIX AX88179" chip works properly.
     - The same problem may occur with PCIe ethernet adapters.
  1. Set to `FreeRun` mode
  1. Increase the values of `send_cycle` and `sync0_cycle`
     - In this case, however, the send latency will increase.
  1. Use Linux or macOS.
     - Virtual machines are not acceptable.

## The link is frequently broken.

- If this occurs frequently during ultrasound output, check if there is enough power.
   - A single device consumes up to 50W.

## Error when using `link::RemoteTwinCAT`

- It may be blocked by a firewall, turn off the firewall or allow port 48898 of TCP/UDP.
- Disconnect all client PCs from LAN except for the server.

## Miscellaneous

- Please feel free to ask questions or report bugs to [Issue on Github](https://github.com/shinolab/autd3/issues)
