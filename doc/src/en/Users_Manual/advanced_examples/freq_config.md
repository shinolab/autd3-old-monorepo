# Mode configuration / Changing the frequency

You can change the frequency of ultrasound from $\ufreq$.
The conventional mode with $\ufreq$ is called Legacy mode, and the mode with variable frequency is called Advanced mode.

The default is Legacy mode.

You can change to the Advanced mode by the following.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
                  .advanced()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
#
#    Ok(())
# }
```
```cpp
auto autd = autd3::Controller::builder()
               .advanced()
```
```cs
var autd = Controller.Builder()
        .Advanced()
```
```python
autd = Controller.builder().advanced()
```

The frequency is specified by `set_frequency` or by changing `cycle` directly of `Transducer`.
The frequency can be specified as $\clkf/N,N=2,...,8191$.
The `cycle` represents this $N$.
In the case of `set_frequency`, the closest value of the possible $N$ is chosen.

Note that you must send `Synchronize` after frequency configuration.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#                  .advanced()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
for dev in autd.geometry_mut() {
    for tr in dev {
        tr.set_frequency(70e3)?;
    }
}

autd.send(Synchronize::new())?;
#    Ok(())
# }
```
```cpp
for (auto& dev : autd.geometry())
    for (auto& tr : dev)
        tr.set_frequency(70e3);

autd.send(autd3::Synchronize());
```
```cs
foreach (var dev in autd.Geometry)
    foreach (var tr in dev)
        tr.Frequency = 70e3;

autd.Send(new Synchronize());
```
```python
from pyautd3 import Synchronize

for dev in autd.geometry:
    for tr in dev:
        tr.frequency = 70e3

autd.send(Synchronize())
```

## AdvancedPhase mode

The communication latency in Advanced mode is twice as long as in Legacy mode, because the amplitude/phase data must be sent in one frame each.
In practice, amplitude data is not expected to be updated frequently, so the AdvancedPhase mode is provided to send only phase data.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let mut autd = Controller::builder()
                  .advanced_phase()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
#
#    Ok(())
# }
```
```cpp
auto autd = autd3::Controller::builder()
               .advanced_phase()
```
```cs
var autd = Controller.Builder()
        .AdvancedPhase()
```
```python
autd = Controller.builder().advanced_phase()
```

In AdvancedPhase mode, the amplitude is controlled by sending the `Amplitudes` class in advance.
All `Gain` amplitude parameters are ignored in this mode.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#                  .advanced_phase()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
let amp = Amplitudes::uniform(1.);
autd.send(amp)?;
#    Ok(())
# }
```

```cpp
const auto amp = autd3::Amplitudes::uniform(1.0);
autd.send(amp);
```

```cs
var amp = Amplitudes.Uniform(1.0);
autd.Send(amp);
```
```python
from pyautd3 import Amplitudes

amp = Amplitudes(1.)
autd.send(amp)
```
