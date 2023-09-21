# Transform

You can use `Transform` to apply some post-processing to `Gain`.

```rust,edition2021
# extern crate autd3;
use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main() {
let g = Uniform::new(1.0).with_transform(|dev, tr: &LegacyTransducer , d| Drive {
    amp: d.amp - 0.5,
    phase: d.phase + PI,
});
# }
```

```cpp
const auto g = autd3::gain::Uniform(1.0).with_transform([](const autd3::Device& dev, const autd3::Transducer& tr,  autd3::Drive d) -> autd3::Drive
{
        d.amp -= 0.5;
        d.phase += autd3::pi;
        return d;
});
```

```cs
var g = new Uniform(1.0).WithTransform((dev, tr, d) =>
{
    d.Amp -= 0.5;
    d.Phase += AUTD3.Pi;
    return d;
});
```

```python
def f(dev, tr, d):
    d.amp -= 0.5
    d.phase += np.pi
    return d
g = Uniform(1.0).with_transform(f)
```

The argument of `with_transform` is `Fn(&Device<T>, &T, &Drive) -> Drive`.
