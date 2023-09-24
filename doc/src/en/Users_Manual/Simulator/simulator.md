# AUTD3 Simulator

AUTD Simulator is a simulator for AUTD3 supporting Windows, Linux, and macOS.

## AUTD Server

The simulator is included in `AUTD Server`.
Download the installer from [GitHub Releases](https://github.com/shinolab/autd3/releases).

When you run `AUTD Server`, the following screen will appear, so open the "Simulator" tab and click "Run" button.

<figure>
  <img src="../../fig/Users_Manual/autdserver_simulator.jpg"/>
</figure>

When you run the simulator, it is waiting for connection.

<figure>
  <img src="../../fig/sim_waiting.jpg"/>
</figure>

In this state, when you `open` the `Controller` using `link::Simulator`, a black panel will appear on the simulator.

<figure>
  <img src="../../fig/sim_init.jpg"/>
</figure>

This black panel is called "Slice", and this "Slice" can be used to visualize the sound field at an arbitrary position.
The phase of the transducer is represented by hue, and its amplitude is represented by intensity.

<figure>
  <img src="../../fig/sim_focus.jpg"/>
</figure>

The sound field displayed in the simulator is a simple superposition of spherical waves; directivity and nonlinear effects are not taken into account.

The GUI displayed on the left side of the screen is used to control "Slice" and the camera.
The GUI is based on [Dear ImGui](https://github.com/ocornut/imgui).

### Slice tab

In the Slice tab, you can change the size, position, and rotation of the slice.
The rotation is specified in terms of XYZ Euler angles.
The "xy", "yz", and "zx" buttons are used to rotate the slice to a position parallel to each plane.

You can select either "Acoustic" mode, which displays the sound pressure, or "Radiation" mode, which displays the square value of the sound pressure.

In the "Color settings" section, you can change the coloring palette, color scale, and alpha value of the slice itself.
If you use a large number of devices, colors may become saturated, in which case you should increase the value of color scale.

### Camera tab

In Camera tab, you can change camera position, rotation, field of view, near clip, and far clip.
The rotation is specified in terms of XYZ Euler angles.

### Config tab

In the Config tab, you can set the sound speed, font size, and background color.

You can also switch the show/enable/overheat settings for each device.
When "show" is turned off, the devices contribute to the sound field only by not being displayed.
When "enable" is turned off, it does not contribute to the sound field.

When "View as device" is turned on, the transducers are displayed as AUTD3 3D models instead of simple circles.

### Info tab

In the Info tab, information on FPS, Silencer, Modulation, and STM can be checked.

The Silencer setting can be checked, but it is not affected in the sound field.

When "Mod enable" is set, the modulation is reflected in the sound field.

The index of modulation data and STM is determined by system time.
"System time" represents the system time, which is elapsed time in nanoseconds since January 1, 2000, 0:00:00.

When "Auto play" is set, the system time is automatically incremented.
Other wise, you can set the system time manually.

### Others

"Save image as file" allows you to save an image of the sound field displayed on the current Slice.

"Auto" button automatically moves the camera to the appropriate location.
"Reset" resets the camera to its beggining state.
"Default" resets the camera to the default settings.

The settings are stored in "settings.json".
One of the settings that can be changed only from this file is `vsync`: Setting `vsync` to `false` disables vertical synchronization.
