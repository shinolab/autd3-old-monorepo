# Focus

`Focus`は最も単純な`Gain`であり, 単一焦点を生成する.

```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
let g = autd3::gain::Focus::new(Vector3::new(x, y, z));
# }
```

```cpp
const auto g = autd3::gain::Focus(autd3::Vector3(x, y, z));
```

```cs
var g = new Focus(new Vector3d(x, y, z));
```

```python
from pyautd3.gain import Focus

g = Focus([x, y, z])
```

## 振幅の指定

`with_amp`にて, 0-1の規格化された音圧振幅を指定できる.
$\[0, 1\]$の範囲外の値は$\[0, 1\]$にクランプされる (すなわち, $0$未満の値は$0$に, $1$より大きい値は$1$になる).

```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
let g = autd3::gain::Focus::new(Vector3::new(x, y, z)).with_amp(1.);
# }
```

```cpp
const auto g = autd3::gain::Focus(autd3::Vector3(x, y, z)).with_amp(1.);
```

```cs
var g = new Focus(new Vector3d(x, y, z)).WithAmp(1);
```

```python
g = Focus([x, y, z]).with_amp(1)
```
