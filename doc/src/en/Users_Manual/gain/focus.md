# Focus

`Focus` is the simplest `Gain` and generates a single focal point.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main() {
# let x = 0.;
# let y = 0.;
# let z = 0.;
let g = autd3::gain::Focus::new(Vector3::new(x, y, z));
# }
```

```cpp
const auto g = autd3::gain::Focus(autd3::Vector3(x, y, z));
```

```cs
var g = new Focus(new Vector3d(x, y, z));
```

```python
from pyautd3.gain import Focus

g = Focus([x, y, z])
```

## Set amplitude

You can change amplitude by `with_amp` method.
The amplitude is normalized to 0-1 (1 by default).

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
let g = autd3::gain::Focus::new(Vector3::new(x, y, z)).with_amp(1.);
# }
```

```cpp
const auto g = autd3::gain::Focus(autd3::Vector3(x, y, z)).with_amp(1.);
```

```cs
var g = new Focus(new Vector3d(x, y, z)).WithAmp(1);
```

```python
g = Focus([x, y, z]).with_amp(1)
```
