# Cache

`with_cache`によって`Gain`の計算結果をキャッシュする`Gain`を生成できる.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(autd3::link::Nop::builder())?;
# let g = Null::new();
// gは何らかのGain
let c = g.with_cache();
# autd.send(c)?;
# Ok(())
# }
```

```cpp
// gは何らかのGain
auto c = g.with_cache();
```

```cs
// gは何らかのGain
var c = g.WithCache();
```

```python
# gは何らかのGain
c = g.with_cache()
```
