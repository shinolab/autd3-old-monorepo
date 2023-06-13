# Plane

`Plane` output a plane wave.

```rust
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
# let nx = 0.;
# let ny = 0.;
# let nz = 0.;
# let theta = 0.;
let g = autd3::gain::Plane::new(Vector3::new(nx, ny, nz));
# }
```

```cpp
const auto g = autd3::gain::Plane(autd3::Vector3(nx, ny, nz));
```

```cs
var g = new Bessel(new Vector3d(nx, ny, nz));
```

```python
from pyautd3.gain import Plane

g = Plane([nx, ny, nz])
```

Specify the direction of the plane wave in the constructor of `Plane`.

## Set amplitude

You can change amplitude by `with_amp` method.
The amplitude is normalized to 0-1 (1 by default).

```rust
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let nx = 0.;
# let ny = 0.;
# let nz = 0.;
let g = autd3::gain::Plane::new(Vector3::new(nx, ny, nz)).with_amp(1.);
# }
```

```cpp
const auto g = autd3::gain::Plane(autd3::Vector3(nx, ny, nz)).with_amp(1);
```

```cs
var g = new Bessel(new Vector3d(nx, ny, nz)).WithAmp(1);
```

```python
g = Plane([nx, ny, nz]).with_amp(1)
```
