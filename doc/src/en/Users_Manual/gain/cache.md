# Cache

You can cache the calculation result of `Gain` by `with_cache` method.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let mut autd = Controller::builder()
#     .add_device(AUTD3::new(Vector3::zeros(), Vector3::zeros()))
#     .open_with(autd3::link::Nop::builder())?;
# let g = Null::new();
// g is some Gain
let c = g.with_cache();
# autd.send(c)?;
# Ok(())
# }
```

```cpp
// g is some Gain
auto c = g.with_cache();
```

```cs
// g is some Gain
var c = g.WithCache();
```

```python
# g is some Gain
c = g.with_cache()
```
