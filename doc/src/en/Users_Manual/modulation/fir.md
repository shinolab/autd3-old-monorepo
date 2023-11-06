# FIR

`FIR` is a feature to apply FIR filter to `Modulation`.

```rust,edition2021
# extern crate autd3;
# extern crate tokio;
use autd3::prelude::*;
use autd3::modulation::FIR;

# #[allow(unused_variables)]
# fn main() {
let m = Sine::new(150).with_low_pass(199, 150.);
# }
```

```cpp
auto m = autd3::modulation::Sine(150).with_low_pass(199, 150.);
```

```cs
var m = new Sine(150).WithLowPass();
```

```python
m = Sine(150).with_low_pass(199, 150.0)
```

The arguments of `with_low_pass` are the number of taps and the cutoff frequency.

There are also other filters:
- `with_high_pass`
- `with_band_pass`
- `with_band_stop`

