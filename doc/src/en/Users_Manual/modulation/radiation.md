# RadiationPressure

`RadiationPressure` is a `Modulation` to apply modulation to radiation pressure (proportional to the square of the sound pressure) instead of sound pressure.

For example, if you use `RadiationPressure` on `Sine` modulation with $\SI{1}{kHz}$, the radiation pressure of the sound pressure amplitude is as follows, and the envelope of the radiation pressure follows the $\SI{1}{kHz}$ sine wave.

<figure>
  <img src="../../fig/Users_Manual/sine_1k_mod_rad.png"/>
</figure>

```rust,edition2021
# extern crate autd3;
use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main() {
let m = Sine::new(150).with_radiation_pressure();
# }
```

```cpp
const auto m = autd3::modulation::Sine(150).with_radiation_pressure();
```

```cs
var m = new Sine(150).WithRadiationPressure();
```

```python
m = Sine(150).with_radiation_pressure()
```
