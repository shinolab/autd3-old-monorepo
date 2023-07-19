# Sine

音圧をSin波状に変形するための`Modulation`.

コンストラクタには周波数$f$を整数で指定する.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Sine::new(150);
# }
```

```cpp
const auto m = autd3::modulation::Sine(150);
```

```cs
var m = new Sine(150);
```

```python
from pyautd3.modulation import Sine

m = Sine(150)
```

## 振幅とオフセットの指定

`Sine`は音圧の波形が
$$
    \frac{amplitude}{2} \times \sin(2\pi ft) + offset
$$
となるようなAMをかける.
ここで, $amplitude$と$offset$はそれぞれ, `with_amp`と`with_offset`にて指定できる (デフォルトはそれぞれ$1$, $0.5$).

ただし, 上記で$\[0, 1\]$を超えるような値は$\[0, 1\]$に収まるように変換される.

```rust,edition2021
# extern crate autd3;
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Sine::new(150)
        .with_amp(1.)
        .with_offset(0.5);
# }
```

```cpp
const auto m = autd3::modulation::Sine(150)
                .with_amp(1.)
                .with_offset(0.5);
```

```cs
var m = new Sine(150)
        .WithAmp(1)
        .WithOffset(0.5);
```

```python
m = Sine(150).with_amp(1).with_offset(0.5)
```
