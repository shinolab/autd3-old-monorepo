# GainSTM

`GainSTM` can handle arbitrary `Gain`s, unlike `FocusSTM`.
However, the number of `Gain`s that can be used is
- 2048 in Legacy mode
- 1024 in Advanced/AdvancedPhase mode

The following is an example of how to use `GainSTM`.
This is a sample that rotates the focus on a circle with a radius of $\SI{30}{mm}$ centered on a point $\SI{150}{mm}$ directly above the center of the array.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Nop::builder()).unwrap();
let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let point_num = 200;
let radius = 30.0 * MILLIMETER;
let stm = GainSTM::new(1.0).add_gains_from_iter((0..point_num).map(|i| {
    let theta = 2.0 * PI * i as float / point_num as float;
    let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
    Focus::new(center + p)
}));
autd.send(stm)?;
# Ok(())
# }
```

```cpp
#include <ranges>
using namespace std::ranges::views;

auto stm = autd3::GainSTM(1).add_gains_from_iter(iota(0) | take(points_num) | transform([&](auto i) {
                                                    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
                                                    return autd3::gain::Focus(center +
                                                                            autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
                                                }));
```

```cs
var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
const int pointNum = 200;
const double radius = 30.0;
var stm = new GainSTM(1.0).AddGainsFromIter(Enumerable.Range(0, pointNum).Select(i =>
{
    var theta = 2.0 * Math.PI * i / pointNum;
    return new Focus(center + radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0));
}));
autd.Send(stm);
```

```python
from pyautd3.stm import GainSTM

center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
point_num = 200
radius = 30.0
stm = GainSTM(1.0).add_gains_from_iter(
    map(
        lambda theta: Focus(
            center + radius * np.array([np.cos(theta), np.sin(theta), 0])
        ),
        map(lambda i: 2.0 * np.pi * i / point_num, range(point_num)),
    )
)
autd.send(stm)
```

## Specify the sampling frequency

You can specify the sampling frequency by `with_sampling_frequency` instead of frequency.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).open_with(autd3::link::Nop::builder())?;
let stm = GainSTM::with_sampling_frequency(1.0);
# let stm = stm.add_gain(Null::default()).add_gain(Null::default());
# autd.send(stm)?;
# Ok(())
# }
```

```cpp
auto stm = autd3::GainSTM::with_sampling_frequency(1);
```

```cs
var stm = GainSTM.WithSamplingFrequency(1);
```

```python
from pyautd3.stm import GainSTM

stm = GainSTM.with_sampling_frequency(1.0)
```

Also, you can specify the sampling frequency division ratio $N$ by `with_sampling_frequency_division`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).open_with(autd3::link::Nop::builder())?;
let stm = GainSTM::with_sampling_frequency_division(5120);
# let stm = stm.add_gain(Null::default()).add_gain(Null::default());
# autd.send(stm)?;
# Ok(())
# }
```

```cpp
auto stm = autd3::GainSTM::with_sampling_frequency_division(5120);
```

```cs
var stm = GainSTM.WithSamplingFrequencyDivision(5120);
```

```python
from pyautd3.stm import GainSTM

stm = GainSTM.with_sampling_frequency_division(5120)
```

## GainSTMMode

`GainSTM` sends all phase/amplitude data, so it has a large latency[^fn_gain_seq].
To solve this problem, `GainSTM` has `PhaseFull` mode that sends only phase and reduces the transmission time by half[^phase_half].

This mode can be switched with `with_mode`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Nop::builder()).unwrap();
let stm = GainSTM::new(1.0).with_mode(GainSTMMode::PhaseFull);
# let stm = stm.add_gain(Null::default()).add_gain(Null::default());
# autd.send(stm)?;
# Ok(())
# }
```

```cpp
auto stm = autd3::GainSTM(1).with_mode(autd3::GainSTMMode::PhaseFull);
```

```cs
var stm = new GainSTM(1).WithMode(GainSTMMode.PhaseFull);
```

```python
from pyautd3.stm import GainSTM, GainSTMMode

stm = GainSTM(1).with_mode(GainSTMMode.PhaseFull)
```

The default is `PhaseDutyFull` mode, which sends all information.

[^fn_gain_seq]: About 75 times of `FocusSTM`

[^phase_half]: Legacy mode only
