# Geometry

This section describes `Geometry` that manages how AUTD3 devices are placed in the real world.

[[_TOC_]]

## Multiple devices

Multiple AUTD3 devices can be daisy-chained together.
The SDK is designed to be transparent even when multiple devices are connected.

If you want to connect multiple devices,
Connect the PC to the first `EtherCAT In` via a cable, and the $i$-th `EtherCAT Out` to the $i+1$-th `EtherCAT In` via a cable (see [Concept](concept.md)).
The power supply can also be daisy-chained, and any of the three power connectors can be used.

> Note: An AUTD3 device consumes $\SI{2}{A}$ current at maximum. Pay attention to the maximum output current of the power supply.

To use multiple devices with the SDK, call the `add_device` function in order of the connected devices.

<figure>
  <img src="../fig/Users_Manual/autd_hori.jpg"/>
  <figcaption>Horizontal alignment</figcaption>
</figure>

For example, suppose that the devices are arranged and connected as shown in the figure above, and that the device on the left is the first device and the device on the right is the second device.
Assume that the global coordinates are the same as the local coordinates of the first device, then you must setup `Geometry` as follows:

```cpp
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .build();
```
The first argument of the `autd3::AUTD3` constructor is the position, and the second is the rotation.
The rotation is specified in ZYZ Euler angles or Quaternion.
And, `DEVICE_WIDTH` is the width of the device including the substrate.
Here, the second argument should be zero, since the device is not rotated.

Also, for example, if the global coordinates are the same as the local coordinates of the **second** device, then you must setup `Geometry` as follows:

```cpp
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3(-autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .build();
```

<figure>
  <img src="../fig/Users_Manual/autd_vert.jpg"/>
  <figcaption>Vertical alignment</figcaption>
</figure>

Furthermore, suppose that the above figure is the same as the one above, with the first unit on the bottom, the second unit on the left, and the global coordinates are the same as the local coordinates of the first unit.
Here, you must setup `Geometry` as follows:

```cpp
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, 0, autd3::AUTD3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi / 2.0, 0)))
                      .build();
```

The SDK API uses global coordinates, so it does not depend on the number of connected devices and can be used transparently.

## Device/Transducer index

Devices are assigned an index starting from 0 in the order they are connected.

In addition, each device has $249$ transducers and is assigned a local index (see [Concept](./concept.md)).
The global indices of the transducers are,
$$
  \text{global index} = \text{device index} \times 249 + \text{local index}.
$$
For example, the indices of the transducers in the second device are from $249$ to $497$.

## Geometry API

### Setting sound speed

In some cases, the wavelength is needed to calculate the phase of the transducer.
The wavelength $\lambda$ of a sound wave is calculated as $\lambda = c/f$ from the sound speed $c$ and frequency $f$.
The `sound_speed` member of `Geometry` represents this sound speed $c$.
```cpp
  autd.geometry().sound_speed = 340e3;
```
The unit of sound speed is mm/s.

You can also set the speed of sound from the temperature via `set_sound_speed_from_temp` function.
```cpp
  autd.geometry().set_sound_speed_from_temp(15);
```
The unit of temperature is degrees Celsius.

The default sound speed is $340\times 10^{3}\,\mathrm{mm/s}$, which corresponds to the speed of sound of air at about 15 degrees Celsius.

### Setting attenuation coefficient

In SDK, the sound pressure $p(\br)$ at the position $\br$ of the ultrasonic wave emitted from the transducer is modeled as
$$
  p(\br)= \frac{D(\theta)}{\|\br\|}\rme^{-\br\|\alpha}\rme^{-\im k \|\br\|},
$$
where $D(\theta)$ is the directivity, $k = 2\pi / \lambda$ is the wavenumber, and $\alpha$ is the attenuation coefficient.
The `attenuation` member of `Geometry` represents this attenuation coefficient $\alpha$.
```cpp
geometry().attenuation = 0.0;
```
The units are Np/mm.

By default, it is set to $0$.

### center/center_of

`center` is the center of all transducers, ``center_of`` is the center of transducers in a specified device.

### num_devices/num_transducers

You can get the number of devices with `num_devices` and the number of transducers with `num_transducers`.

### Moving/Rotating Devices

To change the position and rotation of devices, use the following functions.

- `translate`: translation
- `rotate`: rotation
- `affine`: Affine transformation (translation and rotation)

The first argument of these functions is a device index. 
If you ommit this, the transformation is applied to all devices.

### Access to Transducer

The `Geometry` is a container of `Transducer`, and `Transducer` stores information of each transducer.

To access the `Transducer`, use the indexer.
```cpp
  const auto& tr = autd.geometry()[0];
```
Alternatively, you can use `begin/end` to get the iterators to the first and to the last, respectively.
If you specify the index of the device in these functions, you can get the iterators to the first and the end in the device, respectively.

## Transducer API

### idx

Get the index of the transducer.

```cpp
  const auto tr_idx = autd.geometry()[0].idx();
```

### position/rotation

Get position and rotation.
The rotation is expressed in quaternions.

```cpp
  const auto pos = autd.geometry()[0].position();
  const auto rot = autd.geometry()[0].rotation();
```

### x_direction/y_direction/z_direction

Get x, y, z direction vectors of the transducer.

```cpp
  const auto x_dir = autd.geometry()[0].x_direction();
  const auto y_dir = autd.geometry()[0].y_direction();
  const auto z_dir = autd.geometry()[0].z_direction();
```

### mod_delay

Get and set the modulation delay of the transducer.
For details, see [Modulation](./modulation.md).

```cpp
  autd.geometry()[0].mod_delay = 1;
```

### cycle

Get and set the cycle $N$ of the transducer.
The frequency is $\clkf/N$ for the cycle $N$.

````cpp
  const auto tr_cycle = autd.geometry()[0].cycle;
  autd.geometry()[0].cycle = 4096;
````

The default is $4096 (\ufreq)$.

For details, see [Set mode/Change frequency](./advanced_examples/freq_config.md).

### frequency/set_frequency

Get/Set the frequency.
When setting the frequency $f$, the cycle $N$ closest to $\clkf/f$ is selected.

```cpp
  const auto tr_frequency = autd.geometry()[0].frequency();
  autd.geometry()[0].set_frequency(40e3);
```

The default is $\ufreq$.

For details, see [Set mode/Change frequency](./advanced_examples/freq_config.md).

### wavelength/wavenumber

Get wavelength and wavenumber.

```cpp
  const auto sound_speed = autd.geometry().sound_speed;
  const auto tr_wavelength = autd.geometry()[0].wavelength(sound_speed);
  const auto tr_wavenumber = autd.geometry()[0].wavenumber(sound_speed);
```

### align_phase_at

Calculates the phase of the transducer to align the phase of the ultrasound at a certain position.

## Geometry viewer

You can use `GeometryViewer` to visualize `Geometry`.

```cpp

#include "autd3/extra/geometry_viewer.hpp"

...

  autd3::extra::GeometryViewer().window_size(800, 600).vsync(true).view(autd.geometry());
```

To use `GeometryViewer`, you must link `autd3_model` library and `geometry_viewer` library.
Or, you can build it with the `BUILD_GEOMETRY_VIEWER` option in CMake.
You must install [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/) if you build `GeometryViewer`.
