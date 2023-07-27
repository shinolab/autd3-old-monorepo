# SineLegacy

The legacy version of `Sine` modulation.
The frequency can be a floating point number, but it is not strictly the specified frequency, but the nearest frequency among the output frequencies is selected.
And, the duty ratio follows Sin wave, not sound pressure.

The other APIs are equivalent to `Sine`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::SineLegacy::new(150.);
# }
```

```cpp
const auto m = autd3::modulation::SineLegacy(150);
```

```cs
var m = new SineLegacy(150);
```

```python
from pyautd3.modulation import SineLegacy

m = SineLegacy(150)
```
