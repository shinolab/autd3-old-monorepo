# Concept

The main classes that make up the SDK are as follows.

* `Controller` - All operations on AUTD3 are performed through this class.
* `Geometry` - manages the placement of devices in the real world.
* `Link` - Interface to AUTD3 devices.
* `Gain` - manages the phase/amplitude of each transducer.
* `Modulation` - manages Amplitude Modulation (AM)
* `STM` - manages Spatio-Temporal Modulation (STM) functionality on Hardware

The flow of using the SDK is as follows:

1. Instansiate `Controller`
1. Set the position and orientation of connected devices
1. Create and connect `Link`
1. Initialize devices
1. Create and send `Gain`, `STM`, and `Modulation`

## Hardware description

Here is a top view of AUTD3.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/autd_trans_idx.jpg"/>
  <figcaption>AUTD front</figcaption>
</figure>

The following is an image of the back of AUTD3. The connector for 24V power supply is Molex 5566-02A.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/autd_back.jpg"/>
  <figcaption>AUTD back</figcaption>
</figure>

AUTD3 consists of 249 [^fn_asm] transducers per unit, each of which is assigned an index number, as shown in the figure.
SDK can individually control the transducers' frequency, phase, and amplitude.

The coordinate system of AUTD3 is a right-handed coordinate system, where the origin is the center of the 0-th transducer.
The $x$-axis is the major axis direction, i.e., the direction of 0→17, and the $y$-axis is the direction of 0→18.
As the unit system, $\SI{}{mm}$ is adopted for distance, $\SI{}{rad}$ for angle, and $\SI{}{Hz}$ for frequency.
The transducers are arranged with a spacing of $\SI{10.16}{mm}$, and the size, including the substrate, is $\SI{192}{mm}\times\SI{151.4}{mm}$.
The outline view of the device is shown below.

<figure>
  <img src="https://raw.githubusercontent.com/shinolab/autd3/master/book/src/fig/Users_Manual/transducers_array.jpg"/>
  <figcaption>Design drawing of transducer array</figcaption>
</figure>

In addition, AUTD3 can be expanded by daisy-chaining devices.
Connecting a PC and the first `EherCAT In` with an ethernet cable and connecting the $i$-th `EherCAT Out` and the $i+1$-th `EherCAT In`, an extended array can be configured.
The Ethernet cable used must be CAT 5e or higher.

[^fn_asm]: Three transducers are missing from $18\times 14=252$ for screws. The screw holes are placed in these positions to make the gap as small as possible when some units are placed in a grid.
