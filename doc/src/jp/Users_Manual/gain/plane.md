# Plane

`Plane`は平面波を出力する

```rust
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
let g = autd3::gain::Plane::new(Vector3::new(nx, ny, nz));
# }
```

```cpp
const auto g = autd3::gain::Plane(autd3::Vector3(nx, ny, nz));
```

```cs
var g = new Bessel(new Vector3d(nx, ny, nz));
```

```python
from pyautd3.gain import Plane

g = Plane([nx, ny, nz])
```

コンストラクタには平面波の進行方向を指定する.

## 振幅の指定

`with_amp`にて, 0-1の規格化された音圧振幅を指定できる.
$\[0, 1\]$の範囲外の値は$\[0, 1\]$にクランプされる (すなわち, $0$未満の値は$0$に, $1$より大きい値は$1$になる).

```rust
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let nx = 0.;
# let ny = 0.;
# let nz = 0.;
let g = autd3::gain::Plane::new(Vector3::new(nx, ny, nz)).with_amp(1.);
# }
```

```cpp
const auto g = autd3::gain::Plane(autd3::Vector3(nx, ny, nz)).with_amp(1);
```

```cs
var g = new Bessel(new Vector3d(nx, ny, nz)).WithAmp(1);
```

```python
g = Plane([nx, ny, nz]).with_amp(1)
```
