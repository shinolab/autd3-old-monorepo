# Focus

`Focus`は最も単純な`Gain`であり, 単一焦点を生成する.

```rust,edition2021
# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() {
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

`with_intensity`にて, 出力振幅を指定できる.

```rust,edition2021
# extern crate autd3;
# extern crate tokio;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
let g = autd3::gain::Focus::new(Vector3::new(x, y, z)).with_intensity(EmitIntensity::MAX);
# }
```

```cpp
const auto g = autd3::gain::Focus(autd3::Vector3(x, y, z)).with_intensity(EmitIntensity::MAX);
```

```cs
var g = new Focus(new Vector3d(x, y, z)).WithAmp(1);
```

```python
g = Focus([x, y, z]).with_amp(1)
```
