# Concept

The following is a basic components of AUTD3 SDK.

- `Controller` - Controller class. All operations to AUTD3 are done via this class
- `Geometry` - Container of `Device`
  - `Device` - Class corresponding to AUTD3 device
- `Link` - Interface to AUTD3 devices
- `Gain` - Manage the phase/amplitude of each transducer
- `Modulation` - Manage the amplitude modulation (AM) of each transducer
- `STM` - Manage the spatio-temporal modulation (STM) on firmware

The following is the front and back photos of AUTD3.

<figure>
  <img src="../fig/Users_Manual/autd_trans_idx.jpg"/>
  <figcaption>Front photo of AUTD3 device</figcaption>
</figure>

<figure>
  <img src="../fig/Users_Manual/autd_back.jpg"/>
  <figcaption>Back photo of AUTD3 device</figcaption>
</figure>


AUTD3 is composed of 249 transducers per device[^fn_asm].
From SDK, the phase/amplitude of all transducers can be specified individually.
The coordinate system of AUTD3 adopts the right-handed coordinate system, and the center of the 0th transducer is the origin.
The x-axis is the long axis direction, that is, the direction of 0→17, and the y-axis is the direction of 0→18.
In addition, the unit system is mm for distance, rad for angle, and Hz for frequency.
The transducers are arranged at intervals of $\SI{10.16}{mm}$, and the size including the substrate is $\SI{192}{mm}\times\SI{151.4}{mm}$.

The followings is the dimension of transducer array.

<figure>
  <img src="../fig/Users_Manual/transducers_array.jpg"/>
  <figcaption>The dimension of transducer array</figcaption>
</figure>

In addition, AUTD3 can be connected to each other via the daisy chain.
You can compose extended array by connecting the EherCAT In of the $i$-th device and the EherCAT Out of the $i+1$-th device with an Ethernet cable.
(The ethernet cable must be CAT 5e or higher.)

You have to supply $\SI{24}{V}$ DC power to AUTD3.
The power line can be connected to each other, and any of the three power connectors can be used.
The power connector is Molex 5566-02A.

> Note: AUTD3 consumes up to $\SI{2}{A}$ of current per device. Please pay attention to the maximum output current of the power supply.

[^fn_asm]: $18\times 14=252$ transducers are mounted on the substrate, but 3 transducers are missing for the screw holes. The reason why the screw holes are placed at this position is to minimize the gap when multiple devices are placed side by side.
