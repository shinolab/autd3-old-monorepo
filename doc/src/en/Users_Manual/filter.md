# Filter

Filter is a function to add some value to amplitude (duty ratio) and phase.

Filter data is set to each transducer as follows, and it is reflected to the device by sending `ConfigureAmpFilter`/`ConfigurePhaseFilter`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., AUTD3::DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(autd3::link::Nop::builder())?;
# let mut tr = &mut autd.geometry_mut()[0][0];
let amp_filter = tr.amp_filter();
tr.set_amp_filter(-0.5);
autd.send(ConfigureAmpFilter::new())?;

# let mut tr = &mut autd.geometry_mut()[0][0];
let phase_filter = tr.phase_filter();
tr.set_phase_filter(PI);
autd.send(ConfigurePhaseFilter::new())?;
# Ok(())
# }
```

```cpp
const auto amp_filter = tr.amp_filter();
tr.set_amp_filter(-0.5);
autd.send(ConfigureAmpFilter());

const auto phase_filter = tr.phase_filter();
tr.set_phase_filter(autd3::pi);
autd.send(ConfigurePhaseFilter());
```

```cs
var ampFilter = tr.AmpFilter;
tr.AmpFilter = -0.5;
autd.Send(new ConfigureAmpFilter());

var phaseFilter = tr.PhaseFilter;
tr.PhaseFilter = Math.PI;
autd.Send(new ConfigurePhaseFilter());
```

```python
from pyautd3 import ConfigureAmpFilter, ConfigurePhaseFilter

amp_filter = tr.amp_filter
tr.amp_filter = -0.5
autd.send(ConfigureAmpFilter())

phase_filter = tr.phase_filter
tr.phase_filter = math.pi
autd.send(ConfigurePhaseFilter())
```

You can set the amplitude filter from $-1$ to $1$, where $-1$ means $-50\%$ duty ratio and $1$ means $50\%$ duty ratio.

Also, you can set the phase filter from $-2\pi$ to $2\pi$.

These filters is applied before `Modulation` and `Silencer` and after `Gain` and `FocusSTM/GainSTM`.
