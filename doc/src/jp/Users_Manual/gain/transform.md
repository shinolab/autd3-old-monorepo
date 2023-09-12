# Transform

`Transform`は`Gain`になんらかの後処理を適用するための機能である.

```rust,edition2021
# extern crate autd3;
use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main() {
let g = Uniform::new(1.0).with_transform(|dev, tr: &LegacyTransducer , d| Drive {
    amp: d.amp - 0.5,
    phase: d.phase + PI,
});
# }
```

`with_transform`の引数は`Fn(&Device<T>, &T, &Drive) -> Drive`であり, 第1引数はデバイス, 第2引数は振動子, 第3引数は元の振幅/位相データである.
