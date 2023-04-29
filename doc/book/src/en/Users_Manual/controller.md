# Controller

This section introduces functions that exist in the `Controller` class.

## open/close/is_open

Open and close `Controller`.

You can check if the `Controller` is open by `is_open`.

## geometry

Get `Geometry`.

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

```cpp
autd.force_fan(true);
```

The flag is updated after calling `send`.
If you only want to update the flag, send `UpdateFlag`.

```cpp
autd.force_fan(true);
autd.send(autd3::UpdateFlag());
```

## Read FPGA info

Turn on the `reads_fpga_info` flag so that the device returns the FPGA status.

You can get the FPGA state with the `fpga_info` function.

```cpp
autd.reads_fpga_info(true);
autd.send(autd3::update_flag());

const auto infos = autd.fpga_info();
```

The return value of `fpga_info` is `vector` for `FPGAInfo` devices.


## Firmware information

The `firmware_infos` function allows you to get the firmware version information.

```cpp
for (auto&& firm_info : autd.firmware_infos()) std::cout << firm_info << std::endl;
```

## Send functions

Send data to devices.

### Timeout

Timeout of `send` function can be specified by the last argument.
If this argument is omitted, the timeout set in [Link](./link.md) is used.

```cpp
autd.send(..., autd3::Milliseconds(20));
```

If the timeout is greater than zero, wait until the data to be sent has been processed by the device or until the specified timeout period has elapsed.
The `send` function returns `true` if it is sure that the device has processed the sent data, otherwise it returns `false`.

If the timeout value is zero, the `send` function does not check data and return void.

It is recommended to set this to an appropriate value if you want to send data reliably.

### stop

You can stop output by sending `autd3::Stop`.

```cpp
autd.send(autd3::Stop());
```

Note that this will overwrite `SilencerConfig` with its default value.

### clear

You can clear flags, `Gain`/`Modulation` data, etc. in the device by sending `autd3::Clear`.

```cpp
autd.send(autd3::Clear());
```
