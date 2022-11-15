# Controller

This section introduces other functions that exist in the `Controller` class.

## Ack Check Timeout

If the value of `ack_check_timeout` is greater than 0, the controller will check if the data sent to the device has been processed properly.

```cpp
autd.set_ack_check_timeout(std::chrono::milliseconds(20));
```

If the value of `ack_check_timeout` is greater than 0, `send` function checks ack until the timeout period elapses.
If it expired, `send` returns `false`.

If the value of `ack_check_timeout` is 0, `send` function does not check ack and always returns `true`.

It is recommended to set this option when you want to send data reliably.
Note that if `ack_check_timeout` is greater than 0, the time to send will be increased.

`ack_check_timeout` is set to 0 by default.

## Send intervals

The interval between consecutive frames and data check intervals are configured by `send_interval`.

```cpp
autd.set_send_interval(std::chrono::milliseconds(1));
```

It is set to $\SI{500}{\text{μ}s}$ by default.

## Force fan

AUTD3 has a fan to cool the device, and three modes of fan control: Auto, Off and On.
In Auto mode, the temperature monitoring IC automatically activates the fan when the temperature exceeds a certain level. 
In Off mode, the fan is always off, and in On mode, the fan is always on.

The mode is switched by a jumper switch next to the fan.
It is shown in the figure below.
When the fan side is shorted, the mode is Auto.
Off in the middle is shorted, and On on the right side is shorted.

<figure>
  <img src="../fig/Users_Manual/fan.jpg"/>
  <figcaption>AUTD Fan jumper switch</figcaption>
</figure>

In Auto mode, the fan is automatically activated when the temperature becomes high.
The `force_fan` flag is used to force the fan to start in Auto mode.
The flag is updated after calling one of [Send functions](#send-functions).

```cpp
autd.force_fan = true;
```

## Read FPGA info

Turn on the `reads_fpga_info` flag so that the device returns the FPGA status.
The flag is updated after calling one of [Send functions](#send-functions).

You can get the FPGA state with the `fpga_info` function.

The state of the FPGA can be obtained with the function.

```cpp
autd.reads_fpga_info = true;
autd.update_flag();
const auto fpga_info = autd.read_fpga_info();
```

The return value of `fpga_info` is `vector` for `FPGAInfo` devices.

## stop

You can stop output with the `stop` function.

```cpp
autd << autd3::stop;
```

## clear

Clear flags, `Gain`/`Modulation` data, etc. in the device.

```cpp
autd << autd3::clear;
```

## Firmware information

The `firmware_infos` function allows you to get the firmware version information.

```cpp
for (auto&& firm_info : autd.firmware_infos()) std::cout << firm_info << std::endl;
```

## Send functions

Send functions are functions that send data to the device.
Calling these functions will `force fan`, `reads FPGA info` flags are updated by calling these functions.

The behavior of these functions depends on the values of `ack_check_timeout` and `send_interval`.

If `ack_check_timeout` is greater than 0, these functions wait until the device processes the data or the timeout period elapses.
In particular, the processing time of `Modulation` and `STM` may increase significantly since a check is made for each frame.
If these functions cannot confirm that the device has processed the data, return `false`.
If `ack_check_timeout` is 0, these do not check, and the return value is always `true`.

The value of `send_interval` affects the interval of sending consecutive frames and the interval of the above checks. 

The list of send functions is as follows:

- `update_flag`
- `<<` operator
- `clear`
- `stop`
- `send`

## Async

You can send data in a non-blocking way by using `send_async` or `autd3::async`;

```cpp
autd3::modulation::Sine m(...);
autd3::gain::Focus g(...);

autd.send_async(std::move(m), std::move(g));
// or
autd << autd3::async <<std::move(m), std::move(g);
```

Note that these functions and operators take an r-value only.
Also, the operation is not guaranteed when synchronous and asynchronous transmissions are mixed.
