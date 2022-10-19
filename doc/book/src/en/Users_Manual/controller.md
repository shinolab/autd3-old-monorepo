# Controller

This section introduces other functions that exist in the `Controller` class.

## Check Trials

If the value of `check_trials` is greater than 0, the controller will check if the data sent to the device has been processed properly.

```cpp
autd.check_trials = 50;
```

If the value of `check_trials` is greater than 0, the function checks up to `check_trials` times in the `send` function.
If it fails, `send` returns `false`.

If the value of `check_trials` is 0, the `send` function does not check and always returns `true`.

It is recommended to set this option when you use unreliable links.
Note that if `check_trials` is greater than 0, the time to send will be increased.

`check_trials` is set to 0 by default.

## Send intervals

The interval between consecutive frames and data check intervals are configured by `send_interval`.
These intervals will be $\SI{500}{\text{μ}s}\times \text{send\_interval}$.

```cpp
autd.send_intervals = 1;
```

It is set to 1 by default.

## Force fan

AUTD3 has a fan to cool the device, and three modes of fan control: Auto, Off and On.
In Auto mode, the temperature monitoring IC automatically activates the fan when the temperature exceeds a certain level. 
In Off mode, the fan is always off, and in On mode, the fan is always on.

The mode is switched by a jumper switch next to the fan.
It is shown in the figure below.
When the fan side is shorted, the mode is Auto.
Off in the middle is shorted, and On on the right side is shorted.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/doc/book/src/fig/Users_Manual/fan.jpg"/>
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

## clear

Clear flags, `Gain`/`Modulation` data, etc. in the device.

## Firmware information

The `firmware_infos` function allows you to get the firmware version information.

```cpp
for (auto&& firm_info : autd.firmware_infos()) std::cout << firm_info << std::endl;
```

## Send functions

Send functions are functions that send data to the device.
Calling these functions will `force fan`, `reads FPGA info` flags are updated by calling these functions.

The behavior of these functions depends on the values of `check_trials` and `send_interval`.

If `check_trials` is greater than 0, these functions wait until the device processes the data.
In particular, the processing time of `Modulation` and `STM` may increase significantly since a check is made for each frame.
If these functions cannot confirm that the device has processed the data, return `false`.
If `check_trials` is 0, these do not check, and the return value is always `true`.

The value of `send_interval` affects the interval of sending consecutive frames and the interval of the above checks. 
Specifically, the interval is $\SI{500}{\text{μ}s}\times \text{send\_interval}$.

The list of send functions is as follows:

- `update_flag`
- `clear`[^fn_clear]
- `close`
- `stop`
- `send`

[^fn_clear]: flags will be also cleared
