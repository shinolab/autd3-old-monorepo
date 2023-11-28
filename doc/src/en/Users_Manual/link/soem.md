# SOEM

[SOEM](https://github.com/OpenEtherCATsociety/SOEM) is an open source EtherCAT master library developed by volunteers.
Unlike TwinCAT, it runs on a regular Windows PC, so real-time performance is not guaranteed.
Therefore, it is recommended to use TwinCAT.
SOEM should be used only if there is no other choice or only during development.
On the other hand, SOEM is cross-platform and easy to install.

If you are using Windows, install [npcap](https://nmap.org/npcap/) in **WinPcap API compatible mode**.
If you are using Linux/macOS, no special preparation is required.

> NOTE: If you are using `SOEM`, be aware that it takes about 10-20 seconds after opening `Controller` for the EtherCAT slaves to synchronize with each other.
> This period is subject to individual differences and changes depending on the synchronization signal/transmission cycle.
> During this period, the ultrasound synchronization between devices is not guaranteed.

[[_TOC_]]

## SOEM link API

Following options can be specified for SOEM link.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/soem_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/soem_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/soem_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/soem_0.py}}
```

- `ifname`: Network interface name. The default is blank, and if it is blank, the network interface to which the AUTD3 device is connected is automatically selected.
- `buf_size`: Send queue buffer size. Usually, you don't need to change it.
- `on_err`: Callback when some error occurs. The callback function takes an error message as an argument.
- `state_check_interval`: Interval to check if there is an error. The default is $\SI{100}{ms}$.
- `on_lost`: Callback when an unrecoverable error (e.g., cable is disconnected) occurs.[^fn_soem_err] The callback function takes an error message as an argument.
- `sync0_cycle`: Synchronization signal cycle. The default is 2 (unit is $\SI{500}{us}$).
- `send_cycle`: Send cycle. The default is 2 (unit is $\SI{500}{us}$).
    - `SOEM` may become unstable when a large number of devices are connected[^fn_soem]. In this case, increase the values of `sync0_cycle` and `send_cycle`. These values should be as small as possible without causing errors. The default is 2, and the value depends on the number of devices connected. For example, if there are 9 devices, set the value to 3 or 4.
- `timer_strategy`: Timer strategy. The default is `Sleep`.
    - `Sleep`       : Use standard library sleep
    - `BusyWait`    : Use busy wait. High resolution but high CPU load.
    - `NativeTimer` : Use OS timer function
        - TimerQueueTimer on Windows, POSIX timer on linux, Grand Central Dispatch Timer on macOS
- `sync_mode`: Synchronization mode. See [Beckhoff's explanation](https://infosys.beckhoff.com/english.php?content=../content/1033/ethercatsystem/2469122443.html&id=) for details.

# RemoteSOEM

This link is used to separate the server PC running `SOEM` and the client PC running the user program.

To use `RemoteSOEM`, you need to prepare two PCs.
In this case, one PC must be able to use the `SOEM` link.
This PC is called the "server" here.
On the other hand, there are no particular restrictions on the PC on the development side that uses the SDK, and it is sufficient to be connected to the same LAN as the server.
This is called the "client" here.

First, connect the server and the AUTD device.
Then, connect the server and the client on different LANs[^fn_remote_soem].
Then, check the IP of the LAN between the server and the client.
For example, suppose the server is "172.16.99.104", and the client is "172.16.99.62".

## AUTD Server

To use `RemoteSOEM`, install `AUTD Server` first.
The AUTD server's installer is distributed on [GitHub Releases](https://github.com/shinolab/autd3/releases).

When you run `AUTD Server`, the following screen will appear, so open the "SOEM" tab.

<figure>
  <img src="../../fig/Users_Manual/autdserver_remotesoem.jpg"/>
</figure>

Set port number and click "Run" button.

## RemoteSOEM link API

`RemoteSOEM` constructor takes <server ip address:port> as an argument.

```rust,should_panic,edition2021
{{#include ../../../codes/Users_Manual/link/remote_soem_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/link/remote_soem_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/link/remote_soem_0.cs}}
```

```python
{{#include ../../../codes/Users_Manual/link/remote_soem_0.py}}
```

## SOEMAUTDServer

You can set options for `SOEM` with the option argument of `SOEMAUTDServer`.
Please see `SOEMAUTDServer --help` for details.

## Firewall

If you get a TCP-related error when using `RemoteSOEM`, it may be blocked by the firewall.

[^fn_soem]: It is looser than TwinCAT, and sometimes it works normally.

[^fn_soem_err]: It is unrecoverable, so there is nothing you can do but terminate it immediately.

[^fn_remote_soem]: It can be used even with wireless LAN.
