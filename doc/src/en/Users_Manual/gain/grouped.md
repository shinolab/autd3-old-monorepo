# Group

`Group` is a `Gain` to use different `Gain` for each transducer.

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
var g = new Group((dev, tr) => tr.LocalIdx <= 100 ? "null" : "focus")
          .Set("null", new Null())
          .Set("focus", new Focus(new Vector3d(x, y, z)));
```

```python
from pyautd3.gain import Focus, Null, Group

g = Group(lambda _, tr: "null" if tr.local_idx <= 100 else "focus").set("null", Null()).set("focus", Focus(np.array([x, y, z])))

```

In the above case, transducers whose local indices are less or equal than 100 produce `Null`, and the others produce `Focus`.

> NOTE:
> In this sample, we use string as a key, but you can use any type that can be used as a key of HashMap.
