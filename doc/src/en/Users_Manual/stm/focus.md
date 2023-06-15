# FocusSTM

- The maximum number of sampling points is $65536$.
- The sampling frequency is $\clklf/N$.

THe following is an example of using `FocusSTM` to focus on a point $\SI{150}{mm}$ directly above the center of the array with a radius of $\SI{30}{mm}$ centered on the center of the array.

```rust,should_panic
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
let center = autd.geometry().center() + Vector3::new(0., 0., 150.0 * MILLIMETER);
let point_num = 200;
let radius = 30.0 * MILLIMETER;
let stm = FocusSTM::new(1.0).add_foci_from_iter((0..point_num).map(|i| {
    let theta = 2.0 * PI * i as float / point_num as float;
    let p = radius * Vector3::new(theta.cos(), theta.sin(), 0.0);
    center + p
}));
autd.send(stm)?;
# Ok(())
# }
```

```cpp
const autd3::Vector3 center = autd.geometry().center() + autd3::Vector3(0.0, 0.0, 150.0);
constexpr size_t points_num = 200;
constexpr auto radius = 30.0;
autd3::FocusSTM stm(1);
for (size_t i = 0; i < points_num; i++) {
    const auto theta = 2.0 * autd3::pi * static_cast<double>(i) / static_cast<double>(points_num);
    stm.add_focus(center + autd3::Vector3(radius * std::cos(theta), radius * std::sin(theta), 0));
}
autd.send(stm);
```

```cs
var center = autd.Geometry.Center + new Vector3d(0, 0, 150);
const int pointNum = 200;
const double radius = 30.0;
var stm = new FocusSTM(1);
for (var i = 0; i < pointNum; i++)
{
    var theta = 2.0 * Math.PI * i / pointNum;
    var p = radius * new Vector3d(Math.Cos(theta), Math.Sin(theta), 0);
    stm.AddFocus(center + p);
}
autd.Send(stm);
```

```python
from pyautd3.stm import FocusSTM

center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
size = 200
radius = 30.0
stm = FocusSTM(1.0)
for i in range(size):
    theta = 2.0 * np.pi * i / size
    p = radius * np.array([np.cos(theta), np.sin(theta), 0])
    stm.add_focus(center + p)
autd.send(stm)
```

`FocusSTM`'s constructor takes the STM frequency as an argument.
Note that the specified frequency and the actual frequency may differ due to constraints on the number of sampling points and the sampling period.
For example, the above example runs 200 points at $\SI{1}{Hz}$, so the sampling frequency should be $\SI{200}{Hz}=\clklf/102400$.
However, if `point_num=199`, the sampling frequency must be $\SI{199}{Hz}$, but there is no integer $N$ that satisfies $\SI{199}{Hz}=\clklf/N$.
Therefore, the closest $N$ is selected.
As a result, the specified frequency and the actual frequency are shifted.
`frequency` can be used to check the actual frequency.
