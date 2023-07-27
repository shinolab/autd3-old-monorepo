# Static

Without modulation.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Static::new();
# }
```

```cpp
const auto m = autd3::modulation::Static();
```

```cs
var m = new Static();
```

```python
from pyautd3.modulation import Static

m = Static()
```

## Set amplitude

You can set the amplitude of the static wave with `with_amp`.
The value is clamped to $\[0, 1\]$.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Static::new().with_amp(1.);
# }
```

```cpp
const auto m = autd3::modulation::Static().with_amp(1.);
```

```cs
var m = new Static().WithAmp(1);
```

```python
from pyautd3.modulation import Static

m = Static().with_amp(1)
```
