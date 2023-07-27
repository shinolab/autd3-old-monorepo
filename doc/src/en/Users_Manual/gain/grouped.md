# Grouped

`Grouped` is a `Gain` to use different `Gain` for each device.

In `Grouped`, a device ID is associated with an arbitrary `Gain`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
# let nx = 0.;
# let ny = 0.;
# let nz = 0.;
# let theta = 0.;
# let mut autd = autd3::Controller::builder().add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros())).open_with(autd3::link::NullLink {}).unwrap();
# let g1 = autd3::gain::Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta);
# let g2 = autd3::gain::Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta);
let g = autd3::gain::Grouped::new().add(0, g1).add(1, g2);
# autd.send(g);
# }
```

```cpp
auto g = autd3::gain::Grouped();
g.add(0, g1);
g.add(1, g2);
```

```cs
var g = new Grouped().Add(0, g1).Add(1, g2);
```

```python
from pyautd3.gain import Grouped

g = Grouped().add(0, g1).add(1, g2)
```

In the above case, device 0 uses `g0` and device 1 uses `g1`.
