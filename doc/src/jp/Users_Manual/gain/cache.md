# Cache

`with_cache`によって`Gain`の計算結果をキャッシュする`Gain`を生成できる.

また, 振幅/位相データを変更する事もできる.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# use autd3::link::Debug;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(Debug::new())?;
let mut c = Null::new().with_cache(autd.geometry())?;
c[0].phase = 0.;
# Ok(())
# }
```

```cpp
// gは何らかのGain
autd3::gain::Cache c(g, autd.geometry());

c[0].phase = 0;
```

```cs
// gは何らかのGain
var c = new AUTD3Sharp.Gain.Cache(g, autd.Geometry);

c[0].Phase = 0;
```

```python
from pyautd3.gain import Cache

# gは何らかのGain
c = Cache(g, autd.geometry)

c[0].phase = 0
```
