# Transform

`Transform`は`Modulation`になんらかの後処理を適用するための機能である.

```rust
use autd3::prelude::*;
use autd3::modulation::Transform;

# #[allow(unused_variables)]
# fn main() {
let m = Sine::new(150).with_transform(|i, d| (d - 0.5).abs() + 0.5);
# }
```

`with_transform`の引数は`Fn(usize, f64) -> f64`であり, 第1引数はインデックス, 第2引数は変調データである.
