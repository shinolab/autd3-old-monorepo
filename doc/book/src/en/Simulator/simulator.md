# Simulator

AUTD Simulator is a simulator for AUTD3 supporting Windows, Linux, and macOS.

## Build

To build AUTD Simulator, you need to turn on `BUILD_SIMULATOR` flag of CMake.
If you use macOS, you need to install Vulkan SDK.

```
cmake .. -DBUILD_SIMULATOR=ON
```

See [sample](https://github.com/shinolab/autd3/blob/master/examples/simulator_server.cpp) for running AUTD Simulator.

# How to

When you run the AUTD Simulator, it will be waiting for a connection.
If you run a client program using `link::Simulator`, you will see the sound field which matches the contents of the client program.
The panel shown in the simulator window is called "Slice", and you can visualize the sound field at any position using this slice.
The phase and amplitude of each transducer are represented by hue and intensity, respectively.

The sound field displayed in the simulator is a simple superposition of spherical waves.
Directivity and nonlinear effects are not taken into account.

The GUI displayed on the screen is used to control Slice and the camera.
The GUI is based on [Dear ImGui](https://github.com/ocornut/imgui), which can be operated by mouse.
You can also enter the numerical input mode by "Ctrl+click".

In addition, you can move the camera by drag and rotate the camera by shift+drag.

## Slice tab

Slice tab allows you to change the size, position and rotation of the slice.
Rotation is specified in terms of XYZ Euler angles.
Pressing `xy`, `yz`, or `zx` button rotates the slice parallel to each plane.

Slice represents the intensity of sound pressure in terms of color.
Color scale represents the maximum value of sound pressure in this color space.
When a large number of devices are used, color saturation may occur, in which case the value of `Color scale` should be increased.
The alpha value of the slice itself can be specified with `Slice alpha`.

You can also save the sound field displayed in the slice.

## Camera tab

In Camera tab, you can change camera position, rotation, field of view, near clip, and far clip.
The rotation is specified by XYZ Euler angles.

## Config tab

In Config tab, you can set the sound speed, font size, and background color.

You can also switch display/enable for each device.
If you turn off the display, the devices are not displayed but contribute to the sound field.
If you turn off the enable, it does not contribute to the sound field.

## Info tab

In Info tab, you can check Silencer, Modulation, and STM information.

You can check the Silencer setting, but it is not affected in the sound field.

Also, Modulation is not affected in the sound field.
Instead, how the sound pressure is modulated is shown in this tab.
In raw mode, how the duty ratio is modulated is also shown.

When STM is sent, the information of STM is displayed.
The STM does not switch automatically, you must specify the STM index to display them.

## Other settings

You can change default settings in `simulator::Settings`.

If vsync is set to true, vertical synchronization is enabled.
