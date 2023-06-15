# FIR

`FIR`は`Modulation`にFIRフィルタを適用するための機能である.

```rust
use autd3::prelude::*;
use autd3::modulation::FIR;

# #[allow(unused_variables)]
# fn main() {
let m = Sine::new(150).with_low_pass(199, 150.);
# }
```

`with_low_pass`の引数はタップ数とカットオフ周波数である.

low passの他に

- `with_high_pass`
- `with_band_pass`
- `with_band_stop`
- `with_resampler`

が用意されている.
