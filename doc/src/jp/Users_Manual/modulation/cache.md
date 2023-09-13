# Cache

`with_cache`で計算結果をキャッシュしておくための`Modulation`を生成できる.

また, 変調データを変更する事もできる.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(Debug::new())?;
# let m = Static::new();
// mは何らかのModulation
let mut c = m.with_cache()?;
c[0] = 0.;
# Ok(())
# }
```

```cpp
// mは何らかのModulation
auto c = m.with_cache();
c[0] = 0;
```

```cs
// mは何らかのModulation
var c = m.WithCache();
c[0] = 0;
```

```python
from pyautd3.modulation import Cache

# mは何らかのModulation
c = Cache(m)
c[0] = 0
```
