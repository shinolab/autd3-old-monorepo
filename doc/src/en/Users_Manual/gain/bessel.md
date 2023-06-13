# Bessel

`BesselBeam` generates a Bessel beam.
This `Gain` is based on the paper by Hasegawa et al [^hasegawa2017].

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
let g = autd3::gain::Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta);
# }
```

```cpp
const auto g = autd3::gain::Bessel(autd3::Vector3(x, y, z), autd3::Vector3(nx, ny, nz), theta);
```

```cs
var g = new Bessel(new Vector3d(x, y, z), new Vector3d(nx, ny, nz), theta);
```

```python
from pyautd3.gain import Bessel

g = Bessel([x, y, z], [nx, ny, nz], theta)
```

The first argument of the constructor is the apex of the virtual cone producing the beam, the second argument is the direction of the beam, and the third argument is the angle between the plane perpendicular to the beam and the side of the virtual cone producing the beam ($\theta_z$ in the figure below).

<figure>
  <img src="../../fig/Users_Manual/1.4985159.figures.online.f1.jpg"/>
  <figcaption>Bessel beam (長谷川らの論文より引用)</figcaption>
</figure>

## Set amplitude

You can change amplitude by `with_amp` method.
The amplitude is normalized to 0-1 (1 by default).

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
let g = autd3::gain::Bessel::new(Vector3::new(x, y, z), Vector3::new(nx, ny, nz), theta)
            .with_amp(1.);
# }
```

```cpp
const auto g = autd3::gain::Bessel(autd3::Vector3(x, y, z), theta)
                .with_amp(1.);
```

```cs
var g = new Bessel(new Vector3d(x, y, z), new Vector3d(nx, ny, nz), theta)
            .WithAmp(1);
```

```python
g = Bessel([x, y, z], [nx, ny, nz], theta).with_amp(1)
```

[^hasegawa2017]: Hasegawa, Keisuke, et al. "Electronically steerable ultrasound-driven long narrow air stream." Applied Physics Letters 111.6 (2017): 064104.
