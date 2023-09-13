# Group

`Group`は振動子ごとに別々の`Gain`を使用するための`Gain`である.

`Group`では, デバイスと振動子に対してキーを割り当て, その各キーに`Gain`を紐付けて使用する.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
# #[allow(unused_variables)]
# fn main()  {
# let x = 0.;
# let y = 0.;
# let z = 0.;
# let gain : autd3::gain::Group<_, LegacyTransducer, _, _> =
Group::new(|dev, tr: &LegacyTransducer| match tr.local_idx() {
                0..=100 => Some("null"),
                101.. => Some("focus"),
                _ => None,
            })
            .set("null", Null::new())
            .set("focus", Focus::new(Vector3::new(x, y, z)));
# }
```

```cpp
const auto g = autd3::gain::Group([](const autd3::Device& dev, const autd3::Transducer& tr) -> std::optional<const char*> {
                 if (tr.local_idx() <= 100) return "null";
                 return "focus";
               })
                   .set("null", autd3::gain::Null())
                   .set("focus", autd3::gain::Focus(autd3::Vector3(x, y, z)));
```

```cs
var g = new Group<string>((dev, tr) => tr.LocalIdx <= 100 ? "null" : "focus")
          .Set("null", new Null())
          .Set("focus", new Focus(new Vector3d(x, y, z)));
```

```python
from pyautd3.gain import Focus, Null, Group

g = Group(lambda _, tr: "null" if tr.local_idx <= 100 else "focus").set("null", Null()).set("focus", Focus(np.array([x, y, z])))

```

上の場合は, ローカルインデックスが$0$から$100$の振動子は`Null`を, それ以外の振動子は`Focus`を出力する.
