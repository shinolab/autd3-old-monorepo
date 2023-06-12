# Square

矩形波状の`Modulation`.

コンストラクタには周波数$f$を整数で指定する.

```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Square::new(150);
# }
```

```cpp
const auto m = autd3::modulation::Square(150);
```

```cs
var m = new Square(150);
```

```python
from pyautd3.modulation import Square

m = Square(150)
```

## 振幅の指定

Low/Highレベルの振幅はそれぞれ, `with_low`, `with_high`で指定できる.


```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Square::new(150)
        .with_low(0.)
        .with_high(1.);
# }
```

```cpp
const auto m = autd3::modulation::Square(150)
        .with_low(0.)
        .with_high(1.);
```

```cs
var m = new Square(150)
        .WithLow(0.)
        .WithHigh(1.);
```

```python
from pyautd3.modulation import Square

m = Square(150).with_low(0).with_high(1)
```

## Duty比の指定

`with_duty`で矩形波のDuty比を指定できる.

```rust
# use autd3::prelude::*;

# #[allow(unused_variables)]
# fn main()  {
let m = autd3::modulation::Square::new(150)
        .with_duty(0.5);
# }
```

```cpp
const auto m = autd3::modulation::Square(150)
        .with_duty(0.5);
```

```cs
var m = new Square(150)
        .WithDuty(0.5);
```

```python
from pyautd3.modulation import Square

m = Square(150).with_duty(0.5)
```