# Transform

`Transform` is a feature to apply some post-processing to `Modulation`.

```rust,edition2021
# extern crate autd3;
use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main() {
let m = Sine::new(150).with_transform(|i, d| (d - 0.5).abs() + 0.5);
# }
```

```cpp
const auto m = autd3::modulation::Sine(150).with_transform([](const size_t idx, const double d) -> double { return std::abs(d - 0.5) + 0.5; });
```

```cs
var m = new Sine(150).WithTransform((i, d) => Math.Abs(d - 0.5) + 0.5);
```

```python
m = Sine(150).with_transform(lambda i, d: abs(d - 0.5) + 0.5)
```

`with_transform` takes `Fn(usize, f64) -> f64` as an argument, where the first argument is the index and the second argument is the original modulation data.
