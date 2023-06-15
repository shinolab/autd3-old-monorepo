# Cache

`Cache`は`Gain`の計算結果をキャッシュしておくための`Gain`である.

また, 振幅データを変更する事もできる.

```rust
# use autd3::prelude::*;
# use autd3::link::Debug;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(Debug::new())?;
let mut g = autd3::gain::Cache::new(Null::new(), autd.geometry())?;
g[0].phase = 0.;
# Ok(())
# }
```
