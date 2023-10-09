# Cache

`with_cache`で計算結果をキャッシュしておくための`Modulation`を生成できる.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(autd3::link::Nop::builder())?;
# let m = Static::new();
// mは何らかのModulation
let mut c = m.with_cache()?;
# Ok(())
# }
```

```cpp
// mは何らかのModulation
auto c = m.with_cache();
```

```cs
// mは何らかのModulation
var c = m.WithCache();
```

```python
# mは何らかのModulation
c = m.with_cache()
```
