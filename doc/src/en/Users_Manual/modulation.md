# Modulation

`Modulation` is a mechanism to control AM modulation.

The modulation is applied to the amplitude of the sound pressure.
For example, if you use `Sine` with $\SI{1}{kHz}$, the sound pressure amplitude is as follows, and the envelope of the positive part (or negative part) of sound pressure follows the $\SI{1}{kHz}$ sine wave.

<figure>
  <img src="../fig/Users_Manual/sine_1k_mod.png"/>
</figure>

Currently, `Modulation` has the following restrictions.

- The buffer size is up to 65536.
- The sampling rate is $\clkf/N$, where $N$ is a 32-bit unsigned integer and must be at least 512.
- Modulation automatically loops. It is not possible to control only one loop, etc.
- The start/end timing of Modulation cannot be controlled.

The SDK has `Modulation` by default to generate several types of AM.

* [Static](./modulation/static.md)
* [Sine](./modulation/sine.md)
  * [Fourier](./modulation/fourier.md)
* [Square](./modulation/square.md)
* [Wav](./modulation/wav.md)
* [RawPCM](./modulation/rawpcm.md)
* [Cache](./modulation/cache.md)
* [RadiationPressure](./modulation/radiation.md)
* [Transform](./modulation/transform.md)

## Modulation common API

### Sampling configuration

You can get the sampling frequency with `sampling_config`.

Some `Modulation` can set the sampling configuration with `with_sampling_config`.
However, due to the constraints of `Modulation`, the sampling frequency may not be exactly the specified value.

```rust,edition2021
{{#include ../../codes/Users_Manual/modulation_0.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/modulation_0.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/modulation_0.cs}}
```

```python
{{#include ../../codes/Users_Manual/modulation_0.py}}
```

### Modulation data size

The modulation data size can be obtained as follows.

```rust,edition2021
{{#include ../../codes/Users_Manual/modulation_1.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/modulation_1.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/modulation_1.cs}}
```

```python
{{#include ../../codes/Users_Manual/modulation_1.py}}
```


## Modulation Delay

`Modulation` is applied to all transducers at the same time without considering propagation delay.
Therefore, there is a possibility that modulation is shifted depending on the distance between the transducer and the focal position.

To compensate for this, each transducer has a function to delay the sampling index to be sampled.

The following example shows how to set the delay of the $0$-th transducer of $0$-th device to $1$.

```rust,edition2021
{{#include ../../codes/Users_Manual/modulation_2.rs}}
```

```cpp
{{#include ../../codes/Users_Manual/modulation_2.cpp}}
```

```cs
{{#include ../../codes/Users_Manual/modulation_2.cs}}
```

```python
{{#include ../../codes/Users_Manual/modulation_2.py}}
```

The delay is the delay for the index to be sampled, so the delay time depends on the sampling frequency of `Modulation`.
