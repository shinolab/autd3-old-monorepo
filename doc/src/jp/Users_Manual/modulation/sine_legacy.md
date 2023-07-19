# SineLegacy

古いversionにあった`Sine Modulation`と互換.
周波数として, 浮動小数点数の値を取れるが, 厳密に指定周波数になるのではなく, 出力可能な周波数の内, 最も近い周波数が選ばれる.
また, 音圧ではなくDuty比がSin波状になる.

その他APIは`Sine`と同等である.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::SineLegacy::new(150.);
# }
```

```cpp
const auto m = autd3::modulation::SineLegacy(150);
```

```cs
var m = new SineLegacy(150);
```

```python
from pyautd3.modulation import SineLegacy

m = SineLegacy(150)
```
