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

```cpp
auto m = autd3::modulation::Fourier()
             .add_component(autd3::modulation::Sine(100))
             .add_components_from_iter(std::vector{autd3::modulation::Sine(150), autd3::modulation::Sine(200)});
```

```cs
var m = new Fourier()
        .AddComponent(new Sine(100))
        .AddComponentsFromIter(new []{new Sine(150), new Sine(200)});
```

`+`演算子も定義されている.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
use autd3::modulation::Fourier;

# #[allow(unused_variables)]
# fn main()  {
let m: Fourier = Sine::new(100) + Sine::new(150) + Sine::new(200);
# }
```

```cpp
const auto m = autd3::modulation::Sine(100) + autd3::modulation::Sine(150) + autd3::modulation::Sine(200);
```

```cs
var m = new Sine(100) + new Sine(150) + new Sine(200);
```

## 位相パラメータ

`Fourier`のために, `Sine`には位相パラメータを指定する機能がある.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;
use autd3::modulation::Fourier;

# #[allow(unused_variables)]
# fn main()  {
let m: Fourier = Sine::new(100) + Sine::new(150).with_phase(PI / 2.0);
# }
```

```cpp
const auto m = autd3::modulation::Sine(100) + autd3::modulation::Sine(150).with_phase(autd3::pi / 2.0);
```

```cs
var m = new Sine(100) + new Sine(150).WithPhase(AUTD3.Pi / 2.0);
```
