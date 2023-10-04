# Fourier

`Fourier` is a modulation that generates a waveform by superimposing multiple `Sine`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
use autd3::modulation::Fourier;

# #[allow(unused_variables)]
# fn main()  {
let m = Fourier::new(Sine::new(100))
        .add_component(Sine::new(150))
        .add_components_from_iter([Sine::new(200)]);
# }
```

```cpp
auto m = autd3::modulation::Fourier(autd3::modulation::Sine(100))
             .add_component(autd3::modulation::Sine(150))
             .add_components_from_iter(std::vector{autd3::modulation::Sine(200)});
```

```cs
var m = new Fourier(new Sine(100))
        .AddComponent(new Sine(150))
        .AddComponentsFromIter(new []{new Sine(200)});
```

```python
m = Fourier(Sine(100)).add_component(Sine(150)).add_components_from_iter([Sine(200)])
```

You can also use `+` operator.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
use autd3::modulation::Fourier;

# #[allow(unused_variables)]
# fn main()  {
let m: Fourier = Sine::new(100) + Sine::new(150) + Sine::new(200);
# }
```

```cpp
const auto m = autd3::modulation::Sine(100) + autd3::modulation::Sine(150) + autd3::modulation::Sine(200);
```

```cs
var m = new Sine(100) + new Sine(150) + new Sine(200);
```

## Phase parameter of `Sine`

For `Fourier`, `Sine` has a feature to specify the phase parameter.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
use autd3::modulation::Fourier;

# #[allow(unused_variables)]
# fn main()  {
let m: Fourier = Sine::new(100) + Sine::new(150).with_phase(PI / 2.0);
# }
```

```cpp
const auto m = autd3::modulation::Sine(100) + autd3::modulation::Sine(150).with_phase(autd3::pi / 2.0);
```

```cs
var m = new Sine(100) + new Sine(150).WithPhase(AUTD3.Pi / 2.0);
```

```python
m = Fourier(Sine(100)) + Sine(150).with_phase(np.pi / 2.0)
```
