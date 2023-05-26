# Concept

The main classes that make up the SDK are as follows.

* `Controller` is top-level user interface. All operations on AUTD3 is performed through this class.
* `Geometry` manages the placement of devices in the real world.
* `Link` is interface to AUTD3 devices.
* `Gain` manages the phase/amplitude of each transducer.
* `Modulation` manages Amplitude Modulation (AM).
* `STM` manages Spatio-Temporal Modulation (STM) functionality on firmware.

The flow of using the SDK is as follows:

1. Create `Geometry`: Configure all connected devices' positons and rotations
1. (Optional) Configure ultrasound frequency 
1. Create `Link`
1. Instansiate `Controller`
1. Initialize and synchronize devices
1. (Optional) Configure `Silencer` 
1. (Optional) Create and send `Modulation` 
1. Create and send `Gain`, `STM`

## Hardware description

Here is a top and back view of AUTD3.

<figure>
  <img src="../fig/Users_Manual/autd_trans_idx.jpg"/>
  <figcaption>AUTD front</figcaption>
</figure>

<figure>
  <img src="../fig/Users_Manual/autd_back.jpg"/>
  <figcaption>AUTD back</figcaption>
</figure>

AUTD3 consists of 249 [^fn_asm] transducers per unit.
SDK can individually control the transducers' frequency, phase, and amplitude.

The coordinate system of AUTD3 is a right-handed coordinate system, where the origin is the center of the 0-th transducer.
The x-axis is the major axis direction, i.e., the direction of 0→17, and the y-axis is the direction of 0→18.
As the unit system, $\SI{}{mm}$ is adopted for distance, $\SI{}{rad}$ for angle, and $\SI{}{Hz}$ for frequency.
The transducers are arranged with a spacing of $\SI{10.16}{mm}$, and the size, including the substrate, is $\SI{192}{mm}\times\SI{151.4}{mm}$.
The outline view of the device is shown below.

<figure>
  <img src="../fig/Users_Manual/transducers_array.jpg"/>
  <figcaption>Design drawing of transducer array</figcaption>
</figure>

In addition, AUTD3 can be expanded by daisy-chaining devices.
Connecting a PC and the first `EherCAT In` with an ethernet cable and connecting the $i$-th `EherCAT Out` and the $i+1$-th `EherCAT In`, an extended array can be configured.
The Ethernet cable used must be CAT 5e or higher.

[^fn_asm]: Three transducers are missing from $18\times 14=252$ for screws. The screw holes are placed in these positions to make the gap as small as possible when some units are placed in a grid.
