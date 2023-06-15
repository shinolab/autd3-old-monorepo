# Sine

`Modulation` to transform the square of the sound pressure to a sine wave.

Specify the frequency $f$ as an integer in the constructor.

```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Sine::new(150);
# }
```

```cpp
const auto m = autd3::modulation::Sine(150);
```

```cs
var m = new Sine(150);
```

```python
from pyautd3.modulation import Sine

m = Sine(150)
```

## Set amplitude and offset

`Sine` applies AM so that the waveform of the sound pressure is
$$
    \frac{amplitude}{2} \times \sin(2\pi ft) + offset.
$$
Here, $amplitude$ and $offset$ can be specified by `with_amp` and `with_offset`, respectively (default is $1$ and $0.5$, respectively).

Note that the value that exceeds $\[0, 1\]$ is clamped in $\[0, 1\]$.

```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Sine::new(150)
        .with_amp(1.)
        .with_offset(0.5);
# }
```

```cpp
const auto m = autd3::modulation::Sine(150)
                .with_amp(1.)
                .with_offset(0.5);
```

```cs
var m = new Sine(150)
        .WithAmp(1)
        .WithOffset(0.5);
```

```python
m = Sine(150).with_amp(1).with_offset(0.5)
```
