# Link

`Link` is an interface to the device.
You need to select one of the following.

[[_TOC_]]

## TwinCAT

TwinCAT is the official way to use EherCAT on a PC.
TwinCAT is a very special software that supports only Windows and makes Windows realtime.

TwinCAT also requires a specific network controller, please see [the list of supported network controllers](https://infosys.beckhoff.com/english.php?content=../content/1033/tc3_overview/9309844363.html&id=).

> Note: Alternatively, after installation of TwinCAT, you can check the Vendor ID and Device ID of the supported device in `C:/TwinCAT/3.1/Driver/System/TcI8254x.inf`.

Non-supported network controllers may also work, but in such cases, normal operation and real-time performance are not guaranteed.

### How to install TwinCAT

First of all, TwinCAT cannot coexist with Hyper-V or Virtual Machine Platform.
Therefore, it is necessary to disable these features.
To do so, for example, run PowerShell with administrator privileges, and then run the following command,

```
Disable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V-Hypervisor
Disable-WindowsOptionalFeature -Online -FeatureName VirtualMachinePlatform
```

Second, download TwinCAT XAE from [official website](https://www.beckhoff.com/en-en/).
You need to register to download the software (free of charge).

Run the downloaded installer and follow the instructions.
**At this time, check "TwinCAT XAE Shell install" and uncheck "Visual Studio Integration".**

After installation, reboot the PC, run `C:/TwinCAT/3.1/System/win8settick.bat` with administrator privileges, and reboot again.

Finally, copy `dist/TwinCATAUTDServer/AUTD.xml` in the SDK to `C:/TwinCAT/3.1/Config/Io/EtherCAT`.

### TwinCATAUTDServer

To use TwinCAT Link, you must run `dist/TwinCATAUTDServer/TwinCATAUTDServer.exe` before using SDK.

For the first time, leave the TwinCAT XAE Shell open with the `-k` option to install the driver.

```
TwinCATAUTDServer.exe -k
```

> Note: If you have closed it, you can open it by starting `%TEMP%/TwinCATAUTDServer/TwinCATAUTDServer.sln` as TcXaeShell Application, where `%TEMP%` is an environment variable, usually `C:/Users/(user name)/AppData/Local/Temp`.

Note that the TwinCATAUTDServer will lose the link when you turn off your PC, enter sleep mode, etc., so you should re-run it each time.

#### Install Driver

The first time, you will need to install the driver for EherCAT.
From the top menu of the TwinCAT XAE Shell, select `TwinCAT->Show Realtime Ethernet Compatible Devices`, and select a compatible device in the `Compatible devices list`, and click `Install`. 
If you can see the adapter name in `Installed and ready to use devices (realtime capcble)`, you have successfully completed the installation.

If nothing is shown in `Compatible devices`, TwinCAT does not support the ethernet device of your PC.
The drivers in `Incompatible devices` can be installed, and the adapter listed in `Installed and ready to use devices (for demo use only)` after installation.
In this case, the driver can be used but is not guaranteed to work.

#### License

In addition, since you will get a license-related error the first time, open `Solution Explorer->SYSTEM->License` in the XAE Shell, click `7 Days Trial License ...` and then enter the letters shown on the screen.

Note that the license is a 7-day trial license, but it can be reissued by doing the same procedure again when the license expires.
After issuing the license, close TwinCAT XAE Shell and run `TwinCATAUTDServer.exe` again.

### Troubleshooting

When you try to use many devices, you may get an error like the one shown in the figure below.

<figure>
  <img src="../fig/Users_Manual/tcerror.jpg"/>
  <figcaption>TwinCAT error when using 9 devices</figcaption>
</figure>

In this case, increase the values of `-s` and `-t` of the `TwinCATAUTDServer` option and run TwinCATAUTDServer again.
The values of these options are 2 by default, respectively.
For proper operation, increase their values.

```
TwinCATAUTDServer.exe -s 3 -t 3
```

The value you should set depends on the number of connected devices.
The value should be as small as possible without causing errors.
For example, if you have nine devices, it should work if you set the value 3 or 4.

## RemoteTwinCAT

As mentioned above, using AUTD3 and TwinCAT requires a Windows OS and a specific network adapter.
However, there are many cases in which you want to develop on non-Windows PCs ([SOEM](#soem), described below, also runs on cross-platform).
In such cases, you can use RemoteTwinCAT link to control TwinCAT remotely.

When using RemoteTwinCAT, you need to prepare two PCs.
One of the PCs must be able to use the above TwinCAT link.
Here, let's call this PC "server".
On the other hand, the PC on the development side, i.e., the one to use the SDK, has no restrictions, but must be connected to the same LAN as the server.
Let's call this PC "client".

<figure>
  <img src="../fig/Users_Manual/remotetwincat.jpg"/>
  <figcaption>Network Configuration</figcaption>
</figure>

First, connect the server to the AUTD device.
The LAN adapter used in this case must be a TwinCAT-compatible adapter.
Also, connect the server to the client on another LAN.
This client-server LAN adapter does not need to be TwinCAT-compatible[^fn_remote_twin].
Then, check the IP address of the LAN between the server and the client.
For example, let us assume that the server IP is "169.254.205.219" and the client IP is "169.254.175.45" here.
Next, start `TwinCATAUTDServer` on the server.
At this time, specify the IP address of the client (`-169.254.175.45` in this example) by `-c` option.
And, use the `-k` option to keep `TwinCATAUTDServer` open.

```
TwinCATAUTDServer.exe -c 169.254.175.45 -k
```

Then, open `System→Routes` and check the AMS NetId of the server in the `NetId Management` tab, as shown in the following figure.

<figure>
  <img src="../fig/Users_Manual/NetId_Management.jpg"/>
  <figcaption>Server AmsNetId</figcaption>
</figure>

Here, the value is assumed to be "172.16.99.194.1.1".
In this case, you can use RemoteTwinCAT link as follows:

```cpp
#include "autd3/link/remote_twincat.hpp"

...

  const std::string server_ams_net_id = "172.16.99.194.1.1";
  auto link = autd3::link::RemoteTwinCAT(server_ams_net_id).build();
```

In some cases, you may need to specify the IP address of the server and the AMS NetId of the client on the client side.
In this case, specify them as follows: 

```cpp
  const std::string server_ip_address = "169.254.205.219";
  const std::string server_ams_net_id = "172.16.99.194.1.1";
  const std::string client_ams_net_id = "169.254.175.45.1.1";
  auto link = autd3::link::RemoteTwinCAT(server_ams_net_id)
    .server_ip_address(server_ip_address)
    .client_ams_net_id(client_ams_net_id)
    .build();
```

The AMS NetId of the client can be found in TwinCAT by going to `System→Routes` and looking at AmsNetId in the `Current Route` tab, as shown in the following figure.

<figure>
  <img src="../fig/Users_Manual/Current_Route.jpg"/>
  <figcaption>Client AmsNetId</figcaption>
</figure>

### Firewall

If you get TCP-related errors, it is possible that your firewall is blocking the ADS protocol.
In such a case, you should allow the connection to port 48898 of TCP/UDP in your firewall configuration.

## SOEM

[SOEM](https://github.com/OpenEtherCATsociety/SOEM) is an open source EherCAT Master library.
Unlike TwinCAT, SOEM runs on normal Windows/Linux/macOS, so real-time performance is not guaranteed.
Therefore, basically, it is recommended to use TwinCAT.
SOEM should be used only for unavoidable reasons or during development.
On the other hand, SOEM has the advantage of being cross-platform and simple to install.

For Windows, install [npcap](https://nmap.org/npcap/) with **WinPcap API compatible mode**.
For Linux/macOS, no special preparation is required.

To use SOEM link, build with the `BUILD_LINK_SOEM` flag ON and link `autd3::link::soem`library.

When you use Link of SOEM, include the `autd3/link/soem.hpp` header.

```cpp
#include "autd3/link/soem.hpp"

...
  auto link = autd3::link::SOEM().build();
```

### interface name

You can specify the network interface to which the AUTD3 device is connected with `ifname`.

```cpp
  auto link = autd3::link::SOEM().ifname("interface name").build();
```

The default is blank, and if blank, the network interface to which the AUTD3 device is connected is automatically selected.

The list of available network interfaces can be obtained with the `enumerate_adapters` function.
```cpp
  auto adapters = autd3::link::SOEM::enumerate_adapters();
```

### Callbacks on link disconnection

The ``on_lost`` function allows you to set up a callback in case of a non-recoverable error (for example, cable disconnection)[^fn_soem_err].
The callback function takes an error message as an argument.

```cpp
  auto link = autd3::link::SOEM()
                .on_lost([](const std::string& msg) {
                  std::cerr << "Link is lost\n";
                  std::cerr << msg;
                  std::quick_exit(-1);
                })
                .build();
```

### Synchronous signal/transmit cycle

The behavior of `SOEM` may also become unstable when a large number of devices are connected[^fn_soem].
In this case, increase the `sync0_cycle` and `send_cycle` values.
```cpp
  auto link = autd3::link::SOEM()
                .sync0_cycle(3)
                .send_cycle(3)
                .build();
```
This value should be as small as possible while not causing errors.
The value depends on the number of connected devices.
For example, if you have 9 devices connected, a value of 3 or 4 should work.

The default is 2.

### Timer strategy

EtherCAT works by sending frames cyclically at a certain interval.
`timer_strategy` specifies how this periodic transmission is performed.

* Sleep       : Using sleep function in standard library
* BusyWait    : Using busy waiting loop. High resolution, but high CPU load
* NativeTimer : Using OS native timer
  * Multimedia Timer on Windows, POSIX timer on linux, Grand Central Dispatch Timer on macOS

```cpp
  auto link = autd3::link::SOEM()
                .timer_strategy(autd3::TimerStrategy::Sleep)
                .build();
```

The default is `Sleep`.

### Synchronization mode

Set the synchronization mode of EtherCAT.
There are two synchronization modes: `DC` and `FreeRun`.

* See [Beckhoff's description](https://infosys.beckhoff.com/english.php?content=../content/1033/ethercatsystem/2469122443.html&id=) for more detauls.

```cpp
  auto link = autd3::link::SOEM()
                .sync_mode(autd3::SyncMode::FreeRun)
                .build();
```

The default is `FreeRun`.

## RemoteSOEM

RemoteSOEM Link can be used to separate the server PC running SOEM from the client PC running the user program.

To use the RemoteSOEM, you need to prepare two PCs.
In this case, one of the PCs must be able to use the SOEM link.
Let us call this PC "server" here.
On the other hand, the PC on the development side, i.e., the one to use the SDK, has no restrictions; and should be connected to the same LAN as the server, and is called the "client" here.

First, connect the server to the AUTD device.
Second, connect the server and the client with another LAN [^fn_remote_twin].
Third, check the IP address of the LAN between the server and the client.
For example, assume that the server is "169.254.205.219" and the client is "169.254.175.45".
Next, start `SOEMAUTDServer` on the server.
At this time, please set port number by `-p` option.

On the client side, include the `autd3/link/remote_soem.hpp` header, and build link as follows;

```cpp
#include "autd3/link/remote_soem.hpp"

...

  const std::string ip = "169.254.205.219";
  const uint16_t port = 0 ; // port of SOEMAUTDServer
  auto link = autd3::link::RemoteSOEM(ip, port).build();
```

### Firewall

If you get a TCP-related error, it may be that your firewall is blocking the connection.
In this case, you should allow TCP/UDP connections on the specified port in the firewall configuration.

## Simulator

Simulator link is a link to use [AUTD Simulator](https://shinolab.github.io/autd3/book/en/Simulator/simulator.html).

Before using this link, you need to start AUTD Simulator.

When you use the Simulator link, include the ``autd3/link/simulator.hpp` header.

```cpp
#include "autd3/link/simulator.hpp"

...

  auto link = autd3::link::Simulator().build();
```

[^fn_remote_twin]: Wireless LAN is also acceptable.

[^fn_soem]: More lax than TwinCAT, and may work normally.

[^fn_soem_err]: Note that there is nothing you can do except terminate the program immediately because it is unrecoverable.
