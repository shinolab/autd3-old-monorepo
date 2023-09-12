# Group

`Group`は振動子ごとに別々の`Gain`を使用するための`Gain`である.

`Group`では, デバイスと振動子に対してキーを割り当て, その各キーに`Gain`を紐付けて使用する.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let gain : autd3::gain::Group<_, LegacyTransducer, _, _> =
Group::new(|dev, tr: &LegacyTransducer| match tr.local_idx() {
                0..=100 => Some("null"),
                101.. => Some("focus"),
                _ => None,
            })
            .set("null", Null::new())
            .set("focus", Focus::new(Vector3::new(0.0, 0.0, 150.0)));
# }
```

上の場合は, ローカルインデックスが$0$から$100$の振動子は`Null`を, それ以外の振動子は`Focus`を出力する.
