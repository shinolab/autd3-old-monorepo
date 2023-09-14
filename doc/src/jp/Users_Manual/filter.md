# Filter

Filter機能は, 振幅 (デューティー比) と位相に何らかの値を加算するための機能である.

Filterは以下のように, 各振動子に設定する.
そして, `ConfigureAmpFilter`/`ConfigurePhaseFilter`を送信することで, デバイスに反映される.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# 
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .add_device(AUTD3::new(Vector3::new(0., 0., AUTD3::DEVICE_WIDTH), Vector3::new(0., PI/2.0, 0.)))
#    .open_with(Debug::new())?;
# let mut tr = &mut autd.geometry_mut()[0][0];
let amp_filter = tr.amp_filter();
tr.set_amp_filter(-0.5);
autd.send(ConfigureAmpFilter::new())?;
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

振幅フィルタには$-1$から$1$の値を設定できる.
$-1$がDuty比で$-50\%$, $1$がDuty比で$50\%$に相当する.

また, 位相フィルタには$-2\pi$から$2\pi$の値を設定できる.

これらのフィルタによる影響は`Modulation`や`Silencer`の前, `Gain`や`FocusSTM/GainSTM`の後に適用される.
