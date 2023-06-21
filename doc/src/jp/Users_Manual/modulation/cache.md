# Cache

`with_cache`で計算結果をキャッシュしておくための`Modulation`を生成できる.

また, 変調データを変更する事もできる.

```rust
# use autd3::prelude::*;
# use autd3::link::Debug;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(Debug::new())?;
let mut m = Static::new().with_cache()?;
m[0] = 0.;
# Ok(())
# }
```

```cpp
// mは何らかのModulation
autd3::modulation::Cache c(m);

c[0] = 0;
```
