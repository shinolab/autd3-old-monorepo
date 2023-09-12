# Group

`Group` is a `Gain` to use different `Gain` for each transducer.

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

In the above case, transducers whose local indices are less or equal than 100 produce `Null`, and the others produce `Focus`.
