# Geometry

In this chapter, we explain about `Geometry`.
`Geometry` manages how AUTD3 devices are placed in the real world.

[[_TOC_]]

## Connect multiple devices

AUTD3 devices can be connected to each other via the daisy chain.
SDK is designed to be used transparently even if multiple devices are connected.

To use multiple devices, connect the PC and the EtherCAT In of the first device with an Ethernet cable, and connect the EtherCAT Out of the $i$-th device and the EtherCAT In of the $i+1$-th device with an Ethernet cable (See [Concept](concept.md)).

In SDK, you must call `add_device` function **in the order of the connected devices** when using multiple devices.

<figure>
  <img src="../fig/Users_Manual/hor_left_ori_left_1.png"/>
  <figcaption>Horizontal alignment</figcaption>
</figure>

For example, suppose you have two devices as shown in the figure above.
The left device is the first device, and the right device is the second device.
Then, the code is as follows.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .add_device(AUTD3::new(Vector3::new(DEVICE_WIDTH, 0., 0.), Vector3::zeros()))
#    .open_with(Debug::new())?;
# Ok(())
# }
```

```cpp
auto autd = autd3::Controller::builder()
                .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
```

```cs
var autd = Controller.Builder()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .AddDevice(new AUTD3(new Vector3d(AUTD3.DeviceWidth, 0, 0), Vector3d.zero))
```

```python
auto = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_euler_zyz([DEVICE_WIDTH, 0.0, 0.0], [0.0, 0.0, 0.0]))\
```

Here, the first argument of the `AUTD3` constructor is the position, and the second argument is the rotation.
The rotation is specified by ZYZ Euler angles or quaternions.
Also, `DEVICE_WIDTH` is the width of the device (including the outline of the board).
In this example, no rotation is performed, so the second argument can be zero.

<figure>
  <img src="../fig/Users_Manual/hor_right_ori_left_1.png"/>
  <figcaption>Horizontal alignment</figcaption>
</figure>

And, for example, suppose you have two devices as shown in the figure above, where the global origin is set to the left device.
Then, the code is as follows.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::new(-DEVICE_WIDTH, 0., 0.), Vector3::zeros()))
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#    .open_with(Debug::new())?;
# Ok(())
# }
```

```cpp
auto autd = autd3::Controller::builder()
                .add_device(autd3::AUTD3(autd3::Vector3(-autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
```

```cs
var autd = Controller.Builder()
    .AddDevice(new AUTD3(new Vector3d(-AUTD3.DeviceWidth, 0, 0), Vector3d.zero))
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
```

```python
auto = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([-DEVICE_WIDTH, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
```

<figure>
  <img src="../fig/Users_Manual/vert.png"/>
  <figcaption>Vertical alignment</figcaption>
</figure>

Furthermore, for example, suppose you have two devices as shown in the figure above, where the global origin is set to the lower device.
Then, the code is as follows.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let autd = Controller::builder()
    .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
    .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# Ok(())
# }
```

```cpp
auto autd = autd3::Controller::builder()
                .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                .add_device(autd3::AUTD3(autd3::Vector3(0, 0, autd3::AUTD3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi/2.0, 0)))
```

```cs
var autd = Controller.Builder()
    .AddDevice(new AUTD3(Vector3d.zero, Vector3d.zero))
    .AddDevice(new AUTD3(new Vector3d(0, 0, AUTD3.DeviceWidth), new Vector3d(0, AUTD3.Pi/2, 0)))
```

```python
autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, DEVICE_WIDTH], [0.0, np.pi/2, 0.0]))\
```

## Device/Transducer index

Devices are assigned indices starting from 0 in the order in which they are connected to the PC.

Also, each device has 249 transducers, and local indices are assigned ([see the concept](./concept.md) for the surface photo of AUTD).
The global index of the transducer is
$$
  \text{global index} = \text{device index} \times 249 + \text{local index},
$$
For example, the index of the transducer of the second device is from 249 to 497.

## Geometry API

### Speed of sound

When calculating the phase of the transducer, there are some cases where the wavelength is required.
The wavelength $\lambda$ of the sound wave is calculated from the sound speed $v$ and the frequency $f$ as $\lambda = v/f$.
The `sound_speed` member of `Geometry` represents this sound speed $v$.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
autd.geometry_mut().sound_speed = 340e3;
# Ok(())
# }
```

```cpp
autd.geometry().set_sound_speed(340e3);
```

```cs
autd.Geometry.SoundSpeed = 340e3;
```

```python
autd.geometry.sound_speed = 340e3
```

The unit of the sound speed is mm/s.

You can also set the sound speed from the temperature.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
autd.geometry_mut().set_sound_speed_from_temp(15.);
# Ok(())
# }
```

```cpp
autd.geometry().set_sound_speed_from_temp(15);
```

```cs
autd.Geometry.SetSoundSpeedFromTemp(15);
```

```python
autd.geometry.set_sound_speed_from_temp(15)
```

The unit of the temperature is Celsius.

### Attenuation coefficient

In the SDK, the sound pressure $p(\br)$ at the position $\br$ of the emitted ultrasonic wave from the transducer is modeled as
$$
  p(\br) = \frac{D(\theta)}{\|\br\|}\rme^{-\|\br\|\alpha}\rme^{-\im k \|\br\|},
$$
where $D(\theta)$ is the directivity, $k = 2\pi / \lambda$ is the wave number, and $\alpha$ is the attenuation coefficient.
`attenuation` member of `Geometry` represents this attenuation coefficient $\alpha$.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
autd.geometry_mut().attenuation = 340e3;
# Ok(())
# }
```

```cpp
autd.geometry().set_attenuation(0.0);
```

```cs
autd.Geometry.Attenuation = 0.0;
```

```python
autd.geometry.attenuation = 0.0
```

The unit of the attenuation coefficient is Np/mm

The default value is 0.0.

### center/center_of

You can get the center of all devices with `center`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
let center = autd.geometry().center();
# Ok(())
# }
```

```cpp
const autd3::Vector3 center = autd.geometry().center();
```

```cs
var center = autd.Geometry.Center;
```

```python
center = autd.geometry.center
```

You can get the center of a device by `center_of` method.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
let center = autd.geometry().center_of(0);
# Ok(())
# }
```

```cpp
const autd3::Vector3 center = autd.geometry().center_of(0);
```

```cs
var center = autd.Geometry.CenterOf(0);
```

```python
center = autd.geometry.center_of(0)
```

### num_devices/num_transducers

You can get the number of devices and transducers by `num_devices` and `num_transducers` methods.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
let num_dev = autd.geometry().num_devices();
let num_tr = autd.geometry().num_transducers();
# Ok(())
# }
```

```cpp
const auto num_dev = autd.geometry().num_devices();
const auto num_tr= autd.geometry().num_transducers();
```

```cs
var numDev = autd.Geometry.NumDevices;
var numTr = autd.Geometry.NumTransducers;
```

```python
num_dev = autd.geometry.num_devices
num_tr = autd.geometry.num_transducers
```

### Affine transformation

To modify the position of the device added to `Geometry`, use the following functions.
- `translate`
- `rotate`
- `affine`

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
let t = Vector3::new(1., 0., 0.);
let r = UnitQuaternion::from_quaternion(Quaternion::new(1., 0., 0., 0.));
autd.geometry_mut().translate(t);
autd.geometry_mut().rotate(r);
autd.geometry_mut().affine(t, r);
# Ok(())
# }
```

You can move and rotate only a specific device by using the following functions.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
let t = Vector3::new(1., 0., 0.);
let r = UnitQuaternion::from_quaternion(Quaternion::new(1., 0., 0., 0.));
autd.geometry_mut().translate_of(0, t);
autd.geometry_mut().rotate_of(0, r);
autd.geometry_mut().affine_of(0, t, r);
# Ok(())
# }
```

### Transducer access

`Geometry` is a container of `Transducer`, and `Transducer` contains information of each transducer.

To access `Transducer`, use the following methods.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
let tr = &autd.geometry()[0];
# Ok(())
# }
```

```cpp
const auto tr = autd.geometry()[0];
```

```cs
var tr = autd.Geometry[0];
```

```python
tr = autd.geometry[0]
```

Or, you can use an iterator.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
for tr in autd.geometry().iter() {
  // do something
}
# Ok(())
# }
```

```cpp
for (const auto tr : autd.geometry())
```

```cs
foreach (var tr in autd.Geometry)
```

```python
for tr in autd.geometry:
```

## Transducer API

### idx

Get the global index of the transducer.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# let tr = &autd.geometry()[0];
let idx = tr.idx();
# Ok(())
# }
```

```cpp
const auto idx = tr.idx();
```

```cs
var idx = tr.Idx;
```

```python
idx = tr.idx
```

### position/rotation

Get the position and rotation of the transducer.
The rotation is represented by a quaternion.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# let tr = &autd.geometry()[0];
let position = tr.position();
let rotation = tr.rotation();
# Ok(())
# }
```

```cpp
const auto position = tr.position();
const auto rotation = tr.rotation();
```

```cs
var position = tr.Position;
var rotation = tr.Rotation;
```

```python
position = tr.position
rotation = tr.rotation
```

### x_direction/y_direction/z_direction

Get the direction of the transducer.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# let tr = &autd.geometry()[0];
let x_dir = tr.x_direction();
let y_dir = tr.y_direction();
let z_dir = tr.z_direction();
# Ok(())
# }
```

```cpp
const auto x_dir = tr.x_direction();
const auto y_dir = tr.y_direction();
const auto z_dir = tr.z_direction();
```

```cs
var xDir = tr.XDirection;
var yDir = tr.YDirection;
var zDir = tr.ZDirection;
```

```python
x_dir = tr.x_direction
y_dir = tr.y_direction
z_dir = tr.z_direction
```

### mod_delay

Set/get the modulation delay of the transducer.

See [Modulation](./modulation.md) for more details.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# let mut tr = &mut autd.geometry_mut()[0];
let delay = tr.mod_delay();
tr.set_mod_delay(0);
# Ok(())
# }
```

```cpp
const auto delay = tr.mod_delay();
tr.set_mod_delay(0);
```

```cs
var delay = tr.ModDelay;
tr.ModDelay = 0;
```

```python
delay = tr.mod_delay
tr.mod_delay = 0
```

### cycle

Set/get the cycle of the transducer.

Note: The cycle setting is not available in Legacy mode.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .advanced()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# let mut tr = &mut autd.geometry_mut()[0];
let cycle = tr.cycle();
tr.set_cycle(4096)?;
# Ok(())
# }
```

```cpp
const auto cycle = tr.cycle();
tr.set_cycle(4096);
```

```cs
var cycle = tr.Cycle;
tr.Cycle = 4096;
```

```python
cycle = tr.cycle
tr.cycle = 4096
```

The default value is $4096 (\ufreq)$.

See, [Mode configuration / Changing the frequency](./advanced_examples/freq_config.md) for more details.

### frequency/set_frequency

Set/get the frequency of the transducer.
When setting the frequency $f$, the closest period $N$ to $\clkf/f$ is selected.

Note: The frequency setting is not available in Legacy mode.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .advanced()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# let mut tr = &mut autd.geometry_mut()[0];
let freq = tr.frequency();
tr.set_frequency(40e3)?;
# Ok(())
# }
```

```cpp
const auto freq = tr.frequency();
tr.set_frequency(40e3);
```

```cs
var freq = tr.Frequency;
tr.Frequency = 40e3;
```

```python
freq = tr.frequency
tr.frequency = 40e3
```

The default value is $\ufreq$.

See, [Mode configuration / Changing the frequency](./advanced_examples/freq_config.md) for more details.

### wavelength/wavenumber

Get the wavelength and wavenumber of the transducer.

You need to pass the speed of sound as an argument to `wavelength` and `wavenumber`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .advanced()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# let sound_speed = autd.geometry().sound_speed;
# let mut tr = &mut autd.geometry_mut()[0];
let wavelen = tr.wavelength(sound_speed);
let wavenum = tr.wavenumber(sound_speed);
# Ok(())
# }
```

```cpp
const auto wavelen = tr.wavelength(sound_speed);
const auto wavenum = tr.wavenumber(sound_speed);
```

```cs
var wavelen = tr.Wavelength(soundSpeed);
var wavenum = tr.Wavenumber(soundSpeed);
```

```python
wavelen = tr.wavelength(sound_speed)
wavenum = tr.wavenumber(sound_speed)
```
