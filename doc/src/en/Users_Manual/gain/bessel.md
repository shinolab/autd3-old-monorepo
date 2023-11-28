# Bessel

`BesselBeam` generates a Bessel beam.
This `Gain` is based on the paper by Hasegawa et al [^hasegawa2017].

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/bessel_0.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/bessel_0.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/bessel_0.py}}
```

```python
{{#include ../../../codes/Users_Manual/gain/bessel_0.py}}
```

The first argument of the constructor is the apex of the virtual cone producing the beam, the second argument is the direction of the beam, and the third argument is the angle between the plane perpendicular to the beam and the side of the virtual cone producing the beam ($\theta_z$ in the figure below).

<figure>
  <img src="../../fig/Users_Manual/1.4985159.figures.online.f1.jpg"/>
  <figcaption>Bessel beam[^hasegawa2017]</figcaption>
</figure>

## Set intensity

You can change emission intensity by `with_intensity` method.

```rust,edition2021
{{#include ../../../codes/Users_Manual/gain/bessel_1.rs}}
```

```cpp
{{#include ../../../codes/Users_Manual/gain/bessel_1.cpp}}
```

```cs
{{#include ../../../codes/Users_Manual/gain/bessel_1.py}}
```

```python
{{#include ../../../codes/Users_Manual/gain/bessel_1.py}}
```

[^hasegawa2017]: Hasegawa, Keisuke, et al. "Electronically steerable ultrasound-driven long narrow air stream." Applied Physics Letters 111.6 (2017): 064104.
