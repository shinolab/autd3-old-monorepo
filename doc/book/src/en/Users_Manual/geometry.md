# Geometry

This section describes Geometry that manages how AUTD3 devices are placed in the real world.

## Multiple devices

Multiple AUTD3 devices can be daisy-chained together.
The SDK is designed to be transparent even when multiple devices are connected.

If you want to connect multiple devices,
Connect the PC to the first `EtherCAT In` via a cable, and the $i$-th `EtherCAT Out` to the $i+1$-th `EtherCAT In` via a cable (see [Concept](concept.md)).
The power supply can also be daisy-chained, and any of the three power connectors can be used.

To use multiple devices with the SDK, call the `add_device` function for the number of connected devices.
The first argument of the `add_device` function is the position, and the second is the rotation.
The rotation is specified in ZYZ Euler angles or Quaternion.
Note that the translation is applied in global coordinates after the rotation.

<figure>
  <img src="../fig/Users_Manual/autd_hori.jpg"/>
  <figcaption>Horizontal alignment</figcaption>
</figure>

For example, suppose that the devices are arranged and connected as shown in the figure above, and that the device on the left is the first device and the device on the right is the second device.
Assume that the global coordinates are the same as the local coordinates of the first device, then you must setup Geometry as follows:

```cpp
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(autd3::AUTD3::DEVICE_WIDTH, 0, 0), autd3::Vector3::Zero()))
                      .build();
```
, where `DEVICE_WIDTH` is the width of the device including the substrate.
The second argument should be zero, since the device is not rotated.

Also, for example, if the global coordinates are the same as the local coordinates of the **second** device, then you must setup Geometry as follows:

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
Here, you must setup Geometry as follows:

```cpp
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .add_device(autd3::AUTD3(autd3::Vector3(0, 0, autd3::AUTD3::DEVICE_WIDTH), autd3::Vector3(0, autd3::pi / 2.0, 0)))
                      .build();
```

The SDK API uses global coordinates, so it does not depend on the number of connected devices and can be used transparently.

## Geometry viewer

You can use `GeometryViewer` to visualize `Geometry`.

```cpp

#include "autd3/extra/geometry_viewer.hpp"

...

  autd3::extra::GeometryViewer().window_size(800, 600).vsync(true).view(autd.geometry());
```

To use `GeometryViewer`, the `BUILD_GEOMETRY_VIEWER` option must be set in CMake[^vulkan_mac].

[^vulkan_mac]: If you want to use `GeometryViewer` on mac, you also need to install [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/).
