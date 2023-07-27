# Fourier

複数の周波数の正弦波を重ね合わせた波形を生成する`Modulation`.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
use autd3::modulation::Fourier;

# #[allow(unused_variables)]
# fn main()  {
let m = Fourier::new()
        .add_component(Sine::new(100))
        .add_components_from_iter([Sine::new(150), Sine::new(200)]);
# }
```
