# Controller

The followings are introductino of APIs in `Controller` class.

[[_TOC_]]

## force_fan

AUTD3 has a fan to cool the device.
The fan has three modes, Auto, Off, and On.
With Auto mode, the fan is automatically turned on when the temperature monitoring IC monitors the temperature of the IC and the temperature exceeds a certain temperature.
With Off mode, the fan is always off, and with On mode, the fan is always on.

The switching of the fan mode is done by a jumper switch next to the fan.
If you short the fan side as shown in the following figure, it will be Auto, Off in the middle, and On on the right.

<figure>
  <img src="../fig/Users_Manual/fan.jpg"/>
  <figcaption>Jumper switch</figcaption>
</figure>

In Auto mode, the fan starts automatically when the temperature rises.
You can also force the fan to start by setting the `force_fan` flag.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
autd.force_fan(true);
# Ok(())
# }
```

```cpp
autd.force_fan(true);
```

```cs
autd.ForceFan(true);
```

```python
autd.force_fan(True)
```

The flag is updated when `send` is called and some data is sent.
If you only want to update the flag, you can send `UpdateFlags`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
autd.force_fan(true);
autd.send(UpdateFlags::new())?;
# Ok(())
# }
```

```cpp
autd.force_fan(true);
autd.send(autd3::UpdateFlags());
```

```cs
autd.ForceFan(true);
autd.Send(new UpdateFlags());
```

```python
autd.force_fan(True)
autd.send(UpdateFlags())
```

## fpga_info

Get the FPGA status.
Before using this, you need to set the `reads_fpga_info` flag.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
autd.reads_fpga_info(true);
autd.send(UpdateFlags::new())?;

let info = autd.fpga_info();
# Ok(())
# }
```

```cpp
autd.reads_fpga_info(true);
autd.send(autd3::UpdateFlags());

const auto info = autd.fpga_info();
```

```cs
autd.ReadsFPGAInfo(true);
autd.Send(new UpdateFlags());

var info = autd.FPGAInfo;
```

```python
autd.reads_fpga_info(True)
autd.send(UpdateFlags())

info = autd.fpga_info
```

You can get the following information about the FPGA.
- thermal sensor for fan control is asserted or not

## send

Send the data to the device.

### Timeout

You can specify the timeout time with `with_timeout`.
If you omit this, the timeout time set by [Link](./link.md) will be used.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
# let m = Static::new();
# let g = Null::new();
autd.send((m, g).with_timeout(std::time::Duration::from_millis(20)))?;
# Ok(())
# }
```

```cpp
autd.send(m, g, std::chrono::milliseconds(20));
```

```cs
autd.Send((m, g), TimeSpan.FromMilliseconds(20));
```

```python
autd.send((m, g), timeout=timedelta(milliseconds=20))
```

If the timeout time is greater than 0, the `send` function waits until the sent data is processed by the device or the specified timeout time elapses.
If it is confirmed that the sent data has been processed by the device, the `send` function returns `true`, otherwise it returns `false`.

If the timeout time is 0, the `send` function does not check whether the sent data has been processed by the device or not.

If you want to data to be sent surely, it is recommended to set this to an appropriate value.

### stop

You can stop the output by sending `Stop` data.

Note that the `Stop` data resets the settings of Silencer.

### clear

You can clear the flags and `Gain`/`Modulation` data in the device by sending `Clear` data.
