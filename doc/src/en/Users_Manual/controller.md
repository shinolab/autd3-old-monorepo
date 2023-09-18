# Controller

The followings are introductino of APIs in `Controller` class.

[[_TOC_]]

## fpga_info

Get the FPGA status.
Before using this, you need to set the `reads_fpga_info` flag in `Device`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).open_with(autd3::link::NullLink {}).unwrap();
autd.geometry_mut()[0].reads_fpga_info = true;
autd.send(UpdateFlags::new())?;

let info = autd.fpga_info();
# Ok(())
# }
```

```cpp
autd.geometry()[0].reads_fpga_info(true);
autd.send(autd3::UpdateFlags());

const auto info = autd.fpga_info();
```

```cs
autd.Geometry[0].ReadsFPGAInfo = true;
autd.Send(new UpdateFlags());

var info = autd.FPGAInfo;
```

```python
autd.geometry[0].reads_fpga_info = True
autd.send(UpdateFlags())

info = autd.fpga_info
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

### Stop

You can stop the output by sending `Stop` data.

Note that the `Stop` data resets the settings of Silencer.

### Clear

You can clear the flags and `Gain`/`Modulation` data in the device by sending `Clear` data.

## group

You can group the devices by using `group` function, and send different data to each group.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).open_with(autd3::link::NullLink{}).unwrap();
# let x = 0.;
# let y = 0.;
# let z = 0.;
autd.group(|dev| match dev.idx() {
    0 => Some("focus"),
    1 => Some("null"),
    _ => None,
})
.set("null", Null::new())?
.set("focus", Focus::new(Vector3::new(x, y, z)))?
.send()?;
# Ok(())
# }
```

```cpp
autd.group([](const autd3::Device& dev) -> std::optional<const char*> {
    if (dev.idx() == 0) {
        return "null";
    } else if (dev.idx() == 1) {
        return "focus";
    } else {
        return std::nullopt;
    }
    })
    .set("null", autd3::gain::Null())
    .set("focus", autd3::gain::Focus(x, y, z))
    .send();
```

```cs
autd.Group(dev =>
    {
        return dev.Idx switch
        {
            0 => "null",
            1 => "focus",
            _ => null
        };
    })
    .Set("null", new Null())
    .Set("focus", new Focus(autd.Geometry.Center + new Vector3d(0, 0, 150)))
    .Send();
```

```python
def grouping(dev):
    if dev.idx == 0:
        return "null"
    elif dev.idx == 1:
        return "focus"
    else:
        return None

autd.group(grouping)\
    .set("null", Null())\
    .set("focus", Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0])))\
    .send()
```

Unlike `gain::Group`, you can use any data that can be sent with `send` as a value.
However, you can only group by device.

> NOTE:
> This sample uses a string as a key, but you can use anything that can be used as a key for HashMap.
