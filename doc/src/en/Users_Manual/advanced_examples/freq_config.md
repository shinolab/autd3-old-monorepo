# Mode configuration / Changing the frequency

You can change the frequency of ultrasound from $\ufreq$.
The conventional mode with $\ufreq$ is called Legacy mode, and the mode with variable frequency is called Advanced mode.

The default mode is Legacy mode.

You can change to the Advanced mode by the following.

```rust
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
               .advanced_mode()
```
```cs
var autd = Controller.Builder()
        .AdvancedMode()
```
```python
autd = Controller.builder().advanced_mode()
```


The frequency is specified by `set_frequency` or by changing `cycle` directly of `Transducer`.
The `Transducer` can be accessed via the `Geometry` iterator or via the indexer.
The frequency can be specified as $\clkf/N,N=2,...,8191$.
The `cycle` represents this $N$.
In the case of `set_frequency`, the closest value of the possible $N$ is chosen.

Note that frequency configuratino must be done before sending `Synchronize`.

```rust
# use autd3::prelude::*;
# use autd3::link::Debug;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#                  .advanced()
#        .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#        .open_with(Debug::new())?;
for tr in autd.geometry_mut() {
  tr.set_frequency(70e3)?;
}
#    Ok(())
# }
```
```cpp
for (auto& tr : autd.geometry())
    tr.set_frequency(70e3);
```
```cs
foreach (var tr in autd.Geometry)
    tr.Frequency = 70e3;
```
```python
for tr in autd.geometry:
    tr.frequency = 70e3
```

## AdvancedPhase mode

The communication latency in Advanced mode is twice as long as in Legacy mode, because the amplitude/phase data must be sent in one frame each.
In practice, amplitude data is not expected to be updated frequently, so the AdvancedPhase mode is provided to send only phase data.

```rust
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
               .advanced_phase_mode()
```
```cs
var autd = Controller.Builder()
        .AdvancedPhaseMode()
```
```python
autd = Controller.builder().advanced_phase_mode()
```

In this mode, the amplitude is controlled by sending the `Amplitudes` class in advance.
All `Gain` amplitude parameters are ignored in this mode.

```rust
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
const auto amp = autd3::Amplitudes(1.0);
autd.send(amp);
```
```cs
var amp = new Amplitudes(1.0);
autd.Send(amp);
```
```python
from pyautd3 import Amplitudes

amp = Amplitudes(1.)
autd.send(amp)
```
