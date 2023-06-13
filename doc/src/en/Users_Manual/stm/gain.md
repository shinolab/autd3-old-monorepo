# GainSTM

`GainSTM` can handle arbitrary `Gain`s, unlike `FocusSTM`.
However, the number of `Gain`s that can be used is
- 2048 in Legacy mode
- 1024 in Advanced/AdvancedPhase mode

The following is an example of how to use `GainSTM`.
This is a sample that rotates the focus on a circle with a radius of $\SI{30}{mm}$ centered on a point $\SI{150}{mm}$ directly above the center of the array.

```rust
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let point_num = 200;
let radius = 30.0 * MILLIMETER;
let stm = GainSTM::new(1.0).add_gains_from_iter((0..point_num).map(|i| {
    let theta = 2.0 * PI * i as float / point_num as float;
    let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
    let g = Focus::new(center + p);
    Box::new(g) as _
}));
autd.send(stm)?;
# Ok(())
# }
```

```cpp
const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
constexpr size_t points_num = 200;
constexpr auto radius = 30.0;
autd3::GainSTM stm(1);
for (size_t i = 0; i < points_num; i++) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add_gain(autd3::gain::Focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0)));
}
autd.send(stm);
```

```cs
var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
const int pointNum = 200;
const double radius = 30.0;
var stm = new GainSTM(1);
for (var i = 0; i < pointNum; i++)
{
    var theta = 2.0 * Math.PI * i / pointNum;
    var p = radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0);
    stm.AddGain(new Focus(center + p));
}
autd.Send(stm);
```

```python
from pyautd3.stm import GainSTM

center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
size = 200
radius = 30.0
stm = GainSTM(1.0)
for i in range(size):
    theta = 2.0 * np.pi * i / size
    p = radius * np.array([np.cos(theta), np.sin(theta), 0])
    stm.add_gain(Focus(center + p))
autd.send(stm)
```

## GainSTMMode

`GainSTM` sends all phase/amplitude data, so it has a large latency[^fn_gain_seq].
To solve this problem, `GainSTM` has `PhaseFull` mode that sends only phase and reduces the transmission time by half[^phase_half].

This mode can be switched with `with_mode`.

```rust,should_panic
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
let stm = GainSTM::new(1.0).with_mode(GainSTMMode::PhaseFull);
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

[^fn_gain_seq]: About 60 times of `FocusSTM`

[^phase_half]: Legacy mode only
