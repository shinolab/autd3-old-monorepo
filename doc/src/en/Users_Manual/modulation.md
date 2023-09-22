# Modulation

`Modulation` is a mechanism to control AM modulation.

The modulation is applied to the amplitude of the sound pressure.
For example, if you use `Sine` with $\SI{1}{kHz}$, the sound pressure amplitude is as follows, and the envelope of the positive part (or negative part) of sound pressure follows the $\SI{1}{kHz}$ sine wave.

<figure>
  <img src="../fig/Users_Manual/sine_1k_mod.png"/>
</figure>

Currently, `Modulation` has the following restrictions.

- The buffer size is up to 65536.
- The sampling rate is $\clklf/N$, where $N$ is a 32-bit unsigned integer and must be at least 512.
- Modulation automatically loops. It is not possible to control only one loop, etc.
- The start/end timing of Modulation cannot be controlled.

The SDK has `Modulation` by default to generate several types of AM.

* [Static](./modulation/static.md)
* [Sine](./modulation/sine.md)
  * [Fourier](./modulation/fourier.md)
* [SineLegacy](./modulation/sine_legacy.md)
* [Square](./modulation/square.md)
* [Wav](./modulation/wav.md)
* [RawPCM](./modulation/rawpcm.md)
* [Cache](./modulation/cache.md)
* [RadiationPressure](./modulation/radiation.md)
* [FIR](./modulation/fir.md)
* [Transform](./modulation/transform.md)

## Modulation common API

### Sampling frequency

You can get the sampling frequency with `sampling_frequency`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let m = autd3::modulation::SineLegacy::new(150.);
let fs = m.sampling_frequency();
# }
```

```cpp
const auto fs = m.sampling_frequency();
```

```cs
var fs = m.SamplingFrequency;
```

```python
fs = m.sampling_frequency
```

Some `Modulation` can set the sampling frequency with `with_sampling_frequency`.
However, due to the constraints of `Modulation`, the sampling frequency may not be exactly the specified value.

- e.g.,
  ```rust,edition2021
  # extern crate autd3;
  # use autd3::prelude::*;
  # #[allow(unused_variables)]
  # fn main()  {
  let m = autd3::modulation::Sine::new(150).with_sampling_frequency(4e3);
  # }
  ```

  ```cpp
  const auto m = autd3::modulation::Sine(150).with_sampling_frequency(4e3);
  ```

  ```cs
  var m = new Sine(150).WithSamplingFrequency(4e3);
  ```

  ```python
  m = Sine(150).with_sampling_frequency(4e3)
  ```

### Sampling frequency division

You can get the sampling frequency division $N$ with `sampling_frequency_division`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let m = autd3::modulation::SineLegacy::new(150.);
let div = m.sampling_frequency_division();
# }
```

```cpp
const auto div = m.sampling_frequency_division();
```

```cs
var div = m.SamplingFrequencyDivision;
```

```python
div = m.sampling_frequency_division
```

Some `Modulation` can set the sampling frequency division with `with_sampling_frequency_division`.

- e.g.,
  ```rust,edition2021
  # extern crate autd3;
  # use autd3::prelude::*;
  # #[allow(unused_variables)]
  # fn main()  {
  let m = autd3::modulation::Sine::new(150).with_sampling_frequency_division(5120);
  # }
  ```

  ```cpp
  const auto m = autd3::modulation::Sine(150).with_sampling_frequency_division(5120);
  ```

  ```cs
  var m = new Sine(150).WithSamplingFrequencyDivision(5120);
  ```

  ```python
  m = Sine(150).with_sampling_frequency_division(5120)
  ```

### Modulation data size

The modulation data size can be obtained as follows.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let m = autd3::modulation::SineLegacy::new(150.);
let n = m.len();
# Ok(())
# }
```

```cpp
const auto n = m.size();
```

```cs
var n = m.Length;
```

```python
n = len(m)
```


## Modulation Delay

`Modulation` is applied to all transducers at the same time without considering propagation delay.
Therefore, there is a possibility that modulation is shifted depending on the distance between the transducer and the focal position.

To compensate for this, each transducer has a function to delay the sampling index to be sampled.

The following example shows how to set the delay of the $0$-th transducer of $0$-th device to $1$.

```rust,should_panic,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
autd.geometry_mut()[0][0].set_mod_delay(1);
autd.send(ConfigureModDelay::new())?;
# Ok(())
# }
```

```cpp
autd.geometry()[0][0].set_mod_delay(1);
autd.send(autd3::ConfigureModDelay());
```

```cs
autd.Geometry[0][0].ModDelay = 1;
autd.Send(new ConfigureModDelay());
```

```python
from pyautd3 import ConfigureModDelay

autd.geometry[0][0].mod_delay = 1
autd.send(ConfigureModDelay())
```

The delay is the delay for the index to be sampled, so the delay time depends on the sampling frequency of `Modulation`.
