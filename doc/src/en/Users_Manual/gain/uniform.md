# Uniform

`Uniform` set the same amplitude and phase to all transducers.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
let g = autd3::gain::Uniform::new(1.0);
# }
```

```cpp
const auto g = autd3::gain::Uniform(1.0);
```

```cs
var g = new Uniform(1.0);
```

```python
from pyautd3.gain import Uniform

g = Uniform(1.0)
```

Specify the amplitude in the constructor of `Uniform`.

## Set phase

`with_phase` method sets the phase.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
let g = autd3::gain::Uniform::new(1.0).with_phase(0.0);
# }
```

```cpp
const auto g = autd3::gain::Uniform(1.0).with_phase(0.0);
```

```cs
var g = new Uniform(1.0).WithPhase(0.0);
```

```python
from pyautd3.gain import Uniform

g = Uniform(1.0).with_phase(0.0)
```

The default phase is $0$.
