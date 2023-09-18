# Cache

You can cache the calculation result of `Modulation` by `with_cache` method.

Also, you can modify the modulation data.

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
// m is some Modulation
let mut c = m.with_cache()?;
c[0] = 0.;
# Ok(())
# }
```

```cpp
// m is some Modulation
auto c = m.with_cache();
c[0] = 0;
```

```cs
// m is some Modulation
var c = m.WithCache();
c[0] = 0;
```

```python
# m is some Modulation
c = m.with_cache()
c[0] = 0
```
