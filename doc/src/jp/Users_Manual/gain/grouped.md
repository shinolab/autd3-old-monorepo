# Grouped

`Grouped`は複数のデバイスを使用する際に,
各デバイスで別々の`Gain`を使用するための`Gain`である.

`Grouped`では, デバイスIDと任意の`Gain`を紐付けて使用する.

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
# let mut autd = autd3::Controller::builder().open_with(autd3::link::Debug::new()).unwrap();
# let g1 = autd3::gain::Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta);
# let g2 = autd3::gain::Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta);
let g = autd3::gain::Grouped::new().add(0, g1).add(1, g2);
# autd.send(g);
# }
```

```cpp
auto g = autd3::gain::Grouped();
g.add_gain(0, g1);
g.add_gain(0, g2);
```

```cs
var g = new Grouped();
g.AddGain(0, g1);
g.AddGain(0, g2);
```

```python
from pyautd3.gain import Grouped

g = Grouped()
g.add_gain(0, g1)
g.add_gain(0, g2)
```

上の場合は, デバイス0が`Gain g0`, デバイス1が`Gain g1`を使用する.
