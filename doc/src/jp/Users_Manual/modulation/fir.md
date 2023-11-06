# FIR

`FIR`は`Modulation`にFIRフィルタを適用するための機能である.

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
var m = new Sine(150).WithLowPass(199, 150.0);
```

```python
m = Sine(150).with_low_pass(199, 150.0)
```

`with_low_pass`の引数はタップ数とカットオフ周波数である.

low passの他に

- `with_high_pass`
- `with_band_pass`
- `with_band_stop`

が用意されている.
