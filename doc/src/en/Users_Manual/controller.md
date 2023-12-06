# Controller

The followings are introductino of APIs in `Controller` class.

[[_TOC_]]

## Force fan

AUTD3 device has a fan, and it has three fan modes: Auto, Off, and On.

In Auto mode, the temperature monitoring IC monitors the temperature of the IC, and when it exceeds a certain temperature, the fan starts automatically. 
In Off mode, the fan is always off, and in On mode, the fan is always on. 

The fan mode is switched by the jumper switch next to the fan.
As shown in the figure below, the fan side is shorted to switch to Auto, the center is Off, and the right is On.

<figure>
    <img src="../fig/Users_Manual/fan.jpg"/>
<figcaption>Jumper switch to specify fan mode</figcaption>
</figure>

You can force the fan to start in Auto mode by `ConfigureForceFan`.

```rust,edition2021
{{#include ../../codes/Users_Manual/controller_fan.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/controller_fan.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/controller_fan.cs}}
```

```python
{{#include ../../codes/Users_Manual/controller_fan.py}}
```

## fpga_info

Get the FPGA status.
Before using this, you need to configure reads FPGA info flag by `ConfigureReadsFPGAInfo`.

```rust,edition2021
{{#include ../../codes/Users_Manual/controller_0.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/controller_0.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/controller_0.cs}}
```

```python
{{#include ../../codes/Users_Manual/controller_0.py}}
```

You can get the following information about the FPGA.
- thermal sensor for fan control is asserted or not

## send

Send the data to the device.

You can send a single or two data at the same time.
However, `Stop` is an exception and can only be sent alone.

### Timeout

You can specify the timeout time with `with_timeout`.
If you omit this, the timeout time set by [Link](./link.md) will be used.

```rust,edition2021
{{#include ../../codes/Users_Manual/controller_1.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/controller_1.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/controller_1.cs}}
```

```python
{{#include ../../codes/Users_Manual/controller_1.py}}
```

If the timeout time is greater than 0, the `send` function waits until the sent data is processed by the device or the specified timeout time elapses.
If it is confirmed that the sent data has been processed by the device, the `send` function returns `true`, otherwise it returns `false`.

If the timeout time is 0, the `send` function does not check whether the sent data has been processed by the device or not.

If you want to data to be sent surely, it is recommended to set this to an appropriate value.

### Stop

You can stop the output by sending `Stop` data.

Note that the `Stop` data resets the settings of Silencer.

### Clear

You can clear the flags and `Gain`/`Modulation` data in the device by sending `Clear` data.

## group

You can group the devices by using `group` function, and send different data to each group.

```rust,edition2021
{{#include ../../codes/Users_Manual/controller_2.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/controller_2.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/controller_2.cs}}
```

```python
{{#include ../../codes/Users_Manual/controller_2.py}}
```

Unlike `gain::Group`, you can use any data that can be sent with `send` as a value.
However, you can only group by device.

> NOTE:
> This sample uses a string as a key, but you can use anything that can be used as a key for HashMap.
